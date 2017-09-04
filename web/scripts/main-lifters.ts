// vim: set ts=4 sts=4 sw=4 et:
'use strict';

import * as common from './common'
import { OplDBColumn, MeetDBColumn, db_make_indices_list, db_filter } from './database'

// Appease the TypeScript compiler.
declare let Slick;
declare let opldb;
declare let meetdb;

let grid; // The SlickGrid.
let sortCol = {id: 'date'}; // Initial column sorting information.
let sortAsc = false; // Initial column sorting information.
let lifterString = document.getElementById('lifter');

const selWeightType = document.getElementById("weighttype") as HTMLSelectElement;

// Fills in the <tbody> given the current query.
function getIndices(query: common.QueryObject): number[] {
    // No query: nothing to draw.
    if (query.q === undefined) {
        return [];
    }

    function filter(row) {
        return row[OplDBColumn.Name] === query.q;
    }

    let indices = db_make_indices_list();
    indices = db_filter(indices, filter);

    // Update the name display here: if the name matches something
    // in the database, we're safe from HTML injection.
    if (indices.length > 0) {
        // Pretty-print the name using makeRowObj().
        let rowobj = common.makeRowObj(opldb.data[indices[0]], 0);
        lifterString.innerHTML = 'Meet Results for ' + rowobj.name;
    } else {
        // Don't inject query.q here: may be HTML!
        lifterString.innerHTML = 'Lifter not found.'
    }

    let sortFn = common.getSortFn(sortCol.id, sortAsc);
    indices.sort(sortFn);
    return indices;
}


function makeDataProvider(query: common.QueryObject) {
    let indices = getIndices(query);

    return {
        getLength: function () {
            return indices.length;
        },
        getItem: function(index) {
            return common.makeRowObj(opldb.data[indices[index]], index);
        }
    };
}


function addSelectorListeners(selector) {
    selector.addEventListener("change", redraw);
    selector.addEventListener("keydown", function()
        {
            setTimeout(redraw, 0);
        }
    );
}


function addEventListeners() {
    addSelectorListeners(selWeightType);
}


function redraw() {
    common.setWeightTypeState(selWeightType.value);
    grid.invalidateAllRows();
    grid.render();
}


function onload() {
    addEventListeners();

    let query = common.getqueryobj();

    let rankWidth = 40;
    let nameWidth = 200;
    let shortWidth = 40;
    let dateWidth = 80;
    let numberWidth = 56;

    function urlformatter(row, cell, value, columnDef, dataContext) {
        return value;
    }

    let columns = [
        {id: "filler", width: 20, minWidth: 20, focusable: false,
                       selectable: false, resizable: false},
        {id: "place", name: "Place", field: "place", width: rankWidth},
        {id: "fed", name: "Fed", field: "fed", width: numberWidth,
                    sortable: true, defaultSortAsc: true},
        {id: "date", name: "Date", field: "date", width: dateWidth,
                     sortable: true, defaultSortAsc: false, formatter: urlformatter},
        {id: "location", name: "Location", field: "location", width:dateWidth},
        {id: "meetname", name: "Meet Name", field: "meetname", width: nameWidth,
                         formatter: urlformatter},
        {id: "division", name: "Division", field: "division"},
        {id: "sex", name: "Sex", field: "sex", width: shortWidth},
        {id: "age", name: "Age", field: "age", width: shortWidth},
        {id: "equip", name: "Equip", field: "equip", width: shortWidth},
        {id: "weightclass", name: "Class", field: "weightclass", width: numberWidth},
        {id: "bw", name: "Weight", field: "bw", width: numberWidth,
                   sortable: true, defaultSortAsc: true},
        {id: "squat", name: "Squat", field: "squat", width: numberWidth,
                      sortable: true, defaultSortAsc: false},
        {id: "bench", name: "Bench", field: "bench", width: numberWidth,
                      sortable: true, defaultSortAsc: false},
        {id: "deadlift", name: "Deadlift", field: "deadlift", width: numberWidth,
                         sortable: true, defaultSortAsc: false},
        {id: "total", name: "Total", field: "total", width: numberWidth,
                      sortable: true, defaultSortAsc: false},
        {id: "wilks", name: "Wilks", field: "wilks", width: numberWidth,
                      sortable: true, defaultSortAsc: false}
    ];

    let options = {
        enableColumnReorder: false,
        forceSyncScrolling: true,
        forceFitColumns: true,
        rowHeight: 23,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing"
    };

    let data = makeDataProvider(query);
    grid = new Slick.Grid("#theGrid", data, columns, options);
    grid.setSortColumn(sortCol.id, sortAsc);

    grid.onSort.subscribe(function(e, args) {
        sortCol = args.sortCol;
        sortAsc = args.sortAsc;
        redraw();
    });

    window.addEventListener("resize", function(e) { grid.resizeCanvas(); }, false);
}


document.addEventListener("DOMContentLoaded", onload);

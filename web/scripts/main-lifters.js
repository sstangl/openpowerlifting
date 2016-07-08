// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var grid; // The SlickGrid.
var sortCol = {id: 'date'}; // Initial column sorting information.
var sortAsc = false; // Initial column sorting information.
var lifterString = document.getElementById('lifter');

// TODO: Actually have a toggle for this.
var usingLbs = true;

// TODO: Share this with main-index.js. A bunch of functions can be shared, actually.
function weight(kg) {
    if (kg === undefined)
        return '';
    if (!usingLbs)
        return String(kg);
    return String(Math.round(kg * 2.20462262 * 100) / 100);
}

// FIXME: Move to common code.
function parseWeightClass(x) {
    if (x === undefined)
        return '';
    if (!usingLbs)
        return String(x);
    if (typeof x === 'number')
        return String(Math.round(common.kg2lbs(x)));
    return String(Math.round(common.kg2lbs(x.split('+')[0]))) + '+';
}


// Fills in the <tbody> given the current query.
function getIndices(query) {
    // No query: nothing to draw.
    if (query.q === undefined) {
        return [];
    }

    function filter(row) {
        return row[opldb.NAME] === query.q;
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);

    // Update the name display here: if the name matches something
    // in the database, we're safe from HTML injection.
    if (indices.length > 0) {
        lifterString.innerHTML = 'Meet Results for ' + query.q;
    } else {
        // Don't inject query.q here: may be HTML!
        lifterString.innerHTML = 'Lifter not found.'
    }

    var sortFn = common.getSortFn(sortCol.id, sortAsc);
    indices.sort(sortFn);
    return indices;
}


function makeDataProvider(query) {
    var indices = getIndices(query);

    return {
        getLength: function () {
            return indices.length;
        },
        getItem: function(index) {
            return common.makeRowObj(opldb.data[indices[index]], index);
        }
    };
}


function onload() {
    var query = common.getqueryobj();

    var rankWidth = 40;
    var nameWidth = 200;
    var shortWidth = 40;
    var dateWidth = 80;
    var numberWidth = 56;

    function urlformatter(row, cell, value, columnDef, dataContext) {
        return value;
    }

    var columns = [
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
                      sortable: true, defaultSortAsc: false},
        {id: "mcculloch", name: "McCulloch", field: "mcculloch", width: numberWidth+10,
                          sortable: true, defaultSortAsc: false}
    ];

    var options = {
        enableColumnReorder: false,
        forceSyncScrolling: true,
        forceFitColumns: true,
        rowHeight: 23,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing"
    };

    var data = makeDataProvider(query);
    grid = new Slick.Grid("#theGrid", data, columns, options);
    grid.setSortColumn(sortCol.id, sortAsc);

    function redraw() {
        var source = makeDataProvider(query);
        grid.setData(source);
        grid.invalidateAllRows();
        grid.render();
    }

    grid.onSort.subscribe(function(e, args) {
        sortCol = args.sortCol;
        sortAsc = args.sortAsc;
        redraw();
    });

    window.addEventListener("resize", function(e) { grid.resizeCanvas(); }, false);
}


document.addEventListener("DOMContentLoaded", onload);

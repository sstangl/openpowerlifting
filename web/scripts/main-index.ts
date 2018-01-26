// vim: set ts=4 sts=4 sw=4 et:
'use strict';

import * as common from './common'
import * as database from './database'
import { OplDBColumn, MeetDBColumn } from './database'

// Appease the TypeScript compiler.
declare var $;
declare var Slick;
declare const opldb;
declare const meetdb;

let grid; // The SlickGrid.
let sortCol = {id: 'wilks'}; // Initial column sorting information.
let sortAsc = false; // Initial column sorting information.
let searchInfo = {laststr: ''};

const boxRaw = document.getElementById("raw") as HTMLInputElement;
const boxWraps = document.getElementById("wraps") as HTMLInputElement;
const boxSingle = document.getElementById("single") as HTMLInputElement;
const boxMulti = document.getElementById("multi") as HTMLInputElement;
const boxMen = document.getElementById("men") as HTMLInputElement;
const boxWomen = document.getElementById("women") as HTMLInputElement;
const selWeightType = document.getElementById("weighttype") as HTMLSelectElement;
const selClass = document.getElementById("weightclass") as HTMLSelectElement;
const selFed = document.getElementById("fedselect") as HTMLSelectElement;
const selYear = document.getElementById("yearselect") as HTMLSelectElement;
const searchfield = document.getElementById("searchfield") as HTMLInputElement;
const searchbutton = document.getElementById("searchbutton") as HTMLButtonElement;


// Return the ordered list of rows to display, by index into opldb.data.
function getIndices(): number[] {
    // Determine the filter to be used.
    let raw = boxRaw.checked;
    let wraps = boxWraps.checked;
    let single = boxSingle.checked;
    let multi = boxMulti.checked;
    let men = boxMen.checked;
    let women = boxWomen.checked;

    let selectonfed = (selFed.value !== "all");
    let feds = selFed.value.split(',');

    let selectonclass = (selClass.value !== "all");
    let range = common.getWeightRange(selClass.value);
    let bw_min = +range[0]; // Exclusive.
    let bw_max = +range[1]; // Inclusive.

    let selectonyear = (selYear.value !== "all");
    let year = selYear.value;

    function filter(row): boolean {
        if (!men && !women)
            return false;
        if (!men && row[OplDBColumn.Sex] === 0)
            return false;
        if (!women && row[OplDBColumn.Sex] === 1)
            return false;

        if (selectonclass) {
            let bw = row[OplDBColumn.BodyweightKg];

            // "Undefined" bodyweights need to count for SHW.
            // This is basically just for Big Dogs.
            if (bw_max === 999.0) {
                if (bw !== undefined && (bw <= bw_min))
                    return false;
            } else {
                if (bw === undefined || bw <= bw_min || bw > bw_max)
                    return false;
            }
        }

        if (selectonfed || selectonyear) {
            let meetrow = meetdb.data[row[OplDBColumn.MeetID]];

            if (selectonfed) {
                let fed = meetrow[MeetDBColumn.Federation];
                if (feds.indexOf(fed) < 0) {
                    return false;
                }
            }

            if (selectonyear) {
                let date = meetrow[MeetDBColumn.Date];
                if (date.indexOf(year) < 0) {
                    return false;
                }
            }
        }

        let e = row[OplDBColumn.Equipment];
        return (raw && e === 0) ||
               (wraps && e === 1) ||
               (single && e === 2) ||
               (multi && e === 3);
    }

    let indices = database.db_make_indices_list();
    indices = database.db_filter(indices, filter);

    if (sortAsc)
        indices = database.db_sort_numeric_minfirst(indices, common.colidToIndex(sortCol.id));
    else
        indices = database.db_sort_numeric_maxfirst(indices, common.colidToIndex(sortCol.id));

    indices = database.db_uniq_lifter(indices);
    return indices;
}


function makeDataProvider() {
    let indices = getIndices();

    return {
        getLength: function(): number {
            return indices.length;
        },
        getItem: function(index: number) {
            return common.makeRowObj(opldb.data[indices[index]], index);
        }
    };
}

const LB_CLASSES = [
    // Traditional
    "-97", "-105", "-114", "-123", "-132", "-148", "-165", "-181", "-198", "198+", "-220", "-242", "-275", "-308", "308+",
    // IPF Men
    "-116", "-130", "-145", "-163", "-183", "-205", "-231", "-264", "264+",
    // IPF Women
    "-94", "-103", "-114", "-125", "-138", "-158", "-185", "185+"
]

const KG_CLASSES = [
    // Traditional
    "-44", "-48", "-52", "-56", "-60", "-67.5", "-75", "-82.5", "-90", "90+", "-100", "-110", "-125", "-140", "140+",
    // IPF Men
    "-53", "-59", "-66", "-74", "-83", "-93", "-105", "-120", "120+",
    // IPF Women
    "-43", "-47", "-52", "-57", "-63", "-72", "-84", "84+"
]

function generateWeightClasses() {
    let weightclasses = (selWeightType.value === "lb" ? LB_CLASSES : KG_CLASSES);

    // Offset iteration at i = 1 to skip over the "all" option.
    for (let i = 1; i < selClass.options.length; ++i) {
        selClass.options[i].text = weightclasses[i-1];
    }
}


function redraw() {
    common.setWeightTypeState(selWeightType.value)
    generateWeightClasses();

    let source = makeDataProvider();
    grid.setData(source);
    grid.invalidateAllRows();
    grid.render();
}


function _search_from(query: string, rowid: number): number {
    let data = grid.getData();
    let numrows = data.getLength();

    for (let i = rowid; i < numrows; ++i) {
        let row = data.getItem(i);
        if (row.searchname.indexOf(query) >= 0) {
            return i;
        }
    }
    return -1;
}


function search() {
    let query = searchfield.value.toLowerCase().trim().replace("  "," ");
    if (!query) {
        return;
    }

    let startrowid = 0;
    // If the search string hasn't changed, do a "next"-style search.
    if (query === searchInfo.laststr) {
        startrowid = grid.getViewport().top + 1;
    }

    let rowid = _search_from(query, startrowid);

    // If nothing was found in "next" mode, try searching again from the top.
    if (startrowid > 0 && rowid === -1) {
        rowid = _search_from(query, 0);
    }

    if (rowid >= 0) {
        let numColumns = grid.getColumns().length;

        grid.scrollRowToTop(rowid);
        for (let i = 0; i < numColumns; ++i) {
            grid.flashCell(rowid, i, 100);
        }

        searchInfo.laststr = query;
        searchbutton.innerHTML = "Next";
    }
}


function onResize(evt) {
    grid.resizeCanvas();
}


function searchOnEnter(keyevent) {
    // keyCode is deprecated, but non-Firefox-desktop doesn't support key.
    if (keyevent.keyCode === 13 || keyevent.key === "Enter") {
        search();
    }
}

function scrollOnPageUpDown(keyevent) {
    if (keyevent.keyCode === 33 || keyevent.key === "page up") {
        grid.scrollRowToTop(grid.getViewport().top - 5);
    } else if (keyevent.keyCode === 34 || keyevent.key === "page down") {
        grid.scrollRowToTop(grid.getViewport().top + 5);
    }
}

function addSelectorListeners(selector: HTMLSelectElement) {
    selector.addEventListener("change", redraw);
    selector.addEventListener("keydown", function()
        {
            setTimeout(redraw, 0);
        }
    );
}

function addEventListeners() {
    boxRaw.addEventListener("click", redraw);
    boxWraps.addEventListener("click", redraw);
    boxSingle.addEventListener("click", redraw);
    boxMulti.addEventListener("click", redraw);
    boxMen.addEventListener("click", redraw);
    boxWomen.addEventListener("click", redraw);

    addSelectorListeners(selWeightType);
    addSelectorListeners(selClass);
    addSelectorListeners(selFed);
    addSelectorListeners(selYear);

    searchfield.addEventListener("keypress", searchOnEnter, false);
    searchbutton.addEventListener("click", search, false);

    $("#searchfield").on("input", function () {
        searchbutton.innerHTML = "Search";
    });

    $(window).on("keydown", scrollOnPageUpDown);

    window.addEventListener("resize", onResize, false);
}


function onload() {
    addEventListeners();
    generateWeightClasses();

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
        {id: "rank", name: "Rank", field: "rank", width: numberWidth},
        {id: "name", name: "Name", field: "name", width: nameWidth, formatter: urlformatter},
        {id: "fed", name: "Fed", field: "fed", width: numberWidth},
        {id: "date", name: "Date", field: "date", width: dateWidth, formatter: urlformatter},
        {id: "location", name: "Location", field: "location", width:dateWidth},
        {id: "sex", name: "Sex", field: "sex", width: shortWidth},
        {id: "age", name: "Age", field: "age", width: shortWidth,
                    sortable: true, defaultSortAsc: false},
        {id: "equip", name: "Equip", field: "equip", width: shortWidth},
        {id: "weightclass", name: "Class", field: "weightclass", width: numberWidth},
        {id: "bw", name: "Weight", field: "bw", width: numberWidth,
                   sortable: true, defaultSortAsc: false},
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

    let query = common.getqueryobj();
    if (query.fed !== undefined) {
        selFed.value = query.fed;
    }

    let data = makeDataProvider();
    grid = new Slick.Grid("#theGrid", data, columns, options);
    grid.setSortColumn(sortCol.id, sortAsc);

    grid.onSort.subscribe(function(e, args) {
        sortCol = args.sortCol;
        sortAsc = args.sortAsc;
        redraw();
    });

    search();
}


document.addEventListener("DOMContentLoaded", onload);

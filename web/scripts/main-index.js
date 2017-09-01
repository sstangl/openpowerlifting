// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var grid; // The SlickGrid.
var sortCol = {id: 'wilks'}; // Initial column sorting information.
var sortAsc = false; // Initial column sorting information.
var searchInfo = {laststr: ''};

var boxRaw = document.getElementById("raw");
var boxWraps = document.getElementById("wraps");
var boxSingle = document.getElementById("single");
var boxMulti = document.getElementById("multi");
var boxMen = document.getElementById("men");
var boxWomen = document.getElementById("women");
var selWeightType = document.getElementById("weighttype");
var selClass = document.getElementById("weightclass");
var selFed = document.getElementById("fedselect");
var selYear = document.getElementById("yearselect");
var searchfield = document.getElementById("searchfield");
var searchbutton = document.getElementById("searchbutton");

// FIXME: Move to common code.
function weight(kg) {
    if (kg === undefined)
        return '';
    if (selWeightType.value === "kg")
        return String(kg);
    return String(common.kg2lbs(kg));
}

// FIXME: Move to common code.
function parseWeightClass(x) {
    if (x === undefined)
        return '';
    if (selWeightType.value === "kg")
        return String(x);
    if (typeof x === 'number')
        return String(Math.floor(common.kg2lbs(x)));
    return String(Math.floor(common.kg2lbs(x.split('+')[0]))) + '+';
}

// Return the ordered list of rows to display, by index into opldb.data.
function getIndices() {
    // Update the global pounds setting.

    // Determine the filter to be used.
    var raw = boxRaw.checked;
    var wraps = boxWraps.checked;
    var single = boxSingle.checked;
    var multi = boxMulti.checked;
    var men = boxMen.checked;
    var women = boxWomen.checked;

    var selectonfed = (selFed.value !== "all");
    var feds = selFed.value.split(',');

    var selectonclass = (selClass.value !== "all");
    var range = common.getWeightRange(selClass.value);
    var bw_min = +range[0]; // Exclusive.
    var bw_max = +range[1]; // Inclusive.

    var selectonyear = (selYear.value !== "all");
    var year = selYear.value;

    function filter(row) {
        if (!men && !women)
            return false;
        if (!men && row[opldb.SEX] === 0)
            return false;
        if (!women && row[opldb.SEX] === 1)
            return false;

        if (selectonclass) {
            var bw = row[opldb.BODYWEIGHTKG];
            if (bw === undefined || bw <= bw_min || bw > bw_max)
                return false;
        }

        if (selectonfed || selectonyear) {
            var meetrow = meetdb.data[row[opldb.MEETID]];

            if (selectonfed) {
                var fed = meetrow[meetdb.FEDERATION];
                if (feds.indexOf(fed) < 0) {
                    return false;
                }
            }

            if (selectonyear) {
                var date = meetrow[meetdb.DATE];
                if (date.indexOf(year) < 0) {
                    return false;
                }
            }
        }

        var e = row[opldb.EQUIPMENT];
        return (raw && e === 0) ||
               (wraps && e === 1) ||
               (single && e === 2) ||
               (multi && e === 3);
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);

    if (sortAsc)
        indices = db_sort_numeric_minfirst(indices, common.colidToIndex(sortCol.id));
    else
        indices = db_sort_numeric_maxfirst(indices, common.colidToIndex(sortCol.id));

    indices = db_uniq_lifter(indices);
    return indices;
}


function makeDataProvider() {
    var indices = getIndices();

    return {
        getLength: function () {
            return indices.length;
        },
        getItem: function(index) {
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
    var weightclasses = (selWeightType.value === "lb" ? LB_CLASSES : KG_CLASSES);

    // Offset iteration at i = 1 to skip over the "all" option.
    for (var i = 1; i < selClass.options.length; ++i) {
        selClass.options[i].text = weightclasses[i-1];
    }
}


function redraw() {
    generateWeightClasses();

    var source = makeDataProvider();
    grid.setData(source);
    grid.invalidateAllRows();
    grid.render();
}


function _search_from(query, rowid) {
    var data = grid.getData();
    var numrows = data.getLength();

    for (var i = rowid; i < numrows; ++i) {
        var row = data.getItem(i);
        if (row.searchname.indexOf(query) >= 0) {
            return i;
        }
    }
    return -1;
}


function search() {
    var query = searchfield.value.toLowerCase().trim().replace("  "," ");
    if (!query) {
        return;
    }

    var startrowid = 0;
    // If the search string hasn't changed, do a "next"-style search.
    if (query === searchInfo.laststr) {
        startrowid = grid.getViewport().top + 1;
    }

    var rowid = _search_from(query, startrowid);

    // If nothing was found in "next" mode, try searching again from the top.
    if (startrowid > 0 && rowid === -1) {
        rowid = _search_from(query, 0);
    }

    if (rowid >= 0) {
        var numColumns = grid.getColumns().length;

        grid.scrollRowToTop(rowid);
        for (var i = 0; i < numColumns; ++i) {
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

function addSelectorListeners(selector) {
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

    var options = {
        enableColumnReorder: false,
        forceSyncScrolling: true,
        forceFitColumns: true,
        rowHeight: 23,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing"
    };

    var query = common.getqueryobj();
    if (query.fed !== undefined) {
        selFed.value = query.fed;
    }

    var data = makeDataProvider();
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

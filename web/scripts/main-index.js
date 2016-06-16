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
var searchfield = document.getElementById("searchfield");
var searchbutton = document.getElementById("searchbutton");

function weight(kg) {
    if (kg === undefined)
        return '';
    if (selWeightType.value === "kg")
        return String(kg);
    return String(common.kg2lbs(kg));
}

function parseWeightClass(x) {
    if (x === undefined)
        return '';
    if (selWeightType.value === "kg")
        return String(x);
    if (typeof x === 'number')
        return String(Math.round(common.kg2lbs(x)));
    return String(Math.round(common.kg2lbs(x.split('+')[0]))) + '+';
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
    var fed = selFed.value;

    var selectonclass = (selClass.value !== "all");
    var bw_min = 0.0; // Exclusive
    var bw_max = 999.0;

    if (selectonclass) {
        if (selClass.value === "-44") {
            bw_min = 0.0;
            bw_max = 44.0;
        } else if (selClass.value === "-48") {
            bw_min = 44.0;
            bw_max = 48.0;
        } else if (selClass.value === "-52") {
            bw_min = 48.0;
            bw_max = 52.0;
        } else if (selClass.value === "-56") {
            bw_min = 52.0;
            bw_max = 56.0;
        } else if (selClass.value === "-60") {
            bw_min = 56.0;
            bw_max = 60.0;
        } else if (selClass.value === "-67.5") {
            bw_min = 60.0;
            bw_max = 67.5;
        } else if (selClass.value === "-75") {
            bw_min = 67.5;
            bw_max = 75.0;
        } else if (selClass.value === "-82.5") {
            bw_min = 75.0;
            bw_max = 82.5;
        } else if (selClass.value === "-90") {
            bw_min = 82.5;
            bw_max = 90.0;
        } else if (selClass.value === "90+") {
            bw_min = 90.0;
            bw_max = 999.0;
        } else if (selClass.value === "-100") {
            bw_min = 90.0;
            bw_max = 100.0;
        } else if (selClass.value === "-110") {
            bw_min = 100.0;
            bw_max = 110.0;
        } else if (selClass.value === "-125") {
            bw_min = 110.0;
            bw_max = 125.0;
        } else if (selClass.value === "-140") {
            bw_min = 125.0;
            bw_max = 140.0;
        } else if (selClass.value === "140+") {
            bw_min = 140.0;
            bw_max = 999.0;
        } else {
            console.log("Unknown class: " + selClass.value);
            selectonclass = false;
        }
    }

    function filter(row) {
        if (!men && !women)
            return false;
        if (!men && row[opldb.SEX] == 'M')
            return false;
        if (!women && row[opldb.SEX] == 'F')
            return false;

        if (selectonclass) {
            var bw = row[opldb.BODYWEIGHTKG];
            if (bw === undefined || bw <= bw_min || bw > bw_max)
                return false;
        }

        if (selectonfed) {
            var meetrow = meetdb.data[row[opldb.MEETID]];
            if (meetrow[meetdb.FEDERATION] !== fed) {
                return false;
            }
        }

        var e = row[opldb.EQUIPMENT];
        return (raw && e == "Raw") ||
               (wraps && e == "Wraps") ||
               (single && e == "Single-ply") ||
               (multi && e == "Multi-ply");
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


function weightMax(row, cola, colb) {
    var a = row[cola];
    var b = row[colb];
    if (a === undefined)
        return weight(b);
    if (b === undefined)
        return weight(a);
    return weight(Math.max(a,b));
}


function makeItem(row, index) {
    var meetrow = meetdb.data[row[opldb.MEETID]];
    var name = row[opldb.NAME];

    var country = common.string(meetrow[meetdb.MEETCOUNTRY]);
    var state = common.string(meetrow[meetdb.MEETSTATE]);

    var location = country;
    if (country && state) {
        location = location + "-" + state;
    }

    return {
        rank:        index+1,
        searchname:  name.toLowerCase(),
        name:        '<a href="lifters.html?q='+name+'">'+name+'</a>',
        fed:         common.string(meetrow[meetdb.FEDERATION]),
        date:        common.string(meetrow[meetdb.DATE]),
        location:    location,
        sex:         common.string(row[opldb.SEX]),
        age:         common.string(row[opldb.AGE]),
        equip:       common.parseEquipment(row[opldb.EQUIPMENT]),
        bw:          weight(row[opldb.BODYWEIGHTKG]),
        weightclass: parseWeightClass(row[opldb.WEIGHTCLASSKG]),
        squat:       weightMax(row, opldb.BESTSQUATKG, opldb.SQUAT4KG),
        bench:       weightMax(row, opldb.BESTBENCHKG, opldb.BENCH4KG),
        deadlift:    weightMax(row, opldb.BESTDEADLIFTKG, opldb.DEADLIFT4KG),
        total:       weight(row[opldb.TOTALKG]),
        wilks:       common.number(row[opldb.WILKS]),
        mcculloch:   common.number(row[opldb.MCCULLOCH])
    };
}


function makeDataProvider() {
    var indices = getIndices();

    return {
        getLength: function () { return indices.length; },
        getItem: function(index) { return makeItem(opldb.data[indices[index]], index); }
    };
}


function redraw() {
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
    if (startrowid > 0 && rowid == -1) {
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
        {id: "date", name: "Date", field: "date", width: dateWidth},
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

    // From a post on StackOverflow.
    function numberWithCommas(x) {
        return x.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
    }

    // A very simple, pretty ugly count of number of database rows.
    // Hopefully this is a simple way for people to see that the
    // site is changing.
    var numentries = document.getElementById("numentries");
    numentries.innerText = "(" + numberWithCommas(opldb.data.length) + " Entries)";

    search();
}


document.addEventListener("DOMContentLoaded", onload);

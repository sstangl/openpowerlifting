// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var grid; // The SlickGrid.
var theTable = document.getElementById("thetable");
var boxRaw = document.getElementById("raw");
var boxWraps = document.getElementById("wraps");
var boxSingle = document.getElementById("single");
var boxMulti = document.getElementById("multi");
var boxMen = document.getElementById("men");
var boxWomen = document.getElementById("women");
var selWeightType = document.getElementById("weighttype");
var selClass = document.getElementById("class");

// Toggle between pounds or kilograms, used by weight().
var usingLbs = true;

// The column on which to sort.
var sortByGlobal = opldb.WILKS;


function weight(kg) {
    if (kg === undefined)
        return '';
    if (!usingLbs)
        return String(kg);
    return String(Math.round(kg * 2.2042262 * 100) / 100);
}

function number(num) {
    if (num === undefined)
        return '';
    return String(num);
}

function string(str) {
    if (str === undefined)
        return '';
    return str;
}


function maketd(str) {
    var td = document.createElement('td');
    td.appendChild(document.createTextNode(str));
    return td;
}


// Make the HTML for a single database row.
function makeentry(row, i) {
    var tr = document.createElement('tr');
    tr.appendChild(maketd(String(i+1)));

    // The name should link to the lifter page.
    var name = string(row[opldb.NAME]);
    var lifterlink = document.createElement('a');
    lifterlink.setAttribute('href', 'lifters.html?q=' + name);
    lifterlink.setAttribute('class', 'datalink');
    lifterlink.appendChild(document.createTextNode(name));

    var liftertd = document.createElement('td');
    liftertd.appendChild(lifterlink);
    tr.appendChild(liftertd);

    var meetrow = meetdb.data[row[opldb.MEETID]];
    tr.appendChild(maketd(string(meetrow[meetdb.FEDERATION])));
    tr.appendChild(maketd(string(meetrow[meetdb.DATE])));

    tr.appendChild(maketd(string(row[opldb.SEX])));
    tr.appendChild(maketd(number(row[opldb.AGE])));

    var equipment = row[opldb.EQUIPMENT];
    if (equipment === 'Raw') {
        tr.appendChild(maketd('R'));
    } else if (equipment === 'Wraps') {
        tr.appendChild(maketd('W'));
    } else if (equipment === 'Single-ply') {
        tr.appendChild(maketd('S'));
    } else if (equipment === 'Multi-ply') {
        tr.appendChild(maketd('M'));
    } else {
        tr.appendChild(maketd(''));
    }

    tr.appendChild(maketd(weight(row[opldb.BODYWEIGHTKG])));
    tr.appendChild(maketd(weight(row[opldb.BESTSQUATKG])));
    tr.appendChild(maketd(weight(row[opldb.BESTBENCHKG])));
    tr.appendChild(maketd(weight(row[opldb.BESTDEADLIFTKG])));
    tr.appendChild(maketd(weight(row[opldb.TOTALKG])));
    tr.appendChild(maketd(number(row[opldb.WILKS])));
    tr.appendChild(maketd(number(row[opldb.MCCULLOCH])));
    return tr;
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

        var e = row[opldb.EQUIPMENT];
        return (raw && e == "Raw") ||
               (wraps && e == "Wraps") ||
               (single && e == "Single-ply") ||
               (multi && e == "Multi-ply");
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);
    indices = db_sort_numeric_maxfirst(indices, sortByGlobal);
    indices = db_uniq_lifter(indices);
    return indices;
}


function parseEquipment(str) {
    if (str === "Raw")
        return "R";
    if (str === "Wraps")
        return "W";
    if (str === "Single-ply")
        return "S";
    if (str === "Multi-ply")
        return "M";
    return "";
}


function makeItem(row, index) {
    var meetrow = meetdb.data[row[opldb.MEETID]];
    var name = row[opldb.NAME];

    return {
        rank: index+1,
        name: '<a href="lifters.html?q='+name+'">'+name+'</a>',
        fed: string(meetrow[meetdb.FEDERATION]),
        date: string(meetrow[meetdb.DATE]),
        sex: string(row[opldb.SEX]),
        age: string(row[opldb.AGE]),
        equip: parseEquipment(row[opldb.EQUIPMENT]),
        bw: number(row[opldb.BODYWEIGHTKG]),
        squat: weight(row[opldb.BESTSQUATKG]),
        bench: weight(row[opldb.BESTBENCHKG]),
        deadlift: weight(row[opldb.BESTDEADLIFTKG]),
        total: weight(row[opldb.TOTALKG]),
        wilks: number(row[opldb.WILKS]),
        mcculloch: number(row[opldb.MCCULLOCH]),
    };
}


function makeDataProvider() {
    var indices = getIndices();

    return {
        getLength: function () { return indices.length; },
        getItem: function(index) { return makeItem(opldb.data[indices[index]], index); }
    }
}


function redraw() {
    var source = makeDataProvider();
    grid.setData(source);
    grid.invalidateAllRows();
    grid.render();
}


function addEventListeners() {
    boxRaw.addEventListener("click", redraw);
    boxWraps.addEventListener("click", redraw);
    boxSingle.addEventListener("click", redraw);
    boxMulti.addEventListener("click", redraw);
    boxMen.addEventListener("click", redraw);
    boxWomen.addEventListener("click", redraw);

    selWeightType.addEventListener("change", redraw);
    selWeightType.addEventListener("keydown", function()
        {
            setTimeout(redraw, 0);
        }
    );

    selClass.addEventListener("change", redraw);
    selClass.addEventListener("keydown", function()
        {
            setTimeout(redraw, 0);
        }
    );

    var sortables = document.getElementsByClassName("sortable");
    for (var i = 0; i < sortables.length; ++i) {
        sortables[i].addEventListener("click", function(e)
            {
                if (e.target.id == "sort-bw")
                    sortByGlobal = opldb.BODYWEIGHTKG;
                else if (e.target.id == "sort-squat")
                    sortByGlobal = opldb.BESTSQUATKG;
                else if (e.target.id == "sort-bench")
                    sortByGlobal = opldb.BESTBENCHKG;
                else if (e.target.id == "sort-deadlift")
                    sortByGlobal = opldb.BESTDEADLIFTKG;
                else if (e.target.id == "sort-total")
                    sortByGlobal = opldb.TOTALKG;
                else if (e.target.id == "sort-wilks")
                    sortByGlobal = opldb.WILKS;
                else if (e.target.id == "sort-mcculloch")
                    sortByGlobal = opldb.MCCULLOCH;
                redraw();
            }
        );
    }
}


function onload() {
    addEventListeners();

    var rankWidth = 60;
    var nameWidth = 240;
    var stringWidth = 60;
    var dateWidth = 100;
    var numberWidth = 80;

    function urlformatter(row, cell, value, columnDef, dataContext) {
        return value;
    }

    var columns = [
        {id: "rank", name: "Rank", field: "rank", width: rankWidth},
        {id: "name", name: "Name", field: "name", width: nameWidth, formatter: urlformatter},
        {id: "fed", name: "Fed", field: "fed", width: stringWidth},
        {id: "date", name: "Date", field: "date", width: dateWidth},
        {id: "sex", name: "Sex", field: "sex", width: stringWidth},
        {id: "age", name: "Age", field: "age", width: stringWidth},
        {id: "equip", name: "Equip", field: "equip", width: stringWidth},
        {id: "bw", name: "Weight", field: "bw", width: numberWidth},
        {id: "squat", name: "Squat", field: "squat", width: numberWidth},
        {id: "bench", name: "Bench", field: "bench", width: numberWidth},
        {id: "deadlift", name: "Deadlift", field: "deadlift", width: numberWidth},
        {id: "total", name: "Total", field: "total", width: numberWidth},
        {id: "wilks", name: "Wilks", field: "wilks", width: numberWidth},
        {id: "mcculloch", name: "McCulloch", field: "mcculloch", width: numberWidth},
    ];

    var options = {
        enableColumnReorder: false,
        forceFitColumns: true,
        forceSyncScrolling: true,
    };

    var data = makeDataProvider();
    grid = new Slick.Grid("#theGrid", data, columns, options);
};


document.addEventListener("DOMContentLoaded", onload);

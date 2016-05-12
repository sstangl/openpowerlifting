// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var results = document.getElementById("results");
var boxRaw = document.getElementById("raw");
var boxWraps = document.getElementById("wraps");
var boxSingle = document.getElementById("single");
var boxMulti = document.getElementById("multi");
var boxMen = document.getElementById("men");
var boxWomen = document.getElementById("women");
var boxAllResults = document.getElementById("showall");
var btnShowMore = document.getElementById("showmore");
var selWeightType = document.getElementById("weighttype");
var selClass = document.getElementById("class");

// Toggle between pounds or kilograms, used by weight().
var usingLbs = true;

// The column on which to sort.
var sortByGlobal = WILKS;


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
    tr.appendChild(maketd(string(row[NAME])));
    tr.appendChild(maketd(string(row[SEX])));
    tr.appendChild(maketd(number(row[AGE])));
    tr.appendChild(maketd(weight(row[BODYWEIGHTKG])));
    tr.appendChild(maketd(weight(row[BESTSQUATKG])));
    tr.appendChild(maketd(weight(row[BESTBENCHKG])));
    tr.appendChild(maketd(weight(row[BESTDEADLIFTKG])));
    tr.appendChild(maketd(weight(row[TOTALKG])));
    tr.appendChild(maketd(number(row[WILKS])));
    tr.appendChild(maketd(number(row[MCCULLOCH])));
    return tr;
}


// Fills in the <tbody> given the current selection state.
function redraw() {
    // Remove existing children.
    while (results.lastChild) {
        results.removeChild(results.lastChild);
    }

    // Update the global pounds setting.
    // TODO: This should be carried in a local variable to avoid poking the global.
    if (selWeightType.value == "lb")
        usingLbs = true;
    else
        usingLbs = false;

    // Determine the filter to be used.
    var raw = boxRaw.checked;
    var wraps = boxWraps.checked;
    var single = boxSingle.checked;
    var multi = boxMulti.checked;
    var men = boxMen.checked;
    var women = boxWomen.checked;
    var allresults = boxAllResults.checked;

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
        if (!men && row[SEX] == 'M')
            return false;
        if (!women && row[SEX] == 'F')
            return false;

        if (selectonclass) {
            var bw = row[BODYWEIGHTKG];
            if (bw <= bw_min || bw > bw_max)
                return false;
        }

        var e = row[EQUIPMENT];
        return (raw && e == "Raw") ||
               (wraps && e == "Wraps") ||
               (single && e == "Single-ply") ||
               (multi && e == "Multi-ply");
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);
    indices = db_sort_numeric_maxfirst(indices, sortByGlobal);
    indices = db_uniq_lifter(indices);

    var ntoshow = indices.length;
    if (allresults === false) {
        ntoshow = Math.min(500, indices.length);
        var left = indices.length - ntoshow;
        if (left > 500) {
            btnShowMore.style.visibility = "";
            btnShowMore.innerText = String(indices.length - ntoshow) + " more... (Slow)"
        } else {
            btnShowMore.style.visibility = "hidden";
        }
    }

    var frag = document.createDocumentFragment();
    for (var i = 0; i < ntoshow; i++) {
        var row = opldb[indices[i]];
        frag.appendChild(makeentry(row, i));
    }

    results.appendChild(frag);
}


function addEventListeners() {
    boxRaw.addEventListener("click", redraw);
    boxWraps.addEventListener("click", redraw);
    boxSingle.addEventListener("click", redraw);
    boxMulti.addEventListener("click", redraw);
    boxMen.addEventListener("click", redraw);
    boxWomen.addEventListener("click", redraw);

    boxAllResults.addEventListener("click", function()
        {
            if (boxAllResults.checked) {
                btnShowMore.style.visibility = "hidden";
            } else {
                btnShowMore.style.visibility = "";
            }
            redraw();
        }
    );

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
                    sortByGlobal = BODYWEIGHTKG;
                else if (e.target.id == "sort-squat")
                    sortByGlobal = BESTSQUATKG;
                else if (e.target.id == "sort-bench")
                    sortByGlobal = BESTBENCHKG;
                else if (e.target.id == "sort-deadlift")
                    sortByGlobal = BESTDEADLIFTKG;
                else if (e.target.id == "sort-total")
                    sortByGlobal = TOTALKG;
                else if (e.target.id == "sort-wilks")
                    sortByGlobal = WILKS;
                else if (e.target.id == "sort-mcculloch")
                    sortByGlobal = MCCULLOCH;
                redraw();
            }
        );
    }

    btnShowMore.addEventListener("click", function ()
        {
            boxAllResults.checked = true;
            btnShowMore.style.visibility = "hidden";
            redraw();
        }
    );
}


function onload() {
    addEventListeners();
    redraw();
};


document.addEventListener("DOMContentLoaded", onload);

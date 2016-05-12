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

// Toggle between pounds or kilograms.
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

    // Determine the filter to be used.
    var raw = boxRaw.checked;
    var wraps = boxWraps.checked;
    var single = boxSingle.checked;
    var multi = boxMulti.checked;
    var men = boxMen.checked;
    var women = boxWomen.checked;
    var allresults = boxAllResults.checked;

    function filter(row) {
        if (!men && !women)
            return false;
        if (!men && row[SEX] == 'M')
            return false;
        if (!women && row[SEX] == 'W')
            return false;

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
        ntoshow = 500;
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

    boxAllResults.addEventListener("click", function (e)
        {
            if (boxAllResults.checked) {
                btnShowMore.style.visibility = "hidden";
            } else {
                btnShowMore.style.visibility = "";
            }
            redraw();
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

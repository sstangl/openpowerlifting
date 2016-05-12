// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var results = document.getElementById("results");
var boxRaw = document.getElementById("raw");
var boxWraps = document.getElementById("wraps");
var boxSingle = document.getElementById("single");
var boxMulti = document.getElementById("multi");

var usingLbs = true;


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

    function filter(row) {
        var e = row[EQUIPMENT];
        return (raw && e == "Raw") ||
               (wraps && e == "Wraps") ||
               (single && e == "Single-ply") ||
               (multi && e == "Multi-ply");
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);
    indices = db_sort_numeric_maxfirst(indices, WILKS);
    indices = db_uniq_lifter(indices);

    var frag = document.createDocumentFragment();
    for (var i = 0; i < indices.length; i++) {
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
}


function onload() {
    addEventListeners();
    redraw();
};


document.addEventListener("DOMContentLoaded", onload);

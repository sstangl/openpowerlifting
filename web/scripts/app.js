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


// Make the HTML for a single database row.
function makeentry(row, i) {
    var str = "<tr>";

    str = str + "<td>" + String(i+1) + "</td>";
    str = str + "<td>" + string(row[NAME]) + "</td>";
    str = str + "<td>" + string(row[SEX]) + "</td>";
    str = str + "<td>" + number(row[AGE]) + "</td>";
    str = str + "<td>" + weight(row[BODYWEIGHTKG]) + "</td>";
    str = str + "<td>" + weight(row[BESTSQUATKG]) + "</td>";
    str = str + "<td>" + weight(row[BESTBENCHKG]) + "</td>";
    str = str + "<td>" + weight(row[BESTDEADLIFTKG]) + "</td>";
    str = str + "<td>" + weight(row[TOTALKG]) + "</td>";
    str = str + "<td>" + number(row[WILKS]) + "</td>";
    str = str + "<td>" + number(row[MCCULLOCH]) + "</td>";

    return str + "</tr>";
}


// Fills in the <tbody> given the current selection state.
function redraw() {
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

    var html = '';
    for (var i = 0; i < indices.length; i++) {
        var row = opldb[indices[i]];
        html += makeentry(row, i);
    }
    results.innerHTML = html;
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

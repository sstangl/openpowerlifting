// vim: set ts=4 sts=4 sw=4 et:
'use strict';

// TODO: Actually have a toggle for this.
var usingLbs = true;

// TODO: Share this with app.js. A bunch of functions can be shared, actually.
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


function makeentry(row) {
    var tr = document.createElement('tr');
    var meetrow = meetdb.data[row[opldb.MEETID]];

    tr.appendChild(maketd(string(row[opldb.PLACE])));
    tr.appendChild(maketd(string(row[opldb.NAME])));
    tr.appendChild(maketd(string(meetrow[meetdb.FEDERATION])));
    tr.appendChild(maketd(string(meetrow[meetdb.DATE])));
    tr.appendChild(maketd(string(row[opldb.SEX])));
    tr.appendChild(maketd(string(row[opldb.AGE])));

    var equipment = row[opldb.EQUIPMENT];
    if (equipment === 'Raw') {
        tr.appendChild(maketd('R'));
    } else if (equipment === 'Wraps') {
        tr.appendChild(maketd('W'));
    } else if (equipment === 'Single-ply') {
        tr.appendChild(maketd('S'));
    } else if (equipment === 'Multi-ply') {
        tr.appendChild(maketd('M'));
    } else if (equipment === 'Oldschool') {
        tr.appendChild(maketd('OS'));
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


// Fills in the <tbody> given the current query.
function redraw(query) {
    var results = document.getElementById("results");

    // Remove existing children.
    while (results.lastChild) {
        results.removeChild(results.lastChild);
    }

    // No query: nothing to draw.
    if (query.q === undefined) {
        return;
    }

    function filter(row) {
        return row[opldb.NAME] === query.q;
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);

    // Sort by meet date, most recent first.
    indices.sort(function(a, b) {
        var ameetid = opldb.data[a][opldb.MEETID];
        var bmeetid = opldb.data[b][opldb.MEETID];
        var adate = meetdb.data[ameetid][meetdb.DATE];
        var bdate = meetdb.data[bmeetid][meetdb.DATE];
        return adate < bdate;
    });


    var frag = document.createDocumentFragment();
    for (let index of indices) {
        var row = opldb.data[index];
        frag.appendChild(makeentry(row));
    }
    results.appendChild(frag);
}


// Roughly parse lifter.html?q=foo&a=bar into an object {q: foo, a: bar}.
function getqueryobj() {
    var url = document.location.href;
    var i = url.indexOf('?');
    var args = url.slice(i+1);

    var obj = {};
    for (let arg of args.split('&') ) {
        if (arg.indexOf('=') >= 0) {
            let v = unescape(arg).split('=');
            obj[v[0]] = v[1];
        }
    }
    return obj;
}


function onload() {
    var query = getqueryobj();
    redraw(query);
}


document.addEventListener("DOMContentLoaded", onload);

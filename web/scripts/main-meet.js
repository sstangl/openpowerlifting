// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var contentDiv = document.getElementsByClassName('content')[0];
var meetString = document.getElementById('meet');
var editString = document.getElementById('editurl');

// TODO: Actually have a toggle for this.
var usingLbs = true;

// TODO: Share this with main-index.js. A bunch of functions can be shared, actually.
function weight(kg) {
    if (kg === undefined)
        return '';
    if (!usingLbs)
        return String(kg);
    return String(common.kg2lbs(kg));
}

function parseWeightClass(x) {
    if (x === undefined)
        return '';
    if (!usingLbs)
        return String(x);
    if (typeof x === 'number')
        return String(Math.floor(common.kg2lbs(x)));
    return String(Math.floor(common.kg2lbs(x.split('+')[0]))) + '+';
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


function weightMax(row, cola, colb) {
    var a = row[cola];
    var b = row[colb];
    if (a === undefined)
        return weight(b);
    if (b === undefined)
        return weight(a);
    return weight(Math.max(a,b));
}


function appendtd(tr, string) {
    var td = document.createElement("td");
    td.appendChild(document.createTextNode(string));
    tr.appendChild(td);
}

function appendtdlink(tr, string, url) {
    var td = document.createElement("td");
    var a = document.createElement("a");
    a.setAttribute('href', url);
    a.appendChild(document.createTextNode(string));
    td.appendChild(a);
    td.style.whiteSpace = "nowrap";
    tr.appendChild(td);
}

function appendtdraw(tr, innerHTML) {
    var td = document.createElement("td");
    td.innerHTML = innerHTML;
    td.style.whiteSpace = "nowrap";
    tr.appendChild(td);
}


function buildtable(indices) {
    var frag = document.createDocumentFragment();
    var table = document.createElement("table");
    frag.appendChild(table);

    var thead = document.createElement("thead");
    table.appendChild(thead);
    var tr = document.createElement("tr");
    thead.appendChild(tr);

    var cols = ["Wilks Place", "Div Place", "Name", "Division", "Sex", "Age", "Equip", "Class",
                "Weight", "Squat", "Bench", "Deadlift", "Total", "Wilks"];
    for (var i = 0; i < cols.length; ++i) {
        var td = document.createElement("td");
        td.appendChild(document.createTextNode(cols[i]));
        tr.appendChild(td);
    }

    var tbody = document.createElement("tbody");
    table.appendChild(tbody);

    for (var i = 0; i < indices.length; ++i) {
        var tr = document.createElement("tr");
        tbody.appendChild(tr);

        var rowobj = common.makeRowObj(opldb.data[indices[i]], 0);
        appendtd(tr, String(i+1));
        appendtd(tr, rowobj.place);
        appendtdraw(tr, rowobj.name);
        appendtd(tr, rowobj.division);
        appendtd(tr, rowobj.sex);
        appendtd(tr, rowobj.age);
        appendtd(tr, rowobj.equip);
        appendtd(tr, rowobj.weightclass);
        appendtd(tr, rowobj.bw);
        appendtd(tr, rowobj.squat);
        appendtd(tr, rowobj.bench);
        appendtd(tr, rowobj.deadlift);
        appendtd(tr, rowobj.total);
        appendtd(tr, rowobj.wilks);
    }

    return frag;
}


function onload() {
    var query = common.getqueryobj();
    var meetid = -1;
    if (query.m) {
        meetid = db_get_meetid_by_meetpath(query.m);
    } else {
        meetid = db_get_meetid(query.f, query.d, query.n);
    }

    // Not found.
    if (meetid === -1)
        return;

    // TODO: This could be made faster by using binary search, since MeetID is sequential.
    // TODO: That's actually a bad idea, since it prevents pre-sorting.
    var indices = db_make_indices_list();
    indices = db_filter(indices, function(x) { return x[opldb.MEETID] === meetid; });

    var meetrow = meetdb.data[meetid];
    var meetfed = meetrow[meetdb.FEDERATION];
    var meetdate = meetrow[meetdb.DATE];
    var meetname = meetrow[meetdb.MEETNAME];
    var meetpath = meetrow[meetdb.MEETPATH];
    var editurl = "https://github.com/sstangl/openpowerlifting/tree/master/meet-data/" + meetpath;

    meetString.innerHTML = meetfed
                           + " &nbsp;/ &nbsp;" + meetdate
                           + " &nbsp;/ &nbsp;" + meetname;

    editString.innerHTML = '<a href="' + editurl + '">Edit Meet</a>';

    indices = db_sort_numeric_maxfirst(indices, opldb.WILKS);
    contentDiv.appendChild(buildtable(indices));
}


document.addEventListener("DOMContentLoaded", onload);

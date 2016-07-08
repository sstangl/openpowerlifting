// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var contentDiv = document.getElementsByClassName('content')[0];
var meetString = document.getElementById('meet');

// TODO: Actually have a toggle for this.
var usingLbs = true;

// TODO: Share this with main-index.js. A bunch of functions can be shared, actually.
function weight(kg) {
    if (kg === undefined)
        return '';
    if (!usingLbs)
        return String(kg);
    return String(Math.round(kg * 2.20462262 * 100) / 100);
}

function parseWeightClass(x) {
    if (x === undefined)
        return '';
    if (!usingLbs)
        return String(x);
    if (typeof x === 'number')
        return String(Math.round(common.kg2lbs(x)));
    return String(Math.round(common.kg2lbs(x.split('+')[0]))) + '+';
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


// Fills in the <tbody> given the current query.
function getIndices(query) {
    // No query: nothing to draw.
    if (query.q === undefined) {
        return [];
    }

    function filter(row) {
        return row[opldb.NAME] === query.q;
    }

    var indices = db_make_indices_list();
    indices = db_filter(indices, filter);

    // Update the name display here: if the name matches something
    // in the database, we're safe from HTML injection.
    if (indices.length > 0) {
        lifterString.innerHTML = 'Meet Results for ' + query.q;
    } else {
        // Don't inject query.q here: may be HTML!
        lifterString.innerHTML = 'Lifter not found.'
    }

    var sortFn = common.getSortFn(sortCol.id, sortAsc);
    indices.sort(sortFn);
    return indices;
}


function makeItem(row) {
    var meetrow = meetdb.data[row[opldb.MEETID]];
    var name = row[opldb.NAME];

    var country = common.string(meetrow[meetdb.MEETCOUNTRY]);
    var state = common.string(meetrow[meetdb.MEETSTATE]);

    var location = country;
    if (country && state) {
        location = location + "-" + state;
    }

    return {
        place:       common.string(row[opldb.PLACE]),
        name:        common.string(name),
        fed:         common.string(meetrow[meetdb.FEDERATION]),
        date:        common.string(meetrow[meetdb.DATE]),
        location:    location,
        division:    common.string(row[opldb.DIVISION]),
        meetname:    common.string(meetrow[meetdb.MEETNAME]),
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

    var cols = ["Wilks Place", "Name", "Division", "Sex", "Age", "Equip", "Class",
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

        var rowobj = makeItem(opldb.data[indices[i]]);
        appendtd(tr, String(i+1));
        appendtdlink(tr, rowobj.name, common.makeLiftersUrl(rowobj.name));
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
    var meetid = db_get_meetid(query.f, query.d, query.n)
    if (meetid === -1)
        return;

    // TODO: This could be made faster by using binary search, since MeetID is sequential.
    var indices = db_make_indices_list();
    indices = db_filter(indices, function(x) { return x[opldb.MEETID] === meetid; });

    meetString.innerHTML = query.f + " &nbsp;/ &nbsp;" + query.d + " &nbsp;/ &nbsp;" + query.n;

    indices = db_sort_numeric_maxfirst(indices, opldb.WILKS);
    contentDiv.appendChild(buildtable(indices));
}


document.addEventListener("DOMContentLoaded", onload);

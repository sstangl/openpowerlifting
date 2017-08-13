// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var contentDiv = document.getElementsByClassName('content')[0];
var meetString = document.getElementById('meet');
var editString = document.getElementById('editurl');
var selWeightType = document.getElementById('weighttype');
var selDisplayType = document.getElementById('displaytype');

// Only compute the indices once on load.
var indices_cache;


// TODO: Share this with main-index.js. A bunch of functions can be shared, actually.
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


// Adds <tr> rows to a table for the given division indices.
function build_division_rows(unsorted_indices, tbody) {
    // Sort by TotalKg descending, then by BodyweightKg ascending.
    var indices = unsorted_indices.sort(function (a, b) {
        // First sort by Wilks, descending.
        var av = Number(opldb.data[a][opldb.TOTALKG]);
        var bv = Number(opldb.data[b][opldb.TOTALKG]);
        if (isNaN(av))
            av = Number.MIN_SAFE_INTEGER;
        if (isNaN(bv))
            bv = Number.MIN_SAFE_INTEGER;
        var result = bv - av;
        if (result != 0)
            return result;

        // Next sort by BodyweightKg, ascending.
        av = Number(opldb.data[a][opldb.BODYWEIGHTKG]);
        bv = Number(opldb.data[b][opldb.BODYWEIGHTKG]);
        if (isNaN(av))
            av = Number.MAX_SAFE_INTEGER;
        if (isNaN(bv))
            bv = Number.MAX_SAFE_INTEGER;
        return av - bv;
    });

    for (var i = 0; i < indices.length; ++i) {
        var tr = document.createElement("tr");
        tbody.appendChild(tr);

        var rowobj = common.makeRowObj(opldb.data[indices[i]], 0);

        appendtd(tr, rowobj.place);
        appendtdraw(tr, rowobj.name);
        appendtd(tr, rowobj.sex);
        appendtd(tr, rowobj.age);
        appendtd(tr, rowobj.bw);
        appendtd(tr, rowobj.squat);
        appendtd(tr, rowobj.bench);
        appendtd(tr, rowobj.deadlift);
        appendtd(tr, rowobj.total);
        appendtd(tr, rowobj.wilks);
    }
}


function infer_event(rowobj) {
    // Infer the Event if a Total is given.
    var evstr = "";
    if (rowobj.total && rowobj.squat)
        evstr = evstr + "S";
    if (rowobj.total && rowobj.bench)
        evstr = evstr + "B";
    if (rowobj.total && rowobj.deadlift)
        evstr = evstr + "D";
    return evstr;
}


function get_division_text(index) {
    var rowobj = common.makeRowObj(opldb.data[index]);

    if (!rowobj.total) {
        return "Disqualified";
    }

    var str = "";

    if (rowobj.sex === "M") {
        str += "Men ";
    } else {
        str += "Women ";
    }

    var evt = infer_event(rowobj);
    if (evt === "SBD") {
        // Do nothing.
    } else if (evt === "BD") {
        str += "Push/Pull ";
    } else if (evt === "S") {
        str += "Squat-Only ";
    } else if (evt === "B") {
        str += "Bench-Only ";
    } else if (evt === "D") {
        str += "Deadlift-Only ";
    } else if (evt) {
        str += evt + " ";
    }

    str += rowobj.equip + " ";
    str += rowobj.division + " ";

    if (rowobj.weightclass) {
        if (rowobj.weightclass.indexOf("+") == -1)
            str += '-';
        str += rowobj.weightclass; // Already takes into account kg/lbs setting.
        if (selWeightType.value === "kg") {
            str += "kg";
        } else {
            str += "lbs";
        }
    }

    return str;
}


// Draws division information into contentDiv.
// Divisions are sorted by (Event, Equipment, Sex, Division, WeightClassKg) categories.
function draw_divisions(unsorted_indices) {

    // Internal helper: reduce a rowobj to a hash based on division.
    var make_unique_division_hash = function(rowobj) {
        return infer_event(rowobj) + rowobj.equip + rowobj.sex + rowobj.division + rowobj.weightclass;
    }

    // Filter out the DQ'd lifters.
    var list_dqd = unsorted_indices.filter(function (e) {
        var place = opldb.data[e][opldb.PLACE];
        return place === "DQ" || place === "NS" || place === "DD";
    });

    var indices = unsorted_indices.filter(function (e) {
        var place = opldb.data[e][opldb.PLACE];
        return place !== "DQ" && place !== "NS" && place !== "DD";
    });

    // Filter the indices into buckets by unique division hash.
    var buckets = indices.reduce(function(acc, value) {
        var hash = make_unique_division_hash(common.makeRowObj(opldb.data[value]));
        (acc[hash] = acc[hash] || []).push(value);
        return acc;
    }, {});

    if (list_dqd.length > 0)
        buckets['DQ'] = list_dqd;

    var event_sort_order = ["SBD","BD","SB","SD","S","B","D",""];
    var equipment_sort_order = ["Raw","Wraps","Single","Multi","Straps"];
    var sex_sort_order = ["F","M"];

    // Sort the divisions.
    var divisions = Object.keys(buckets).sort(function(a,b) {
        // Get a representative rowobject from each bucket.
        var a_obj = common.makeRowObj(opldb.data[buckets[a][0]]);
        var b_obj = common.makeRowObj(opldb.data[buckets[b][0]]);

        // First, sort by event, per the event_sort_order table.
        var a_event = event_sort_order.indexOf(infer_event(a_obj));
        var b_event = event_sort_order.indexOf(infer_event(b_obj));
        if (a_event != b_event) {
            return a_event - b_event;
        }

        // Next, sort by Equipment, per equipment_sort_order.
        var a_equip = equipment_sort_order.indexOf(a_obj.equip);
        var b_equip = equipment_sort_order.indexOf(b_obj.equip);
        if (a_equip != b_equip) {
            return a_equip - b_equip;
        }

        // Next, sort by Sex, per sex_sort_order.
        var a_sex = sex_sort_order.indexOf(a_obj.sex);
        var b_sex = sex_sort_order.indexOf(b_obj.sex);
        if (a_sex != b_sex) {
            return a_sex - b_sex;
        }

        // Next, sort by Division, alphabetically.
        // Except, put the Open category first, with some detection logic.
        if (a_obj.division.indexOf("Open") >= 0 || a_obj.division.indexOf("-O") >= 0) {
            a_obj.division = "!"; // Sort earlier lexicographically.
        }
        if (b_obj.division.indexOf("Open") >= 0 || b_obj.division.indexOf("-O") >= 0) {
            b_obj.division = "!"; // Sort earlier lexicographically.
        }

        if (a_obj.division != b_obj.division) {
            return a_obj.division > b_obj.division;
        }

        // Finally, by WeightClassKg, lowest first.
        var a_wtcls = Number(a_obj.weightclass.replace("+",""));
        var b_wtcls = Number(b_obj.weightclass.replace("+",""));
        return a_wtcls >= b_wtcls;
    });


    // Output the divisions into a large table.
    var frag = document.createDocumentFragment();
    var table = document.createElement("table");
    frag.appendChild(table);

    var thead = document.createElement("thead");
    table.appendChild(thead);
    var tr = document.createElement("tr");
    thead.appendChild(tr);

    var cols = ["Div Place", "Name", "Sex", "Age",
                "Weight", "Squat", "Bench", "Deadlift", "Total", "Wilks"];
    for (var i = 0; i < cols.length; ++i) {
        var td = document.createElement("td");
        td.appendChild(document.createTextNode(cols[i]));
        tr.appendChild(td);
    }

    var tbody = document.createElement("tbody");
    table.appendChild(tbody);

    // Output the divisions into a large table.
    for (var i = 0; i < divisions.length; ++i) {
        // Output an informational row.
        tr = document.createElement("tr");
        td = document.createElement("td");
        td.setAttribute("colspan", String(cols.length));
        td.className = "meetdivisiontext";
        td.appendChild(document.createTextNode(get_division_text(buckets[divisions[i]][0])));
        tr.appendChild(td);
        tbody.appendChild(tr);

        // Sort and dump out the division.
        build_division_rows(buckets[divisions[i]], tbody);
    }

    contentDiv.appendChild(frag);
}


function build_wilks_table(unsorted_indices) {
    // Sort by Wilks descending, then by BodyweightKg ascending.
    var indices = unsorted_indices.sort(function (a, b) {
        // First sort by Wilks, descending.
        var av = Number(opldb.data[a][opldb.WILKS]);
        var bv = Number(opldb.data[b][opldb.WILKS]);
        if (isNaN(av))
            av = Number.MIN_SAFE_INTEGER;
        if (isNaN(bv))
            bv = Number.MIN_SAFE_INTEGER;
        var result = bv - av;
        if (result != 0)
            return result;

        // Next sort by BodyweightKg, ascending.
        av = Number(opldb.data[a][opldb.BODYWEIGHTKG]);
        bv = Number(opldb.data[b][opldb.BODYWEIGHTKG]);
        if (isNaN(av))
            av = Number.MAX_SAFE_INTEGER;
        if (isNaN(bv))
            bv = Number.MAX_SAFE_INTEGER;
        return av - bv;
    });

    // Only show each lifter once, since divisions are hidden.
    indices = db_uniq_lifter(indices);

    var frag = document.createDocumentFragment();
    var table = document.createElement("table");
    frag.appendChild(table);

    var thead = document.createElement("thead");
    table.appendChild(thead);
    var tr = document.createElement("tr");
    thead.appendChild(tr);

    var cols = ["Wilks Place", "Name", "Sex", "Age", "Equip", "Class",
                "Weight", "Squat", "Bench", "Deadlift", "Total", "Wilks"];
    for (var i = 0; i < cols.length; ++i) {
        var td = document.createElement("td");
        td.appendChild(document.createTextNode(cols[i]));
        tr.appendChild(td);
    }

    var tbody = document.createElement("tbody");
    table.appendChild(tbody);

    var wilkscounter = 1;

    for (var i = 0; i < indices.length; ++i) {
        var tr = document.createElement("tr");
        tbody.appendChild(tr);

        var rowobj = common.makeRowObj(opldb.data[indices[i]], 0);

        if (rowobj.place !== "DQ" && rowobj.place !== "DD" && rowobj.place !== "NS") {
            appendtd(tr, String(wilkscounter));
            wilkscounter += 1;
        } else {
            appendtd(tr, "");
        }

        appendtdraw(tr, rowobj.name);
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


function addSelectorListeners(selector) {
    selector.addEventListener("change", redraw);
    selector.addEventListener("keydown", function()
        {
            setTimeout(redraw, 0);
        }
    );
}


function addEventListeners() {
    addSelectorListeners(selWeightType);
    addSelectorListeners(selDisplayType);
}


function redraw() {
    // Knock out all of the content.
    while (contentDiv.firstChild) {
        contentDiv.removeChild(contentDiv.firstChild);
    }
    // Redraw the content.
    if (selDisplayType.value === "wilks") {
        contentDiv.appendChild(build_wilks_table(indices_cache));
    } else {
        draw_divisions(indices_cache);
    }
}


function onload() {
    addEventListeners();

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

    indices_cache = indices;
    redraw();
}


document.addEventListener("DOMContentLoaded", onload);

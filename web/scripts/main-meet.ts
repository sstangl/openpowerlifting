// vim: set ts=4 sts=4 sw=4 et:
'use strict';

import * as common from './common'
import * as database from './database'
import { OplDBColumn, MeetDBColumn } from './database'

// Appease the TypeScript compiler.
declare const opldb;
declare const meetdb;

const contentDiv = document.getElementsByClassName('content')[0];
const selWeightType = document.getElementById('weighttype') as HTMLSelectElement;
const selDisplayType = document.getElementById('displaytype') as HTMLSelectElement;

let meetString = document.getElementById('meet');
let editString = document.getElementById('editurl');

// Only compute the indices once on load.
let indices_cache;


function maketd(s: string) {
    let td = document.createElement('td');
    td.appendChild(document.createTextNode(s));
    return td;
}


function weightMax(row, cola, colb) {
    let a = row[cola];
    let b = row[colb];
    if (a === undefined)
        return common.weight(b);
    if (b === undefined)
        return common.weight(a);
    return common.weight(Math.max(a,b));
}


function appendtd(tr: HTMLTableRowElement, s: string) {
    let td = document.createElement("td");
    td.appendChild(document.createTextNode(s));
    tr.appendChild(td);
}

function appendtdlink(tr: HTMLTableRowElement, s: string, url) {
    let td = document.createElement("td");
    let a = document.createElement("a");
    a.setAttribute('href', url);
    a.appendChild(document.createTextNode(s));
    td.appendChild(a);
    td.style.whiteSpace = "nowrap";
    tr.appendChild(td);
}

function appendtdraw(tr: HTMLTableRowElement, innerHTML: string) {
    let td = document.createElement("td");
    td.innerHTML = innerHTML;
    td.style.whiteSpace = "nowrap";
    tr.appendChild(td);
}


// Adds <tr> rows to a table for the given division indices.
function build_division_rows(unsorted_indices: number[], tbody)
{
    // Sort by TotalKg descending, then by BodyweightKg ascending.
    let indices = unsorted_indices.sort(function (a, b) {
        // First sort by Wilks, descending.
        let av = Number(opldb.data[a][OplDBColumn.TotalKg]);
        let bv = Number(opldb.data[b][OplDBColumn.TotalKg]);
        if (isNaN(av))
            av = Number.MIN_VALUE;
        if (isNaN(bv))
            bv = Number.MIN_VALUE;
        let result = bv - av;
        if (result != 0)
            return result;

        // Next sort by BodyweightKg, ascending.
        av = Number(opldb.data[a][OplDBColumn.BodyweightKg]);
        bv = Number(opldb.data[b][OplDBColumn.BodyweightKg]);
        if (isNaN(av))
            av = Number.MAX_VALUE;
        if (isNaN(bv))
            bv = Number.MAX_VALUE;
        return av - bv;
    });

    for (let i = 0; i < indices.length; ++i) {
        let tr = document.createElement("tr");
        tbody.appendChild(tr);

        let rowobj = common.makeRowObj(opldb.data[indices[i]], 0);

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


function infer_event(rowobj: common.RowObject): string {
    // Infer the Event if a Total is given.
    let evstr = "";
    if (rowobj.total && rowobj.squat)
        evstr = evstr + "S";
    if (rowobj.total && rowobj.bench)
        evstr = evstr + "B";
    if (rowobj.total && rowobj.deadlift)
        evstr = evstr + "D";
    return evstr;
}


function get_division_text(index: number) {
    let rowobj = common.makeRowObj(opldb.data[index]);

    if (!rowobj.total) {
        return "Disqualified";
    }

    let str = "";

    if (rowobj.sex === "M") {
        str += "Men ";
    } else {
        str += "Women ";
    }

    let evt = infer_event(rowobj);
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
function draw_divisions(unsorted_indices: number[]) {

    // Internal helper: reduce a rowobj to a hash based on division.
    let make_unique_division_hash = function(rowobj: common.RowObject) {
        return infer_event(rowobj) + rowobj.equip + rowobj.sex + rowobj.division + rowobj.weightclass;
    }

    // Filter out the DQ'd lifters.
    let list_dqd = unsorted_indices.filter(function (e) {
        let place = opldb.data[e][OplDBColumn.Place];
        return place === "DQ" || place === "NS" || place === "DD";
    });

    let indices = unsorted_indices.filter(function (e) {
        let place = opldb.data[e][OplDBColumn.Place];
        return place !== "DQ" && place !== "NS" && place !== "DD";
    });

    // Filter the indices into buckets by unique division hash.
    let buckets = indices.reduce(function(acc, value) {
        let hash = make_unique_division_hash(common.makeRowObj(opldb.data[value]));
        (acc[hash] = acc[hash] || []).push(value);
        return acc;
    }, {});

    if (list_dqd.length > 0)
        buckets['DQ'] = list_dqd;

    let event_sort_order = ["SBD","BD","SB","SD","S","B","D",""];
    let equipment_sort_order = ["Raw","Wraps","Single","Multi","Straps"];
    let sex_sort_order = ["F","M"];

    // Sort the divisions.
    let divisions = Object.keys(buckets).sort(function(a: string,b: string) {
        // Get a representative rowobject from each bucket.
        let a_obj = common.makeRowObj(opldb.data[buckets[a][0]]);
        let b_obj = common.makeRowObj(opldb.data[buckets[b][0]]);

        // First, sort by event, per the event_sort_order table.
        let a_event = event_sort_order.indexOf(infer_event(a_obj));
        let b_event = event_sort_order.indexOf(infer_event(b_obj));
        if (a_event != b_event) {
            return a_event - b_event;
        }

        // Next, sort by Equipment, per equipment_sort_order.
        let a_equip = equipment_sort_order.indexOf(a_obj.equip);
        let b_equip = equipment_sort_order.indexOf(b_obj.equip);
        if (a_equip != b_equip) {
            return a_equip - b_equip;
        }

        // Next, sort by Sex, per sex_sort_order.
        let a_sex = sex_sort_order.indexOf(a_obj.sex);
        let b_sex = sex_sort_order.indexOf(b_obj.sex);
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
            if (a_obj.division > b_obj.division)
                return 1;
            return -1;
        }

        // Finally, by WeightClassKg, lowest first.
        let a_wtcls = Number(a_obj.weightclass.replace("+",""));
        let b_wtcls = Number(b_obj.weightclass.replace("+",""));
        return a_wtcls - b_wtcls;
    });


    // Output the divisions into a large table.
    let frag = document.createDocumentFragment();
    let table = document.createElement("table");
    frag.appendChild(table);

    let thead = document.createElement("thead");
    table.appendChild(thead);
    let tr = document.createElement("tr");
    thead.appendChild(tr);

    let cols = ["Div Place", "Name", "Sex", "Age",
                "Weight", "Squat", "Bench", "Deadlift", "Total", "Wilks"];
    for (let i = 0; i < cols.length; ++i) {
        let td = document.createElement("td");
        td.appendChild(document.createTextNode(cols[i]));
        tr.appendChild(td);
    }

    let tbody = document.createElement("tbody");
    table.appendChild(tbody);

    // Output the divisions into a large table.
    for (let i = 0; i < divisions.length; ++i) {
        // Output an informational row.
        let tr = document.createElement("tr");
        let td = document.createElement("td");
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


function build_wilks_table(unsorted_indices: number[]) {
    // Sort by Wilks descending, then by BodyweightKg ascending.
    let indices = unsorted_indices.sort(function (a, b) {
        // First sort by Wilks, descending.
        let av = Number(opldb.data[a][OplDBColumn.Wilks]);
        let bv = Number(opldb.data[b][OplDBColumn.Wilks]);
        if (isNaN(av))
            av = Number.MIN_VALUE;
        if (isNaN(bv))
            bv = Number.MIN_VALUE;
        let result = bv - av;
        if (result != 0)
            return result;

        // Next sort by BodyweightKg, ascending.
        av = Number(opldb.data[a][OplDBColumn.BodyweightKg]);
        bv = Number(opldb.data[b][OplDBColumn.BodyweightKg]);
        if (isNaN(av))
            av = Number.MAX_VALUE;
        if (isNaN(bv))
            bv = Number.MAX_VALUE;
        return av - bv;
    });

    // Only show each lifter once, since divisions are hidden.
    indices = database.db_uniq_lifter(indices);

    let frag = document.createDocumentFragment();
    let table = document.createElement("table");
    frag.appendChild(table);

    let thead = document.createElement("thead");
    table.appendChild(thead);
    let tr = document.createElement("tr");
    thead.appendChild(tr);

    let cols = ["Wilks Place", "Name", "Sex", "Age", "Equip", "Class",
                "Weight", "Squat", "Bench", "Deadlift", "Total", "Wilks"];
    for (let i = 0; i < cols.length; ++i) {
        let td = document.createElement("td");
        td.appendChild(document.createTextNode(cols[i]));
        tr.appendChild(td);
    }

    let tbody = document.createElement("tbody");
    table.appendChild(tbody);

    let wilkscounter = 1;

    for (let i = 0; i < indices.length; ++i) {
        let tr = document.createElement("tr");
        tbody.appendChild(tr);

        let rowobj = common.makeRowObj(opldb.data[indices[i]], 0);

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


function addSelectorListeners(selector: HTMLSelectElement) {
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
    common.setWeightTypeState(selWeightType.value);

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

    let query = common.getqueryobj();
    let meetid = -1;
    if (query.m === undefined)
        return;

    meetid = database.db_get_meetid_by_meetpath(query.m);

    // Not found.
    if (meetid === -1)
        return;

    let indices = database.db_make_indices_list();
    indices = database.db_filter(indices, function(x) { return x[OplDBColumn.MeetID] === meetid; });

    let meetrow = meetdb.data[meetid];
    let meetfed = meetrow[MeetDBColumn.Federation];
    let meetdate = meetrow[MeetDBColumn.Date];
    let meetname = meetrow[MeetDBColumn.MeetName];
    let meetpath = meetrow[MeetDBColumn.MeetPath];
    let editurl = "https://gitlab.com/openpowerlifting/opl-data/tree/master/meet-data/" + meetpath;

    meetString.innerHTML = meetfed
                           + " &nbsp;/ &nbsp;" + meetdate
                           + " &nbsp;/ &nbsp;" + meetname;

    editString.innerHTML = '<a href="' + editurl + '">Edit Meet</a>';

    indices_cache = indices;
    redraw();
}


document.addEventListener("DOMContentLoaded", onload);

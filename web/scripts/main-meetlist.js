// vim: set ts=4 sts=4 sw=4 et:
'use strict';

import { flashRow, getqueryobj } from './common.js'


var searchInfo = {lastrowid: 0, laststr: ''};
var searchfield = document.getElementById("searchfield");
var searchbutton = document.getElementById("searchbutton");
var selFed = document.getElementById("fedselect");
var meettable = document.getElementById("meettable");


function scrollIntoView(obj) {
    var curtop = 0;
    if (obj.offsetParent) {
        do {
            curtop += obj.offsetTop;
        } while (obj = obj.offsetParent);
    }

    // Compensate for the fixed topbar.
    // FIXME: Figure out the topbar height by just asking the topbar.
    curtop -= 80;
    window.scroll(0, [curtop]);
}


function _search_from(query, tds, rowid) {
    for (var i = rowid; i < tds.length; ++i) {
        var str = tds[i].textContent.toLowerCase();
        if (str.indexOf(query) >= 0) {
            return i;
        }
    }
    return -1;
}


function search() {
    var query = searchfield.value.toLowerCase().trim().replace("  "," ");
    if (!query) {
        return;
    }

    // <td> elements containing meet names have class "meetname".
    var tds = document.getElementsByClassName("meetname");

    var startrowid = 0;
    // If the search string hasn't changed, do a "next"-style search.
    if (query === searchInfo.laststr) {
        // FIXME: Search from top of viewport somehow and remove lastrowid.
        startrowid = searchInfo.lastrowid + 1;
    }

    var rowid = _search_from(query, tds, startrowid);

    // If nothing was found in "next" mode, try searching again from the top.
    if (startrowid > 0 && rowid === -1) {
        rowid = _search_from(query, tds, 0);
    }

    if (rowid >= 0) {
        scrollIntoView(tds[rowid]);
        flashRow(tds[rowid].parentNode);

        searchInfo.laststr = query;
        searchInfo.lastrowid = rowid;
        searchbutton.innerHTML = "Next";
    }
}


function searchOnEnter(keyevent) {
    if (keyevent.keyCode === 13 || keyevent.key === "Enter") {
        search();
    } else {
        searchbutton.innerHTML = "Search";
    }
}


function selectfed() {
    var fedlist = selFed.value;
    // If the selector is "all", remove all the classes.
    if (fedlist === "all") {
        meettable.className = "";
        return;
    }

    // Otherwise, the selector is a comma-separated list of federation names.
    // Also include the class "selectorActive" to get the CSS working.
    //
    // An underscore is prepended to each federation to handle federations
    // beginning with numbers, like 365Strong.
    var fedspaces = '_' + fedlist.replace(new RegExp(',', 'g'), ' _');
    meettable.className = "selectorActive " + fedspaces;
}


function onload() {
    searchfield.addEventListener("keypress", searchOnEnter, false);
    searchbutton.addEventListener("click", search, false);

    selFed.addEventListener("change", selectfed);
    selFed.addEventListener("keydown", function()
        {
            setTimeout(selectfed, 0);
        }
    );

    var query = getqueryobj();
    if (query.fed !== undefined) {
        selFed.value = query.fed;
    }

    // Also handle the case of it being set by the browser.
    if (selFed.value !== "all") {
        selectfed();
    }
}


document.addEventListener("DOMContentLoaded", onload);

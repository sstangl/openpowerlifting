// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var searchInfo = {lastrowid: 0, laststr: ''};
var searchfield = document.getElementById("searchfield");
var searchbutton = document.getElementById("searchbutton");


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
        common.flashRow(tds[rowid].parentNode);

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


function onload() {
    searchfield.addEventListener("keypress", searchOnEnter, false);
    searchbutton.addEventListener("click", search, false);
}


document.addEventListener("DOMContentLoaded", onload);

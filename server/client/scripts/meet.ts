// vim: set ts=4 sts=4 sw=4 et:
//
// Implementation of main logic for the Meet page.

'use strict';

// These are generated inline via templates/meet.html.tera.
declare const path_if_by_wilks: string;
declare const path_if_by_glossbrenner: string;
declare const path_if_by_nasa: string;
declare const path_if_by_ipfpoints: string;
declare const path_if_by_division: string;

let selSort: HTMLSelectElement;

// When selectors are changed, the URL in the address bar should
// change to match.
function redirect() {
    switch (selSort.value) {
        case "by-division":
            window.location.href = path_if_by_division;
            break;
        case "by-glossbrenner":
            window.location.href = path_if_by_glossbrenner;
            break;
        case "by-ipf-points":
            window.location.href = path_if_by_ipfpoints;
            break;
        case "by-nasa":
            window.location.href = path_if_by_nasa;
            break;
        case "by-wilks":
            window.location.href = path_if_by_wilks;
            break;
    }
}

function onLoad() {
    selSort = document.getElementById("sortselect") as HTMLSelectElement;
    selSort.addEventListener("change", redirect);
}

document.addEventListener("DOMContentLoaded", onLoad);

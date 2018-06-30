// vim: set ts=4 sts=4 sw=4 et:
//
// Implementation of main logic for the MeetList page.

'use strict';

let selFed: HTMLSelectElement;
let selYear: HTMLSelectElement;

// Returns a string like "/uspa/2018", or the empty string
// for the default selection.
function selection_to_path(): string {
    let url = "";
    if (selFed.value !== "all") {
        url += "/" + selFed.value;
    }
    if (selYear.value !== "all") {
        url += "/" + selYear.value;
    }
    return url;
}

// When selectors are changed, the URL in the address bar should
// change to match.
function reload() {
    let path = selection_to_path();

    if (path === "") {
        window.location.href = "/mlist";
    } else {
        window.location.href = "/mlist" + path;
    }
}

function addSelectorListeners(selector) {
    selector.addEventListener("change", reload);
}

function addEventListeners() {
    selFed = document.getElementById("fedselect") as HTMLSelectElement;
    selYear = document.getElementById("yearselect") as HTMLSelectElement;

    addSelectorListeners(selFed);
    addSelectorListeners(selYear);
}

document.addEventListener("DOMContentLoaded", addEventListeners);

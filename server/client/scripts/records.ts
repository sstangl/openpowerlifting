// vim: set ts=4 sts=4 sw=4 et:
//
// Implementation of main logic for the Records page.

'use strict';

let selEquipment: HTMLSelectElement;
let selClassKind: HTMLSelectElement;
let selSex: HTMLSelectElement;
let selFederation: HTMLSelectElement;

// Returns a string like "/women/uspa", or the empty string
// for the default selection.
function records_selection_to_path(): string {
    let url = "";
    if (selEquipment.value !== "raw_wraps") {
        url += "/" + selEquipment.value;
    }
    if (selClassKind.value !== "traditional-classes") {
        url += "/" + selClassKind.value;
    }
    if (selSex.value !== "men") {
        url += "/" + selSex.value;
    }
    if (selFederation.value !== "all") {
        url += "/" + selFederation.value;
    }
    return url;
}

// When selectors are changed, the URL in the address bar should
// change to match.
function records_reload() {
    let path = records_selection_to_path();

    if (path === "") {
        window.location.href = "/records";
    } else {
        window.location.href = "/records" + path;
    }
}

function records_addSelectorListeners(selector) {
    selector.addEventListener("change", records_reload);
}

function records_addEventListeners() {
    selEquipment = document.getElementById("equipmentselect") as HTMLSelectElement;
    selClassKind = document.getElementById("classkindselect") as HTMLSelectElement;
    selSex = document.getElementById("sexselect") as HTMLSelectElement;
    selFederation = document.getElementById("fedselect") as HTMLSelectElement;

    records_addSelectorListeners(selEquipment);
    records_addSelectorListeners(selClassKind);
    records_addSelectorListeners(selSex);
    records_addSelectorListeners(selFederation);
}

document.addEventListener("DOMContentLoaded", records_addEventListeners);

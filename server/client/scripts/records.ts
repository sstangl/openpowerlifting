// vim: set ts=4 sts=4 sw=4 et:
//
// Implementation of main logic for the Records page.

'use strict';

// Variables provided by the server.
declare const urlprefix: string;

declare const default_equipment: string;
declare const default_classkind: string | undefined;
declare const default_fed: string;
declare const default_sex: string;
declare const default_ageclass: string;
declare const default_year: string;

let selEquipment: HTMLSelectElement;
let selClassKind: HTMLSelectElement | null; // OpenIPF doesn't use this.
let selSex: HTMLSelectElement;
let selFederation: HTMLSelectElement;
let selAgeClass: HTMLSelectElement;
let selRecordsYear: HTMLSelectElement;

// Returns a string like "/women/uspa", or the empty string
// for the default selection.
function records_selection_to_path(): string {
    let url = "";
    if (selEquipment.value !== default_equipment) {
        url += "/" + selEquipment.value;
    }
    if (selClassKind && selClassKind.value !== default_classkind) {
        url += "/" + selClassKind.value;
    }
    if (selFederation.value !== default_fed) {
        url += "/" + selFederation.value;
    }
    if (selSex.value !== default_sex) {
        url += "/" + selSex.value;
    }
    if (selAgeClass.value !== default_ageclass) {
        url += "/" + selAgeClass.value;
    }
    if (selRecordsYear.value !== default_year) {
        url += "/" + selRecordsYear.value;
    }
    return url;
}

// When selectors are changed, the URL in the address bar should
// change to match.
function records_reload() {
    let path = records_selection_to_path();

    if (path === "") {
        window.location.href = urlprefix + "records";
    } else {
        window.location.href = urlprefix + "records" + path;
    }
}

function records_addSelectorListeners(selector) {
    if (selector) {
        selector.addEventListener("change", records_reload);
    }
}

function records_addEventListeners() {
    selEquipment = document.getElementById("equipmentselect") as HTMLSelectElement;
    selClassKind = document.getElementById("classkindselect") as HTMLSelectElement;
    selSex = document.getElementById("sexselect") as HTMLSelectElement;
    selFederation = document.getElementById("fedselect") as HTMLSelectElement;
    selAgeClass = document.getElementById("ageselect") as HTMLSelectElement;
    selRecordsYear = document.getElementById("yearselect") as HTMLSelectElement;

    records_addSelectorListeners(selEquipment);
    records_addSelectorListeners(selClassKind);
    records_addSelectorListeners(selSex);
    records_addSelectorListeners(selFederation);
    records_addSelectorListeners(selAgeClass);
    records_addSelectorListeners(selRecordsYear);
}

document.addEventListener("DOMContentLoaded", records_addEventListeners);

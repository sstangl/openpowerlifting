// vim: set ts=4 sts=4 sw=4 et:
//
// This file is part of OpenPowerlifting, an open archive of powerlifting data.
// Copyright (C) 2019 The OpenPowerlifting Project.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
let selState: HTMLSelectElement;

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
    if (selState.value !== "") {
        url += "/" + selState.value;
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

function records_addSelectorListeners(selector?: HTMLSelectElement) {
    if (selector) {
        selector.addEventListener("change", records_reload);
    }
}

// Render the selected filters into the header, for use on mobile devices.
//
// On desktop, the selected filters are visually obvious, because they're
// always on the screen. On mobile, the filters are hidden in a menu.
// So instead we show breadcrumbs for filters that differ from the defaults.
function renderSelectedFilters(): void {
    const div = document.getElementById("selectedFilters");
    if (div === null) return;

    // Clear old filters.
    div.innerHTML = "";

    // Helper function to create a new filter breadcrumb.
    function newFilter(parent: HTMLElement, label: string): void {
        const item = document.createElement("span");
        item.setAttribute("class", "selected-filter");
        item.innerHTML = label;
        parent.appendChild(item);
    }

    // Create new filters.
    if (selEquipment.value !== default_equipment) {
        newFilter(div, selEquipment.selectedOptions[0].label);
    }
    if (selClassKind !== null && selClassKind.value !== default_classkind) {
        newFilter(div, selClassKind.selectedOptions[0].label);
    }
    if (selSex.value !== default_sex) {
        newFilter(div, selSex.selectedOptions[0].label);
    }
    if (selFederation.value !== default_fed) {
        let label = selFederation.selectedOptions[0].label;

        // If there is " - " in the label, then it's the federation acronym
        // followed by the expansion. Just include the acronym.
        label = label.split(" - ")[0];
        newFilter(div, label);
    }

    if (selAgeClass.value !== default_ageclass) {
        newFilter(div, selAgeClass.selectedOptions[0].label);
    }
    if (selRecordsYear.value !== default_year) {
        newFilter(div, selRecordsYear.selectedOptions[0].label);
    }
    if (selState.value !== "") {
        newFilter(div, selState.selectedOptions[0].label);
    }
}

function initRecords() {
    selEquipment = document.getElementById("equipmentselect") as HTMLSelectElement;
    selClassKind = document.getElementById("classkindselect") as HTMLSelectElement;
    selSex = document.getElementById("sexselect") as HTMLSelectElement;
    selFederation = document.getElementById("fedselect") as HTMLSelectElement;
    selAgeClass = document.getElementById("ageselect") as HTMLSelectElement;
    selRecordsYear = document.getElementById("yearselect") as HTMLSelectElement;
    selState = document.getElementById("stateselect") as HTMLSelectElement;

    records_addSelectorListeners(selEquipment);
    records_addSelectorListeners(selClassKind);
    records_addSelectorListeners(selSex);
    records_addSelectorListeners(selFederation);
    records_addSelectorListeners(selAgeClass);
    records_addSelectorListeners(selRecordsYear);
    // Intentionally missing selState: it's a hidden element.

    renderSelectedFilters();
}

export {
    initRecords
}

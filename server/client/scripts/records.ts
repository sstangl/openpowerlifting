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

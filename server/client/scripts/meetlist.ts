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

function addSelectorListeners(selector: HTMLSelectElement): void {
    selector.addEventListener("change", reload);
}

function addEventListeners(): void {
    selFed = document.getElementById("fedselect") as HTMLSelectElement;
    selYear = document.getElementById("yearselect") as HTMLSelectElement;

    addSelectorListeners(selFed);
    addSelectorListeners(selYear);
}

document.addEventListener("DOMContentLoaded", addEventListeners);

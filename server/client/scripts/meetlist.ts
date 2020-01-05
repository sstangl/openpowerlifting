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

// Variables provided by the server.
declare const urlprefix: string;
declare const default_fed: string;
declare const default_year: string;

let selFed: HTMLSelectElement;
let selYear: HTMLSelectElement;

// Returns a string like "/uspa/2018", or the empty string
// for the default selection.
function selection_to_path(): string {
    let url = "";
    if (selFed.value !== default_fed) {
        url += "/" + selFed.value;
    }
    if (selYear.value !== default_year) {
        url += "/" + selYear.value;
    }
    return url;
}

// When selectors are changed, the URL in the address bar should
// change to match.
function reload() {
    let path = selection_to_path();

    if (path === "") {
        window.location.href = urlprefix + "mlist";
    } else {
        window.location.href = urlprefix + "mlist" + path;
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

    if (selFed.value !== 'all') {
        let label = selFed.selectedOptions[0].label;

        // If there is " - " in the label, then it's the federation acronym
        // followed by the expansion. Just include the acronym.
        label = label.split(" - ")[0];
        newFilter(div, label);
    }

    if (selYear.value !== 'all') {
        newFilter(div, selYear.selectedOptions[0].label);
    }
}


function addSelectorListeners(selector: HTMLSelectElement): void {
    selector.addEventListener("change", reload);
}

function initMeetList(): void {
    selFed = document.getElementById("fedselect") as HTMLSelectElement;
    selYear = document.getElementById("yearselect") as HTMLSelectElement;

    addSelectorListeners(selFed);
    addSelectorListeners(selYear);

    renderSelectedFilters();
}

export {
    initMeetList
}

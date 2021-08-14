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

// Implementation of main logic for the Rankings page.

'use strict';

import { RemoteCache, WorkItem, Column } from "./remotecache";
import { RankingsSearcher } from "./search";
import { isMobile } from "./mobile";

// Variables provided by the server.
declare const initial_data: Object[];
declare const urlprefix: string; // For navigation only, not for API calls.

declare const default_equipment: string;
declare const default_weightclass: string;
declare const default_fed: string;
declare const default_sex: string;
declare const default_ageclass: string;
declare const default_year: string;
declare const default_event: string;
declare const default_sort: string;

declare const translation_column_formulaplace: string;
declare const translation_column_liftername: string;
declare const translation_column_federation: string;
declare const translation_column_date: string;
declare const translation_column_location: string;
declare const translation_column_sex: string;
declare const translation_column_age: string;
declare const translation_column_equipment: string;
declare const translation_column_weightclass: string;
declare const translation_column_bodyweight: string;
declare const translation_column_squat: string;
declare const translation_column_bench: string;
declare const translation_column_deadlift: string;
declare const translation_column_total: string;
declare const translation_column_wilks: string;
declare const translation_column_mcculloch: string;
declare const translation_column_glossbrenner: string;
declare const translation_column_goodlift: string;
declare const translation_column_dots: string;
declare const translation_column_wilks2020: string;
declare const translation_default_sort: string;

let global_grid: any;  // The SlickGrid.
let global_cache: any;  // The active RemoteCache rendered in the SlickGrid.

// A RemoteCache in line to replace the global_cache, but which hasn't
// had its initial data loaded yet, and is still waiting on an AJAX response.
//
// The pending_cache is swapped to overwrite the global_cache when its onFirstLoad
// event fires, by the event handler. Swapping only when data is available avoids
// flickering. The concept is similar to the double-sided OpenGL framebuffer.
let pending_cache: any;

// Tells an event handler to not create a new history state.
// Used when navigating backwards/forwards, instead of by changing a selector.
let global_suppress_history_changes: boolean = false;

let searcher: any;
let searchInfo = {laststr: ''};

let selEquipment: HTMLSelectElement;
let selWeightClass: HTMLSelectElement;
let selFed: HTMLSelectElement;
let selAgeClass: HTMLSelectElement;
let selYear: HTMLSelectElement;
let selSex: HTMLSelectElement;
let selEvent: HTMLSelectElement;
let selSort: HTMLSelectElement;

let searchField: HTMLInputElement;
let searchButton: HTMLButtonElement;


// Refers to the global_cache for replacing the underlying RemoteCache
// when selectors change.
function makeDataProvider() {
    return {
        getLength: function() { return global_cache.getLength(); },
        getItem: function(idx: number) {
            let entry: (string | number)[] = global_cache.rows[idx];
            if (entry === undefined) {
                return;
            }

            let loc = entry[Column.Country];
            if (entry[Column.Country] && entry[Column.State]) {
                loc = loc + "-" + entry[Column.State];
            }

            let name = '<a class="' + entry[Column.Color] +
                '" href="' + urlprefix + 'u/' + entry[Column.Username] + '">' +
                entry[Column.Name] + '</a>';

            if (entry[Column.Instagram]) {
                name += '<a href="https://www.instagram.com/' + entry[Column.Instagram] +
                        '" class="instagram" rel="noopener" target="_blank">' +
                        '<i class="iglogo"></i></a>';
            }

            if (entry[Column.Vkontakte]) {
                name += '<a href="https://vk.com/' + entry[Column.Vkontakte] +
                        '" class="instagram" rel="noopener" target="_blank">' +
                        '<i class="vklogo"></i></a>';
            }

            const date = '<a href="' + urlprefix + 'm/' + entry[Column.Path] + '">' +
                entry[Column.Date] + '</a>';

            return {
                rank: entry[Column.Rank],
                name: name,
                fed: entry[Column.Federation],
                date: date,
                loc: loc,
                sex: entry[Column.Sex],
                age: entry[Column.Age],
                equipment: entry[Column.Equipment],
                weightclass: entry[Column.WeightClass],
                bodyweight: entry[Column.Bodyweight],
                squat: '<span class="squat">' + entry[Column.Squat] + '</span>',
                bench: '<span class="bench">' + entry[Column.Bench] + '</span>',
                deadlift: '<span class="deadlift">' + entry[Column.Deadlift] + '</span>',
                total: entry[Column.Total],
                points: entry[Column.Points],
                idx: idx
            };
        }
    }
}

function onResize() {
    global_grid.resizeCanvas();
}

function searchOnEnter(keyevent: any) {
    // keyCode is deprecated, but non-Firefox-desktop doesn't support key.
    if (keyevent.keyCode === 13 || keyevent.key === "Enter") {
        search();
    }
}

function search(): void {
    const query = searchField.value;
    if (!query) {
        return;
    }

    let startRow = 0;
    // If the search string hasn't changed, do a "next"-style search.
    if (query === searchInfo.laststr) {
        startRow = global_grid.getViewport().top + 1;
    }

    // Queue up an AJAX request.
    searcher.search({path: selection_to_path(), query: query, startRow: startRow});
    searchInfo.laststr = query;
}

function setSortColumn(): void {
    let sortcol = "points";
    if (selSort.value === "by-squat") {
        sortcol = "squat";
    } else if (selSort.value === "by-bench") {
        sortcol = "bench";
    } else if (selSort.value === "by-deadlift") {
        sortcol = "deadlift";
    } else if (selSort.value === "by-total") {
        sortcol = "total";
    }
    global_grid.setSortColumn(sortcol, false); // Sort descending.
}

function selection_to_points_title(): string {
    switch (selSort.value) {
        case "by-dots": return translation_column_dots;
        case "by-glossbrenner": return translation_column_glossbrenner;
        case "by-goodlift": return translation_column_goodlift;
        case "by-mcculloch": return translation_column_mcculloch;
        case "by-wilks": return translation_column_wilks;
        case "by-wilks-2020": return translation_column_wilks2020;
        default: return translation_default_sort;
    }
}

// Returns a string like "/raw/uspa", or the empty string
// for the default selection.
function selection_to_path(): string {
    let url = "";
    if (selEquipment.value !== default_equipment) {
        url += "/" + selEquipment.value;
    }
    if (selWeightClass.value !== default_weightclass) {
        url += "/" + selWeightClass.value;
    }
    if (selFed.value !== default_fed) {
        url += "/" + selFed.value;
    }

    // In certain situations, the WeightClass selector is allowed to assign Sex.
    // It looks for a tag like sex="men" or sex="women" on the <option>.
    const sexAttribute = selWeightClass.selectedOptions[0].attributes["sex"];
    if (sexAttribute !== undefined) {
        const sex = sexAttribute.value;
        selSex.value = sex;
        url += "/" + sex;
    } else if (selSex.value !== default_sex) {
        url += "/" + selSex.value;
    }

    if (selAgeClass.value !== default_ageclass) {
        url += "/" + selAgeClass.value;
    }
    if (selYear.value !== default_year) {
        url += "/" + selYear.value;
    }
    if (selEvent.value !== default_event) {
        url += "/" + selEvent.value;
    }
    if (selSort.value !== default_sort) {
        url += "/" + selSort.value;
    }
    return url;
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
    newFilter(div, selEquipment.selectedOptions[0].label);
    if (selWeightClass.value !== default_weightclass) {
        newFilter(div, selWeightClass.selectedOptions[0].label);
    }
    if (selFed.value !== default_fed) {
        let label = selFed.selectedOptions[0].label;

        // If there is " - " in the label, then it's the federation acronym
        // followed by the expansion. Just include the acronym.
        label = label.split(" - ")[0];
        newFilter(div, label);
    }
    if (selSex.value !== default_sex) {
        newFilter(div, selSex.selectedOptions[0].label);
    }
    if (selAgeClass.value !== default_ageclass) {
        newFilter(div, selAgeClass.selectedOptions[0].label);
    }
    if (selYear.value !== default_year) {
        newFilter(div, selYear.selectedOptions[0].label);
    }
    if (selEvent.value !== default_event) {
        newFilter(div, selEvent.selectedOptions[0].label);
    }
    if (selSort.value !== default_sort) {
        newFilter(div, selSort.selectedOptions[0].label);
    }
}

// Save the current selection, for use with pushing history state.
function saveSelectionState() {
    return {
        "equipment": selEquipment.value,
        "weightclass": selWeightClass.value,
        "federation": selFed.value,
        "sex": selSex.value,
        "ageclass": selAgeClass.value,
        "year": selYear.value,
        "event": selEvent.value,
        "sort": selSort.value
    };
}

// Load the current selection, for use with popping history state.
function restoreSelectionState(state: any) {
    // Although the selectors are being changed in this function,
    // we don't want the changeSelection() event handler to have any effect.
    removeAllSelectorListeners();

    selEquipment.value = state["equipment"];
    selWeightClass.value = state["weightclass"];
    selFed.value = state["federation"];
    selSex.value = state["sex"];
    selAgeClass.value = state["ageclass"];
    selYear.value = state["year"];
    selEvent.value = state["event"];
    selSort.value = state["sort"];

    addAllSelectorListeners();
}

// When selectors are changed, the URL in the address bar should
// change to match.
function changeSelection() {
    let path = selection_to_path();
    let url = path ? (urlprefix + "rankings" + path) : urlprefix;

    // Adding new history state is suppressed when this function is used
    // to cause data updates on back/forward site navigation.
    if (global_suppress_history_changes === false) {
        history.pushState(saveSelectionState(), "", url);
    }

    // Updates the global_cache -- but there's no underlying data yet.
    let cache = makeRemoteCache(path, false);

    // The AJAX request has to be forced because the length is unknown,
    // so the normal bounds-checking logic doesn't apply here.
    cache.forceData({ startRow: 0, endRow: 99 });

    // If the selection changed while a previous RemoteCache request
    // was pending, terminate that request so it doesn't race with this one.
    searcher.terminateAllRequests();
    if (pending_cache !== undefined) {
        pending_cache.terminateActiveRequests();
    }
    pending_cache = cache;

    // On mobile, the columns may change if the sort selector changes.
    if (isMobile() && global_grid instanceof Slick.Grid) {
        renderGridTable(); // Cause a re-render, changing columns.
        renderSelectedFilters(); // Update the selection indicators.
    }

}

function addSelectorListeners(selector: HTMLElement) {
    selector.addEventListener("change", changeSelection);
}

// Used when navigating through history: otherwise navigation
// would add more history events.
function removeSelectorListeners(selector: HTMLElement) {
    selector.removeEventListener("change", changeSelection);
}

function addAllSelectorListeners() {
    addSelectorListeners(selEquipment);
    addSelectorListeners(selWeightClass);
    addSelectorListeners(selFed);
    addSelectorListeners(selAgeClass);
    addSelectorListeners(selYear);
    addSelectorListeners(selSex);
    addSelectorListeners(selEvent);
    addSelectorListeners(selSort);
}

function removeAllSelectorListeners() {
    removeSelectorListeners(selEquipment);
    removeSelectorListeners(selWeightClass);
    removeSelectorListeners(selFed);
    removeSelectorListeners(selAgeClass);
    removeSelectorListeners(selYear);
    removeSelectorListeners(selSex);
    removeSelectorListeners(selEvent);
    removeSelectorListeners(selSort);
}

function initializeEventListeners() {
    selEquipment = document.getElementById("equipmentselect") as HTMLSelectElement;
    selWeightClass = document.getElementById("weightclassselect") as HTMLSelectElement;
    selFed = document.getElementById("fedselect") as HTMLSelectElement;
    selAgeClass = document.getElementById("ageselect") as HTMLSelectElement;
    selYear = document.getElementById("yearselect") as HTMLSelectElement;
    selSex = document.getElementById("sexselect") as HTMLSelectElement;
    selEvent = document.getElementById("eventselect") as HTMLSelectElement;
    selSort = document.getElementById("sortselect") as HTMLSelectElement;
    searchField = document.getElementById("searchfield") as HTMLInputElement;
    searchButton = document.getElementById("searchbutton") as HTMLButtonElement;

    addAllSelectorListeners();

    searchField.addEventListener("keypress", searchOnEnter, false);
    searchButton.addEventListener("click", search, false);

    window.addEventListener("resize", onResize, false);
    window.onpopstate = function(event: any) {
        restoreSelectionState(event.state);
        global_suppress_history_changes = true;
        changeSelection();
        global_suppress_history_changes = false;
    }
}

// Creates a new RemoteCache, pointing at the given rankings endpoint URL.
// Multiple RemoteCaches may exist simultaneously, but only one can be canonical
// and have its events hooked up to the grid.
function makeRemoteCache(path: string, use_initial_data: boolean) {
    // Construct a new RemoteCache.
    const langSelect = document.getElementById("langselect") as HTMLSelectElement;
    const unitSelect = document.getElementById("weightunits") as HTMLSelectElement;

    let data = use_initial_data ? initial_data : null;
    let cache = RemoteCache("TESTING", data, path, langSelect.value, unitSelect.value);

    // Hook up event handlers.

    // The first data load should replace the RemoteCache and realign
    // the grid to the first position.
    cache.onFirstLoad.subscribe(function (e, args: WorkItem) {
        // For sanity checking, make sure this cache is actually intended
        // to replace the active global_cache.
        if (pending_cache !== cache) {
            return;
        }
        pending_cache = undefined;

        // Terminate any ongoing AJAX requests from the existing RemoteCache.
        if (global_cache !== undefined) {
            global_cache.terminateActiveRequests();
        }
        searcher.terminateAllRequests();

        // Make this the One True Cache.
        global_cache = cache;

        // Change the Points title to have the right string.
        global_grid.updateColumnHeader("points", selection_to_points_title());
        setSortColumn();

        // Invalidate everything.
        global_grid.updateRowCount();
        global_grid.invalidateAllRows();

        // Move the grid into position.
        global_grid.scrollRowToTop(args.startRow);
        global_grid.render();
    } as any);

    // Data loads after the first should let the grid know that new
    // data is available by invalidating the current empty rows.
    cache.onDataLoaded.subscribe(function (e, args: WorkItem) {
        for (var i = args.startRow; i <= args.endRow; ++i) {
            global_grid.invalidateRow(i);
        }
        global_grid.updateRowCount();
        global_grid.render();
    } as any);

    return cache;
}

// (Re-)Renders the Grid.
//
// Mobile devices use a different grid ordering for different kinds of selections.
// For simplicity, when the selectors are changed, the Grid is just re-rendered.
function renderGridTable(): void {
    // Check templates/rankings.html.tera.
    const nameWidth = 200;
    const rowHeight = isMobile() ? 26 : 23;
    const rankWidth = 45; // five digit numbers should be fully visible
    const shortWidth = 40;
    const dateWidth = isMobile() ? 80 : 70;
    const numberWidth = 55;

    const fillerWidth = isMobile() ? 10 : 20;

    // Helper function to provide a by-value URL formatter, needed by the grid.
    function urlformatter(row, cell, value, columnDef, dataContext) {
        return value;
    }

    // Helper function to select a column by "id" property.
    function col(id: string): Object {
        return columns.find(c => c.id === id) || columns[0];
    }

    let columns = [
        {id: "filler", width: fillerWidth, minWidth: fillerWidth, focusable: false,
            selectable: false, resizable: false},
        {id: "rank", name: translation_column_formulaplace, field: "rank", width: rankWidth},
        {id: "name", name: translation_column_liftername, field: "name", width: nameWidth, formatter: urlformatter},
        {id: "fed", name: translation_column_federation, field: "fed", width: numberWidth},
        {id: "date", name: translation_column_date, field: "date", width: dateWidth, formatter: urlformatter},
        {id: "location", name: translation_column_location, field: "loc", width: dateWidth},
        {id: "sex", name: translation_column_sex, field: "sex", width: shortWidth},
        {id: "age", name: translation_column_age, field: "age", width: shortWidth},
        {id: "equipment", name: translation_column_equipment, field: "equipment", width: shortWidth},
        {id: "weightclass", name: translation_column_weightclass, field: "weightclass", width: numberWidth},
        {id: "bodyweight", name: translation_column_bodyweight, field: "bodyweight", width: numberWidth},
        {id: "squat", name: translation_column_squat, field: "squat", width: numberWidth, formatter: urlformatter},
        {id: "bench", name: translation_column_bench, field: "bench", width: numberWidth, formatter: urlformatter},
        {id: "deadlift", name: translation_column_deadlift, field: "deadlift", width: numberWidth, formatter: urlformatter},
        {id: "total", name: translation_column_total, field: "total", width: numberWidth},
        {id: "points", name: selection_to_points_title(), field: "points", width: numberWidth}
    ];

    // Mobile screens are tiny.
    // To make this usable, we intend to place the information most relevant
    // to the current selection as left as possible (to the Name).
    if (isMobile()) {
        // The first three columns are fixed.
        const acc: Array<any> = [];
        acc.push(col("filler"), col("rank"), col("name"));

        // The S/B/D/T/P columns change order based on the sorting.
        // The general idea is to place the most relevant information as close
        // to the left as possible, but Points is always to the right of Total.
        switch (selSort.value) {
            case "by-squat":
                acc.push(col("squat"), col("total"), col("points"));
                break;

            case "by-bench":
                acc.push(col("bench"), col("total"), col("points"));
                break;

            case "by-deadlift":
                acc.push(col("deadlift"), col("total"), col("points"));
                break;

            default: // Handles "by-total" and the various by-points sortings.
                acc.push(col("total"), col("points"));
                acc.push(col("squat"), col("bench"), col("deadlift"));
                break;
        }

        // The final columns are fixed, the order not being particularly important.
        // Just tried to guess what people might be most likely to look for.
        acc.push(col("sex"), col("age"));
        acc.push(col("equipment"), col("weightclass"), col("bodyweight"));
        acc.push(col("fed"), col("date"), col("location"));

        // Use these new columns instead.
        columns = acc;
    }

    const options = {
        enableColumnReorder: false,
        forceSyncScrolling: false,
        rowHeight: rowHeight,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing",

        // On mobile, columns need their full width for visibility.
        // The user can scroll horizontally.
        forceFitColumns: (isMobile() ? false : true)
    }

    global_grid = new Slick.Grid("#theGrid", makeDataProvider() as any, columns, options);

    // Hook up the cache.
    global_grid.onViewportChanged.subscribe(function (e, args) {
        var vp = global_grid.getViewport();
        global_cache.ensureData({ startRow: vp.top, endRow: vp.bottom });
    });

    setSortColumn();
    global_grid.resizeCanvas();
    global_grid.onViewportChanged.notify();
}

function initRankings(): void {
    initializeEventListeners();

    // Make sure that selector state is provided for each entry in history.
    if (history.state !== null) {
        // The page was loaded by navigating backwards from an external page.
        restoreSelectionState(history.state);
    } else {
        // This is the first load of this page, navigating forwards:
        // stash the current selection state in the history.
        history.replaceState(saveSelectionState(), "", undefined);
    }

    // Hook up the SlickGrid.
    global_cache = makeRemoteCache(selection_to_path(), true);
    renderGridTable();
    renderSelectedFilters();

    // Hook up the searcher.
    searcher = RankingsSearcher();
    searcher.onSearchFound.subscribe(function (e, next_index: number) {
        const numColumns = global_grid.getColumns().length;
        global_grid.scrollRowToTop(next_index);
        for (let i = 0; i < numColumns; ++i) {
            global_grid.flashCell(next_index, i, 100);
        }
    });
}

export {
    initRankings
}

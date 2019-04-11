// vim: set ts=4 sts=4 sw=4 et:
//
// Implementation of main logic for the Rankings page.

'use strict';

import { RemoteCache, WorkItem, Column } from './remotecache';
import { SearchRankingsResult, SearchWorkItem, RankingsSearcher } from './search';

// Appease the TypeScript compiler.
declare var Slick;

// Variables provided by the server.
declare const initial_data;
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
declare const translation_column_ipfpoints: string;

let global_grid;  // The SlickGrid.
let global_cache;  // The active RemoteCache rendered in the SlickGrid.

// A RemoteCache in line to replace the global_cache, but which hasn't
// had its initial data loaded yet, and is still waiting on an AJAX response.
//
// The pending_cache is swapped to overwrite the global_cache when its onFirstLoad
// event fires, by the event handler. Swapping only when data is available avoids
// flickering. The concept is similar to the double-sided OpenGL framebuffer.
let pending_cache;

// Tells an event handler to not create a new history state.
// Used when navigating backwards/forwards, instead of by changing a selector.
let global_suppress_history_changes: boolean = false;

let searcher;
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
        getItem: function(idx) {
            let entry: (string | number)[] = global_cache.rows[idx];
            if (entry === undefined) {
                return;
            }

            let loc = entry[Column.Country];
            if (entry[Column.Country] && entry[Column.State]) {
                loc = loc + "-" + entry[Column.State];
            }

            let name = '<a class="' + entry[Column.Color] +
                '" href="/u/' + entry[Column.Username] + '">' +
                entry[Column.Name] + '</a>';

            if (entry[Column.Instagram]) {
                name += '<a href="https://www.instagram.com/' + entry[Column.Instagram] +
                        '" class="instagram" rel="noopener" target="_blank">' +
                        '<i class="fa fa-instagram fa-resize"></i></a>';
            }

            if (entry[Column.Vkontakte]) {
                name += '<a href="https://vk.com/' + entry[Column.Vkontakte] +
                        '" class="instagram" rel="noopener" target="_blank">' +
                        '<i class="fa fa-vk fa-resize"></i></a>';
            }

            const date = '<a href="/m/' + entry[Column.Path] + '">' +
                entry[Column.Date] + '</a>';

            return {
                rank: (entry[Column.SortedIndex] as number) + 1,
                name: name,
                fed: entry[Column.Federation],
                date: date,
                loc: loc,
                sex: entry[Column.Sex],
                age: entry[Column.Age],
                equipment: entry[Column.Equipment],
                weightclass: entry[Column.WeightClass],
                bodyweight: entry[Column.Bodyweight],
                squat: entry[Column.Squat],
                bench: entry[Column.Bench],
                deadlift: entry[Column.Deadlift],
                total: entry[Column.Total],
                points: entry[Column.Points],
            };
        }
    }
}

function onResize(evt) {
    global_grid.resizeCanvas();
}

function searchOnEnter(keyevent) {
    // keyCode is deprecated, but non-Firefox-desktop doesn't support key.
    if (keyevent.keyCode === 13 || keyevent.key === "Enter") {
        search();
    }
}

function search() {
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
    let sort = selSort.value;
    if (sort === "by-mcculloch") {
        return translation_column_mcculloch;
    }
    if (sort === "by-glossbrenner") {
        return translation_column_glossbrenner;
    }
    if (sort === "by-ipf-points") {
        return translation_column_ipfpoints;
    }
    return translation_column_wilks;
}

// Returns a string like "/raw/uspa", or the empty string
// for the default selection.
function selection_to_path(): string {
    let url = "";
    if (selEquipment.value !== "raw_wraps") {
        url += "/" + selEquipment.value;
    }
    if (selWeightClass.value !== "all") {
        url += "/" + selWeightClass.value;
    }
    if (selFed.value !== "all") {
        url += "/" + selFed.value;
    }
    if (selSex.value !== "all") {
        url += "/" + selSex.value;
    }
    if (selAgeClass.value !== "all") {
        url += "/" + selAgeClass.value;
    }
    if (selYear.value !== "all") {
        url += "/" + selYear.value;
    }
    if (selEvent.value !== "all") {
        url += "/" + selEvent.value;
    }
    if (selSort.value !== "by-wilks") {
        url += "/" + selSort.value;
    }
    return url;
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
function restoreSelectionState(state) {
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
    let url = path ? ("/rankings" + path) : "/";

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

}

function addSelectorListeners(selector) {
    selector.addEventListener("change", changeSelection);
}

// Used when navigating through history: otherwise navigation
// would add more history events.
function removeSelectorListeners(selector) {
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
    window.onpopstate = function(event) {
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
    });

    // Data loads after the first should let the grid know that new
    // data is available by invalidating the current empty rows.
    cache.onDataLoaded.subscribe(function (e, args: WorkItem) {
        for (var i = args.startRow; i <= args.endRow; ++i) {
            global_grid.invalidateRow(i);
        }
        global_grid.updateRowCount();
        global_grid.render();
    });

    return cache;
}

function onLoad() {
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

    // Check templates/rankings.html.tera.
    const nameWidth = 200;
    const shortWidth = 40;
    const dateWidth = 70;
    const numberWidth = 55;

    function urlformatter(row, cell, value, columnDef, dataContext) {
        return value;
    }

    let columns = [
        {id: "filler", width: 20, minWidth: 20, focusable: false,
            selectable: false, resizable: false},
        {id: "rank", name: translation_column_formulaplace, field: "rank", width: 40},
        {id: "name", name: translation_column_liftername, field: "name", width: nameWidth, formatter: urlformatter},
        {id: "fed", name: translation_column_federation, field: "fed", width: numberWidth},
        {id: "date", name: translation_column_date, field: "date", width: dateWidth, formatter: urlformatter},
        {id: "location", name: translation_column_location, field: "loc", width: dateWidth},
        {id: "sex", name: translation_column_sex, field: "sex", width: shortWidth},
        {id: "age", name: translation_column_age, field: "age", width: shortWidth},
        {id: "equipment", name: translation_column_equipment, field: "equipment", width: shortWidth},
        {id: "weightclass", name: translation_column_weightclass, field: "weightclass", width: numberWidth},
        {id: "bodyweight", name: translation_column_bodyweight, field: "bodyweight", width: numberWidth},
        {id: "squat", name: translation_column_squat, field: "squat", width: numberWidth},
        {id: "bench", name: translation_column_bench, field: "bench", width: numberWidth},
        {id: "deadlift", name: translation_column_deadlift, field: "deadlift", width: numberWidth},
        {id: "total", name: translation_column_total, field: "total", width: numberWidth},
        {id: "points", name: selection_to_points_title(), field: "points", width: numberWidth}
    ];

    let options = {
        enableColumnReorder: false,
        forceSyncScrolling: false,
        forceFitColumns: true,
        rowHeight: 23,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing"
    }

    global_cache = makeRemoteCache(selection_to_path(), true);
    global_grid = new Slick.Grid("#theGrid", makeDataProvider(), columns, options);

    // Hook up the cache.
    global_grid.onViewportChanged.subscribe(function (e, args) {
        var vp = global_grid.getViewport();
        global_cache.ensureData({ startRow: vp.top, endRow: vp.bottom });
    });

    setSortColumn();
    global_grid.resizeCanvas();
    global_grid.onViewportChanged.notify();

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

document.addEventListener("DOMContentLoaded", onLoad);

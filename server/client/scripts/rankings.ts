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
declare const translation_column_formulaplace: String;
declare const translation_column_liftername: String;
declare const translation_column_federation: String;
declare const translation_column_date: String;
declare const translation_column_location: String;
declare const translation_column_sex: String;
declare const translation_column_age: String;
declare const translation_column_equipment: String;
declare const translation_column_weightclass: String;
declare const translation_column_bodyweight: String;
declare const translation_column_squat: String;
declare const translation_column_bench: String;
declare const translation_column_deadlift: String;
declare const translation_column_total: String;
declare const translation_column_points: String;

let global_grid;
let global_data;

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


function makeDataProvider(cache) {
    return {
        getLength: function() { return cache.length; },
        getItem: function(idx) {
            let entry: (string | number)[] = cache.rows[idx];
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

            if (entry[Column.Flair] === "bbbc") {
                name += ' <img src="https://raw.githubusercontent.com/sstangl/openpowerlifting-static/gh-pages/bbbc.png" class="bosslogo"/>';
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
    searcher.search({query: query, startRow: startRow});
    searchInfo.laststr = query;
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

// When selectors are changed, the URL in the address bar should
// change to match.
function reload() {
    let path = selection_to_path();

    if (path === "") {
        window.location.href = "/";
    } else {
        window.location.href = "/rankings" + path;
    }
}

function addSelectorListeners(selector) {
    selector.addEventListener("change", reload);
}

function addEventListeners() {
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

    addSelectorListeners(selEquipment);
    addSelectorListeners(selWeightClass);
    addSelectorListeners(selFed);
    addSelectorListeners(selAgeClass);
    addSelectorListeners(selYear);
    addSelectorListeners(selSex);
    addSelectorListeners(selEvent);
    addSelectorListeners(selSort);

    searchField.addEventListener("keypress", searchOnEnter, false);
    searchButton.addEventListener("click", search, false);

    window.addEventListener("resize", onResize, false);
}

function onLoad() {
    addEventListeners();

    // The server can pass initial data to the client.
    // Check templates/rankings.html.tera.
    if (initial_data) {
        global_data = initial_data;
    } else {
        console.log("Failed to initialize data.");
    }

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
        {id: "points", name: translation_column_points, field: "points", width: numberWidth}
    ];

    let options = {
        enableColumnReorder: false,
        forceSyncScrolling: false,
        forceFitColumns: true,
        rowHeight: 23,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing"
    }

    const langSelect = document.getElementById("langselect") as HTMLSelectElement;
    const unitSelect = document.getElementById("weightunits") as HTMLSelectElement;

    const language = langSelect.value;
    const units = unitSelect.value;

    let cache = RemoteCache("TESTING", initial_data, selection_to_path(), language, units);
    global_grid = new Slick.Grid("#theGrid", makeDataProvider(cache), columns, options);

    // Hook up the cache.
    global_grid.onViewportChanged.subscribe(function (e, args) {
        var vp = global_grid.getViewport();
        cache.ensureData({ startRow: vp.top, endRow: vp.bottom });
    });
    cache.onDataLoaded.subscribe(function (e, args: WorkItem) {
        for (var i = args.startRow; i <= args.endRow; ++i) {
            global_grid.invalidateRow(i);
        }
        global_grid.updateRowCount();
        global_grid.render();
    });

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
    global_grid.resizeCanvas();
    global_grid.onViewportChanged.notify();

    // Hook up the searcher.
    searcher = RankingsSearcher(selection_to_path());
    searcher.onSearchFound.subscribe(function (e, next_index: number) {
        const numColumns = global_grid.getColumns().length;
        global_grid.scrollRowToTop(next_index);
        for (let i = 0; i < numColumns; ++i) {
            global_grid.flashCell(next_index, i, 100);
        }
    });
}

document.addEventListener("DOMContentLoaded", onLoad);

let global_grid;
let global_data;


let selEquipment;
let selWeightClass;
let selYear;
let selSex;


function makeDataProvider() {
    return {
        getLength: function() { return window.global_data.length; },
        getItem: function(idx) {
            let entry = window.global_data[idx];

            let loc = entry.country;
            if (entry.country && entry.state) {
                loc = loc + "-" + entry.state;
            }

            let name = '<a class="' + entry.color +
                       '" href="/u/' + entry.username + '">' + entry.name + '</a>';

            if (entry.instagram) {
                name += '<a href="https://www.instagram.com/' + entry.instagram +
                        '" class="instagram" target="_blank">' +
                        '<i class="fa fa-instagram fa-resize"></i></a>';
            }

            if (entry.vkontakte) {
                name += '<a href="https://vk.com/' + entry.vkontakte +
                        '" class="instagram" target="_blank">' +
                        '<i class="fa fa-vk fa-resize"></i></a>';
            }


            return {
                rank: entry.sorted_index + 1,
                name: name,
                fed: entry.federation,
                date: '<a href="/m/' + entry.path + '">' + entry.date + '</a>',
                loc: loc,
                sex: entry.sex,
                age: entry.age,
                equipment: entry.equipment,
                weightclass: entry.weightclass,
                bodyweight: entry.bodyweight,
                squat: entry.squat,
                bench: entry.bench,
                deadlift: entry.deadlift,
                total: entry.total,
                wilks: entry.wilks,
            };
        }
    }
}

function onResize(evt) {
    global_grid.resizeCanvas();
}

// When selectors are changed, the URL in the address bar should
// change to match.
function reload() {
    let url = "/rankings";
    let specific = false;

    if (selEquipment.value !== "raw_wraps") {
        url += "/" + selEquipment.value;
        specific = true;
    }
    if (selWeightClass.value !== "all") {
        url += "/" + selWeightClass.value;
        specific = true;
    }
    if (selSex.value !== "all") {
        url += "/" + selSex.value;
        specific = true;
    }
    if (selYear.value !== "all") {
        url += "/" + selYear.value;
        specific = true;
    }

    if (specific === true) {
        window.location = url;
    } else {
        window.location = "/";
    }
}

function addSelectorListeners(selector) {
    selector.addEventListener("change", reload);
}

function addEventListeners() {
    selEquipment = document.getElementById("equipmentselect");
    selWeightClass = document.getElementById("weightclassselect");
    selYear = document.getElementById("yearselect");
    selSex = document.getElementById("sexselect");

    addSelectorListeners(selEquipment);
    addSelectorListeners(selWeightClass);
    addSelectorListeners(selYear);
    addSelectorListeners(selSex);

    window.addEventListener("resize", onResize, false);
}

function onLoad() {
    addEventListeners();

    // The server can pass initial data to the client.
    // Check templates/rankings.html.tera.
    if (initial_data) {
        window.global_data = initial_data;
    } else {
        console.log("Failed to initialize data.");
    }

    const nameWidth = 200;
    const shortWidth = 40;
    const dateWidth = 80;
    const numberWidth = 56;

    function urlformatter(row, cell, value, columnDef, dataContext) {
        return value;
    }

    let columns = [
        {id: "filler", width: 20, minWidth: 20, focusable: false,
            selectable: false, resizable: false},
        {id: "rank", name: translation_column_formulaplace, field: "rank", width: numberWidth},
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
        {id: "wilks", name: translation_column_wilks, field: "wilks", width: numberWidth}
    ];

    let options = {
        enableColumnReorder: false,
        forceSyncScrolling: false,
        forceFitColumns: true,
        rowHeight: 23,
        topPanelHeight: 23,
        cellFlashingCssClass: "searchflashing"
    }

    global_grid = new Slick.Grid("#theGrid", makeDataProvider(), columns, options);
    global_grid.setSortColumn("wilks", false); // Sort descending.
    global_grid.resizeCanvas();
}

document.addEventListener("DOMContentLoaded", onLoad);

// vim: set ts=4 sts=4 sw=4 et:
//
// Data store for the SlickGrid on the Rankings page.
//
// The data is initially seeded through the HTML request,
// with further updates provided by AJAX calls, dispatched
// on scroll.

'use strict';

declare var Slick;

let langselect;
let weightunits;

// Column mapping for the server rankings JSON.
// This should match with the serialization of the JsEntryRow
// in the Rust server source.
export const enum Column {
    SortedIndex,
    Name,
    Username,
    Instagram,
    Vkontakte,
    Color,
    Federation,
    Date,
    Country,
    State,
    Path,
    Sex,
    Equipment,
    Age,
    Division,
    Bodyweight,
    WeightClass,
    Squat,
    Bench,
    Deadlift,
    Total,
    Wilks,
}

// Parameters for a possible remote request.
export interface WorkItem {
    startRow: number;  // Inclusive.
    endRow: number;  // Inclusive.
}

// Data that should be remembered about an AJAX request.
interface AjaxRequest {
    handle: XMLHttpRequest;
    item: WorkItem;
}

// Creates a data provider for the SlickGrid that understands how to
// make AJAX requests to a JSON endpoint to gather missing data.
export function RemoteCache(
    magicVersion: string,  // Unique checksum of database, for versioning.
    initialJson,  // Initial data, in the HTML to avoid a round-trip.
    selection: string,  // Selection string, for forming AJAX URLs.
    language: string,  // Language code, for including in AJAX requests.
    units: string  // Units, for including in AJAX requests.
) {
    const REQUEST_LENGTH = 100;  // Batch this many rows in one request.
    const AJAX_TIMEOUT = 50;  // Milliseconds before making AJAX request.

    let rows: ((string | number)[])[] = [];  // Array of cached row data.
    const length: number = initialJson.total_length;

    let activeTimeout: number = null;  // Timeout before making AJAX request.
    let activeAjaxRequest: AjaxRequest = null;

    // The viewport can update while the AJAX request is still ongoing.
    // The request is still allowed to finish, but it might have to
    // make another request with the pendingItem upon completion.
    let pendingItem: WorkItem = null;

    const onDataLoading = new Slick.Event();  // Data is currently loading.
    const onDataLoaded = new Slick.Event();  // Data has finished loading.

    // Single definition point for defining the URL endpoint.
    function makeApiUrl(item: WorkItem): string {
        const startRow = Math.max(item.startRow, 0);
        const endRow = Math.min(item.endRow, length - 1);
        return `/api/rankings${selection}?start=${startRow}&end=${endRow}&lang=${language}&units=${units}`;
    }

    // Given more JSON data, add it to the rows array.
    function addRows(json): void {
        for (let i = 0; i < json.rows.length; ++i) {
            const source: (string | number)[] = json.rows[i];
            const index = source[Column.SortedIndex] as number;
            rows[index] = source;
        }
    }

    // Cancels any pending AJAX calls, but does not cancel ongoing ones.
    function cancelPendingRequests() {
        if (activeTimeout !== null) {
            clearTimeout(activeTimeout);
            activeTimeout = null;
        }
        pendingItem = null;
    }

    // Ask for more data than is actually needed to cut down on the
    // number of requests.
    function maximizeItem(item: WorkItem): WorkItem {
        let startRow = item.startRow;
        let endRow = item.endRow;

        while (endRow - startRow + 1 < REQUEST_LENGTH
               && endRow < length - 1
               && rows[endRow] === undefined)
        {
            ++endRow;
        }

        // Now try the other direction: this handles the scrolling-up case.
        while (endRow - startRow + 1 < REQUEST_LENGTH
               && startRow > 0
               && rows[startRow] === undefined)
        {
            --startRow;
        }

        return { startRow: startRow, endRow: endRow };
    }

    // Function called by the timeout handler.
    function makeAjaxRequest(): void {
        // This function was called by the timeout handler.
        activeTimeout = null;

        // Sanity checking: if there's already an active AJAX request,
        // it should just be allowed to finish. When it finishes,
        // it will automatically queue the next request.
        if (activeAjaxRequest !== null) {
            return;
        }

        // Sanity checking: we have to arrive here with some work to do.
        if (pendingItem === null) {
            return;
        }

        // Pop the pendingItem.
        // Ask for as much data in the single request as possible.
        const item = maximizeItem(pendingItem);
        pendingItem = null;

        let handle = new XMLHttpRequest();
        handle.open("GET", makeApiUrl(item));
        handle.responseType = "json";
        handle.addEventListener("load", function(e) {
            addRows(activeAjaxRequest.handle.response);
            activeAjaxRequest = null;
            onDataLoaded.notify(item);

            // Ensure any pendingItem is resolved if necessary.
            if (pendingItem !== null && activeTimeout === null) {
                const item = pendingItem;
                pendingItem = null;
                ensureData(item);
            }
        });
        handle.addEventListener("error", function(e) {
            console.log(e);
            activeAjaxRequest = null;
            onDataLoaded.notify(item);
        });

        activeAjaxRequest = { handle: handle, item: item };
        activeAjaxRequest.handle.send();

        // Notify that we've started loading some data.
        onDataLoading.notify(item);
    }

    // Check that the data in the given inclusive range is loaded.
    // If not, arrange an AJAX request to load it.
    // This is the main function that does work.
    function ensureData(item: WorkItem): void {
        // Ensure sane bounds.
        let startRow = Math.max(item.startRow, 0);
        let endRow = Math.min(item.endRow, length - 1);

        // Find the closest row that hasn't been filled in.
        while (startRow < endRow && rows[startRow] !== undefined) {
            ++startRow;
        }

        // Find the farthest row that hasn't been filled in.
        while (startRow < endRow && rows[endRow] !== undefined) {
            --endRow;
        }

        // If everything has already been filled in, we're done!
        if (startRow > endRow || ((startRow == endRow) && rows[startRow] !== undefined)) {
            onDataLoaded.notify({startRow: startRow, endRow: endRow});
            return;
        }

        // Ensure that an AJAX request will be made.
        pendingItem = { startRow: startRow, endRow: endRow };
        if (activeTimeout === null) {
            activeTimeout = setTimeout(makeAjaxRequest, AJAX_TIMEOUT);
        }
    }

    // Initialization.
    addRows(initialJson);

    return {
        // Properties.
        "rows": rows,
        "length": length,

        // Methods.
        "ensureData": ensureData,

        // Events.
        "onDataLoading": onDataLoading,
        "onDataLoaded": onDataLoaded
    };
}

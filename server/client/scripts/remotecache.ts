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

// Creates a data provider for the SlickGrid that understands how to
// make AJAX requests to a JSON endpoint to gather missing data.
//
// magicVersion is a magic string provided by the server on first load.
// If the server is restarted with new data, our existing data may be invalid,
// so we need to detect if that happened.
// That's done by comparing magicVersion strings.
// This also has the nice benefit of working around AJAX cache issues.
//
// length is provided by the server on first load: we need to know how long
// to make the scrollbar.
export function RemoteCache(magicVersion: string, initial_json, selection: string, language: string, units: string) {
    // Try to make requests of this many rows.
    const REQUEST_LENGTH = 100;
    // How many milliseconds to wait before trying to make an AJAX call.
    const AJAXTIMEOUT = 50;

    // The actual cache.
    let rows = [];
    for (var i = 0; i < initial_json.rows.length; ++i) {
        let source = initial_json.rows[i];
        rows[parseInt(source.sorted_index)] = source;
    }
    const length: number = initial_json.total_length;

    // Private variables for internal state.
    let activeTimeout = null;
    let activeAjaxRequest = null;

    // Events.
    const onDataLoading = new Slick.Event();
    const onDataLoaded = new Slick.Event();

    // Single definition point for defining the URL endpoint.
    function makeApiUrl(startRow: number, endRow: number): string {
        startRow = Math.max(startRow, 0);
        endRow = Math.min(endRow, length - 1);
        return `/api/rankings${selection}?start=${startRow}&end=${endRow}&lang=${language}&units=${units}`;
    }

    function clearActiveRequests() {
        if (activeAjaxRequest !== null) {
            activeAjaxRequest.abort();
            activeAjaxRequest = null;
        }
        if (activeTimeout !== null) {
            clearTimeout(activeTimeout);
            activeTimeout = null;
        }
    }

    // Check that the data in the given inclusive range is loaded.
    // If not, arrange an AJAX request to load it.
    // This is the main function that does work.
    function ensureData(startRow: number, endRow: number): void {
        // The viewport has moved, so clear out any active requests.
        clearActiveRequests();

        // Ensure sane bounds.
        startRow = Math.max(startRow, 0);
        endRow = Math.min(endRow, length - 1);

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

        // Otherwise, we're making a request.
        // Ask for more data than is actually needed to cut down on the
        // number of requests.
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

        // Set a timeout to make an AJAX request.
        activeTimeout = setTimeout(function() {
            // Notify that we've started loading some data.
            onDataLoading.notify({startRow: startRow, endRow: endRow});

            activeAjaxRequest = new XMLHttpRequest();
            activeAjaxRequest.open("GET", makeApiUrl(startRow, endRow));
            activeAjaxRequest.responseType = "json";
            activeAjaxRequest.onload = function(e) {
                let json = activeAjaxRequest.response;
                for (var i = 0; i < json.rows.length; ++i) {
                    let source = json.rows[i];
                    rows[parseInt(source.sorted_index)] = source;
                }
                activeAjaxRequest = null;
                // Notify that the data loaded successfully.
                onDataLoaded.notify({startRow: startRow, endRow: endRow});
            };
            activeAjaxRequest.send();
        }, AJAXTIMEOUT);
    }

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

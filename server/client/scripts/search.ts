// vim: set ts=4 sts=4 sw=4 et:
//
// Interacts with the /api/search/ server endpoint.

'use strict';

declare var Slick;

export interface SearchRankingsResult {
    next_index: (number | null);
}

// Parameters for a possible remote request.
export interface SearchWorkItem {
    query: string;
    startRow: number;  // Inclusive.
}

// Creates a function that manages AJAX requests to the rankings search endpoint.
export function RankingsSearcher(
    selection: string,  // For forming URLs.
) {
    const AJAX_TIMEOUT = 50;  // Milliseconds before making AJAX request.

    let activeTimeout: number = null;  // Timeout before making AJAX request.
    let activeAjaxRequest: XMLHttpRequest = null;

    let pendingItem: SearchWorkItem = null;

    const onSearchFound = new Slick.Event();
    const onSearchNotFound = new Slick.Event();

    // Creates a URL for the rankings search endpoint.
    function makeApiUrl(item: SearchWorkItem): string {
        // Remove some characters that will cause malformed URLs.
        const query = item.query.replace(/[&\/\\#,+()$~%.'":*?<>{}]/g, '_');
        const startRow = Math.max(item.startRow, 0);
        return `/api/search/rankings${selection}?q=${query}&start=${startRow}`;
    }

    // Cancels any pending or active AJAX calls.
    function cancelAllRequests(): void {
        if (activeAjaxRequest !== null) {
            activeAjaxRequest.abort();
            activeAjaxRequest = null;
        }
        if (activeTimeout !== null) {
            clearTimeout(activeTimeout);
            activeTimeout = null;
        }
    }

    // Function called by the timeout handler.
    function makeAjaxRequest(): void {
        // This function was called by the timeout handler.
        activeTimeout = null;

        // Pop the pendingItem.
        const item = pendingItem;
        pendingItem = null;

        let handle = new XMLHttpRequest();
        handle.open("GET", makeApiUrl(item));
        handle.responseType = "json";
        handle.addEventListener("load", function(e) {
            const index = activeAjaxRequest.response.next_index;
            activeAjaxRequest = null;

            if (index !== null) {
                onSearchFound.notify(index);
            } else {
                onSearchNotFound.notify();
            }
        });
        handle.addEventListener("error", function(e) {
            console.log(e);
            activeAjaxRequest = null;
            onSearchNotFound.notify();
        });

        activeAjaxRequest = handle;
        activeAjaxRequest.send();
    }

    function search(item: SearchWorkItem): void {
        cancelAllRequests();
        pendingItem = item;
        activeTimeout = setTimeout(makeAjaxRequest, AJAX_TIMEOUT);
    }

    return {
        // Methods.
        "search": search,

        // Events.
        "onSearchFound": onSearchFound,
        "onSearchNotFound": onSearchNotFound
    };
}

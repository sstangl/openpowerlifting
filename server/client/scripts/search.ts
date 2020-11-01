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

// Interacts with the /api/search/ server endpoint.

'use strict';

declare var Slick;

// Provided by the rankings template.
declare const urlprefix: string;

export interface SearchRankingsResult {
    next_index: (number | null);
}

// Parameters for a possible remote request.
export interface SearchWorkItem {
    path: string;  // The URL path.
    query: string;  // The search query.
    startRow: number;  // Inclusive.
}

// Creates a function that manages AJAX requests to the rankings search endpoint.
export function RankingsSearcher() {
    const AJAX_TIMEOUT = 50;  // Milliseconds before making AJAX request.

    let activeTimeout: number | null = null;  // Timeout before making AJAX request.
    let activeAjaxRequest: XMLHttpRequest | null = null;

    let pendingItem: SearchWorkItem | null = null;

    const onSearchFound = new Slick.Event();
    const onSearchNotFound = new Slick.Event();

    // Creates a URL for the rankings search endpoint.
    function makeApiUrl(item: SearchWorkItem): string {
        // Remove some characters that will cause malformed URLs.
        const query = item.query.replace(/[&\/\\#,+()$~%.'":*?<>{}]/g, '_');
        const startRow = Math.max(item.startRow, 0);
        return `${urlprefix}api/search/rankings${item.path}?q=${query}&start=${startRow}`;
    }

    // Cancels any pending or active AJAX calls.
    function terminateAllRequests(): void {
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

        // Can't happen: appeases TypeScript.
        if (pendingItem === null) {
            return;
        }

        // Pop the pendingItem.
        const item = pendingItem;
        pendingItem = null;

        let handle = new XMLHttpRequest();
        handle.open("GET", makeApiUrl(item));
        handle.responseType = "json";
        handle.addEventListener("load", function(e) {
            // Can't happen, but just to appease TypeScript.
            if (activeAjaxRequest === null) {
                return;
            }

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
        terminateAllRequests();
        pendingItem = item;
        activeTimeout = window.setTimeout(makeAjaxRequest, AJAX_TIMEOUT);
    }

    return {
        // Methods.
        "search": search,
        "terminateAllRequests": terminateAllRequests,

        // Events.
        "onSearchFound": onSearchFound,
        "onSearchNotFound": onSearchNotFound
    };
}

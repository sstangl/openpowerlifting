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

// Common entrance point to initialization code, valid on every page.

import { initLocaleEventListeners } from "./locale";
import { initMobileFooter, isMobile } from "./mobile";

import { initMeet } from "./meet";
import { initMeetList } from "./meetlist";
import { initRankings } from "./rankings";
import { initRecords } from "./records";

// Pages that have unique scripts supply a PAGE_KIND global in their template.
enum PageKind {
    Meet = "MEET",
    MeetList = "MEETLIST",
    Rankings = "RANKINGS",
    Records = "RECORDS",
}

// Optionally provided by each template to allow main() to select the proper
// entrance point for the current page.
declare const PAGE_KIND: PageKind | undefined;

function main() {
    // Initializes the "Change Language" and "Change Units" selectors, if present.
    initLocaleEventListeners();

    // If on mobile, add handlers for the navigational footer.
    if (isMobile()) {
        initMobileFooter();
    }

    if (typeof PAGE_KIND === "string") { // Necessary to handle the undefined case.
        switch (PAGE_KIND) {
            case PageKind.Meet: initMeet(); break;
            case PageKind.MeetList: initMeetList(); break;
            case PageKind.Rankings: initRankings(); break;
            case PageKind.Records: initRecords(); break;
            default: break; // Some pages (like the FAQ) have no scripts attached.
        }
    }
}

document.addEventListener("DOMContentLoaded", main);

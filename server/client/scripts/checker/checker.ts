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

// Interfaces with the in-server checker, mounted at /dev.

'use strict';

import { Csv } from "./csv";

let checkButton: HTMLButtonElement;
let meetTextArea: HTMLTextAreaElement;
let entriesTextArea: HTMLTextAreaElement;

// Simple way to report errors until we make something pretty and colorful.
let ioErrorPre: HTMLElement;
let meetErrorPre: HTMLElement;
let entriesErrorPre: HTMLElement;

// The checker::Message type is an object that distinguishes between
// "Error" type messages and "Warning" type messages.
//
// It's defined in Rust, in checker/src/lib.rs.
interface Message {
  Error?: string;
  Warning?: string;
};

// Converts a Message object to a simple, uncolored string, for the moment.
function msg2str(msg: Message): string {
    if (msg.hasOwnProperty("Error")) {
        return "Error: " + msg["Error"];
    }
    return "Warning: " + msg["Warning"];
}

function runChecker(): void {
    let handle = new XMLHttpRequest();
    handle.open("POST", "/dev/checker");
    handle.responseType = "text";
    handle.setRequestHeader("Content-Type", "application/json;charset=UTF-8");

    // Clear the error reporting sections.
    // This causes relayout and therefore flickering, but makes it clear
    // when the server misses a response.
    ioErrorPre.innerText = null;
    meetErrorPre.innerText = null;
    entriesErrorPre.innerText = null;

    handle.onreadystatechange = function() {
        if (this.readyState === XMLHttpRequest.DONE && this.status === 200) {
            // Get the CheckerOutput (defined in server/src/pages/context.rs).
            let output = JSON.parse(this.responseText);

            // I/O errors take precedence.
            if (output.io_error !== null) {
                ioErrorPre.innerText = output.io_error;
                return;
            }

            if (output.meet_messages.length > 0) {
                meetErrorPre.innerText = output.meet_messages.map(msg2str).join("\n");
            } else {
                meetErrorPre.innerText = "Pass! :)";
            }

            if (output.entries_messages.length > 0) {
                entriesErrorPre.innerText = output.entries_messages.map(msg2str).join("\n");
            } else if (output.meet_messages.length === 0) {
                // The entries.csv is only checked if the meet.csv passes.
                entriesErrorPre.innerText = "Pass! :)";
            }
        }
    };

    // Serialization of server/src/pages/checker.rs' CheckerInput.
    let checkerInput = {
        "meet": meetTextArea.value,
        "entries": entriesTextArea.value
    };

    handle.send(JSON.stringify(checkerInput));
}

// Spreadsheet applications use tab separators when copy/pasting.
// This translates "\t" to "," for use in a timeout created by an onPaste handler.
function replaceTabs(elem: HTMLTextAreaElement) {
    elem.value = elem.value.replace(/\t/g, ",");
}

function initializeEventListeners() {
    checkButton = document.getElementById("checkButton") as HTMLButtonElement;
    meetTextArea = document.getElementById("meetTextArea") as HTMLTextAreaElement;
    entriesTextArea = document.getElementById("entriesTextArea") as HTMLTextAreaElement;

    ioErrorPre = document.getElementById("ioErrorPre");
    meetErrorPre = document.getElementById("meetErrorPre");
    entriesErrorPre = document.getElementById("entriesErrorPre");

    checkButton.addEventListener("click", runChecker, false);

    // Allow pasting from spreadsheet software by converting tabs to commas.
    meetTextArea.addEventListener("paste", e => {
        setTimeout(function() { replaceTabs(e.target as HTMLTextAreaElement) }, 0);
    }, false);
    entriesTextArea.addEventListener("paste", e => {
        setTimeout(function() { replaceTabs(e.target as HTMLTextAreaElement) }, 0);
    }, false);
}

function checkerOnLoad() {
    initializeEventListeners();
}

document.addEventListener("DOMContentLoaded", checkerOnLoad);

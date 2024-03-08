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

import { csvToKg } from "./functions/tokg";
import { csvCalcPlace } from "./functions/calc-place";
import { csvStandardiseCountries } from "./functions/standardise-countries";
import { csvRound } from "./functions/round-kg";

let checkButton: HTMLButtonElement;
let toKgButton: HTMLButtonElement;
let calcPlaceButton: HTMLButtonElement;
let standardiseCountriesButton: HTMLButtonElement;
let roundKgButton: HTMLButtonElement;

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
  severity: string;
  text: string;
};

// Converts a Message object to a simple, uncolored string, for the moment.
function msg2str(msg: Message): string {
    if (msg.severity === "Warning") {
        return "<b><font color=\"#fdb93e\">Warning: " + msg.text + "</font></b>";
    }
    return "<b><font color=\"#fb3640\">Error: " + msg.text + "</font></b>";
}

function runChecker(): void {
    let handle = new XMLHttpRequest();
    handle.open("POST", "/dev/checker");
    handle.responseType = "text";
    handle.setRequestHeader("Content-Type", "application/json;charset=UTF-8");

    // Clear the error reporting sections.
    // This causes relayout and therefore flickering, but makes it clear
    // when the server misses a response.
    ioErrorPre.innerText = "";
    meetErrorPre.innerText = "";
    entriesErrorPre.innerText = "";

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
                meetErrorPre.innerHTML = output.meet_messages.map(msg2str).join("\n");
            } else {
                meetErrorPre.innerText = "Pass! :)";
            }

            if (output.entries_messages.length > 0) {
                entriesErrorPre.innerHTML = output.entries_messages.map(msg2str).join("\n");
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
    toKgButton = document.getElementById("toKgButton") as HTMLButtonElement;
    calcPlaceButton = document.getElementById("calcPlaceButton") as HTMLButtonElement;
    standardiseCountriesButton = document.getElementById("standardiseCountriesButton") as HTMLButtonElement;
    roundKgButton = document.getElementById("roundKgButton") as HTMLButtonElement;

    meetTextArea = document.getElementById("meetTextArea") as HTMLTextAreaElement;
    entriesTextArea = document.getElementById("entriesTextArea") as HTMLTextAreaElement;

    ioErrorPre = document.getElementById("ioErrorPre") as HTMLElement;
    meetErrorPre = document.getElementById("meetErrorPre") as HTMLElement;
    entriesErrorPre = document.getElementById("entriesErrorPre") as HTMLElement;

    checkButton.addEventListener("click", runChecker, false);

    toKgButton.addEventListener("click", function () {
        // Parse the entries text field as CSV.
        let csv = new Csv();
        let csvOrError = csv.fromString(entriesTextArea.value);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Perform conversion.
        csvOrError = csvToKg(csv);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Render back.
        entriesTextArea.value = csv.toString();
    }, false);

    calcPlaceButton.addEventListener("click", function () {
        // Parse the entries text field as CSV.
        let csv = new Csv();
        let csvOrError = csv.fromString(entriesTextArea.value);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Perform conversion.
        csvOrError = csvCalcPlace(csv);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Render back.
        entriesTextArea.value = csv.toString();
    }, false);

    standardiseCountriesButton.addEventListener("click", function () {
        // Parse the entries text field as CSV.
        let csv = new Csv();
        let csvOrError = csv.fromString(entriesTextArea.value);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Perform conversion.
        csvOrError = csvStandardiseCountries(csv);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Render back.
        entriesTextArea.value = csv.toString();
    }, false);

    roundKgButton.addEventListener("click", function () {
        // Parse the entries text field as CSV.
        let csv = new Csv();
        let csvOrError = csv.fromString(entriesTextArea.value);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Perform conversion.
        csvOrError = csvRound(csv);
        if (typeof csvOrError === "string") {
            entriesErrorPre.innerText = csvOrError;
            return;
        }
        csv = csvOrError;

        // Render back.
        entriesTextArea.value = csv.toString();
    }, false);

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

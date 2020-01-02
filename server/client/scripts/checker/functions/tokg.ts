// vim: set ts=2 sts=2 sw=2 et:
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

// Converts all CSV columns with "LBS" in them to kilograms, "Kg".
// Weightclasses are rounded to their kg class.

'use strict';

import { Csv, csvString } from "../csv";


// Converts a weight in pounds to Kg, as a string with a maximum of 2 decimal places.
const lbsToKg = (lbs: number): string => {
  const kg = lbs / 2.20462262;
  return new Intl.NumberFormat("en-US", { useGrouping: false, maximumFractionDigits: 2}).format(kg);
};


// Converts a weightclass in pounds to Kg, rounding to the nearest 0.5kg.
// Returns undefined if Number-parsing failed.
const weightClassToKg = (wtcls: string, csv: Csv, rowIndex: number): string | undefined => {
  if (wtcls === "") return "";

  // Handle "SHW" specially with reference to the "Sex" column.
  if (wtcls === "SHW") {
    const sexIndex: number = csv.index("Sex"); // Guaranteed to exist by csvToKg().
    const sex: string = csv.rows[rowIndex][sexIndex];
    if (sex === "F") return "90+";
    return "140+";
  }

  // If the class is like "198+", remove the "+" and remember that it was present.
  let isSuper: boolean = wtcls.endsWith('+');
  if (isSuper) {
    wtcls = wtcls.replace("+", "");
  }

  let asNumber = Number(wtcls);
  if (isNaN(asNumber)) return undefined;

  // Convert to Kg.
  let kg = asNumber / 2.20462262;
  // Round to the nearest 0.5 (which is equivalent to rounding kg*2 to an integer).
  kg = Math.round(kg * 2) / 2;

  let kgstr = new Intl.NumberFormat("en-US", { useGrouping: false, maximumFractionDigits: 1}).format(kg);

  // Make some common corrections due to LBS already being heavily rounded.
  // Checked in Python using: `print(round(x / 2.20462262 * 2) / 2)`
  switch (kgstr) {
    // Traditional weightclasses.
    case "47.5": kgstr = "48"; break;
    case "51.5": kgstr = "52"; break;
    case "67": kgstr = "67.5"; break;
    case "82": kgstr = "82.5"; break;
    case "124.5": kgstr = "125"; break;
    case "139.5": kgstr = "140"; break;

    // IPF Men.
    case "52.5": kgstr = "53"; break;
    case "119.5": kgstr = "120"; break;

    // IPF Women.
    case "42.5": kgstr = "43"; break;
    case "46.5": kgstr = "47"; break;
    case "56.5": kgstr = "57"; break;
    case "62.5": kgstr = "63"; break;
    case "71.5": kgstr = "72"; break;

    default: break;
  };

  if (isSuper) kgstr += "+";
  return kgstr;
};


// Creates a new Csv file where LBS columns are converted to Kg.
//
// On success, returns the new Csv.
// On failure, returns a string describing the error.
export const csvToKg = (source: Csv): Csv | string => {
  const csv = source.shallowClone();

  // Check for prerequisites.
  if (csv.index("Sex") < 0) {
    return "Converting to Kg requires a 'Sex' column to handle SHW WeightClassLBS";
  }

  for (let i = 0; i < csv.fieldnames.length; ++i) {
    const lower = csv.fieldnames[i].toLowerCase();
    if (lower.includes("lbs") === false) {
      continue;
    }

    // Convert all the rows.
    if (lower.includes("weightclass")) {
      // Weightclasses get special handling.
      for (let j = 0; j < csv.rows.length; ++j) {
        const value: string = csv.rows[j][i];
        const valueOrError = weightClassToKg(value, source, j);
        if (valueOrError === undefined) {
          return `Error in '${csv.fieldnames[i]}' row ${j+1}: '${value}' not a number`;
        }
        csv.rows[j][i] = csvString(valueOrError);
      }
    } else {
      // Generic weight column.
      for (let j = 0; j < csv.rows.length; ++j) {
        const value: string = csv.rows[j][i];
        const asNumber = Number(value);
        if (isNaN(asNumber)) {
          return `Error in '${csv.fieldnames[i]}' row ${j+1}: '${value}' not a number`;
        }
        csv.rows[j][i] = csvString(lbsToKg(asNumber));
      }
    }

    // Replace the first occurence of "LBS" (case-insensitive) with "Kg".
    csv.fieldnames[i] = csv.fieldnames[i].replace(/lbs/i, "Kg");
  }

  return csv;
};

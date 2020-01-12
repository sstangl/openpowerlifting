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

// Automatically (re)calculates the Place column.

'use strict';

import { Csv, csvString } from "../csv";


// Get a unique string for the lifter's category.
// Lifters in the same category compete against each other for placings.
const getCategory = (csv: Csv, row: ReadonlyArray<string>): string => {
  const divIndex: number = csv.index("Division");

  const division: string = divIndex >= 0 ? row[divIndex].toLowerCase() : "open";
  const equipment: string = row[csv.index("Equipment")];
  const sex: string = row[csv.index("Sex")];
  const event: string = row[csv.index("Event")];
  const weightclass: string = row[csv.index("WeightClassKg")];

  return [division, equipment, sex, event, weightclass].join(",");
};


// Creates a new Csv file with the Place recalculated.
//
// The algorithm used is O(n), using a hashmap.
//
// On success, returns the new Csv.
// On failure, returns a string describing the error.
export const csvCalcPlace = (source: Csv): Csv | string => {
  const csv = source.shallowClone();

  // Check that all required columns are present.
  if (csv.index("Equipment") < 0) return "Missing column 'Equipment'";
  if (csv.index("Sex") < 0) return "Missing column 'Sex'";
  if (csv.index("Event") < 0) return "Missing column 'Event'";
  if (csv.index("WeightClassKg") < 0) return "Missing column 'WeightClassKg'";
  if (csv.index("TotalKg") < 0) return "Missing column 'TotalKg'";

  // If there's an existing Place column, clear it in place. Otherwise, append a new one.
  if (csv.index("Place") >= 0) {
    const idx = csv.index("Place");
    for (let i = 0; i < csv.rows.length; ++i) {
      // But, preserve Guest if manually specified.
      if (csv.rows[i][idx] !== "G") {
        csv.rows[i][idx] = "";
      }
    }
  } else {
    csv.appendColumn("Place");
  }

  // Gather column indices needed below.
  const placeIndex = csv.index("Place");
  const totalIndex = csv.index("TotalKg");

  // Only use bodyweight to break ties if provided.
  // Otherwise, setting the bodyweight to the total effectively ignores it.
  let bwIndex = totalIndex;
  if (csv.index("BodyweightKg") >= 0) bwIndex = csv.index("BodyweightKg");

  // Group rows in a Map by their category.
  let categories = new Map();
  for (let i = 0; i < csv.rows.length; ++i) {
    // Skip guests: leave them in-place.
    if (csv.rows[i][placeIndex] === "G") {
      continue;
    }

    const category = getCategory(csv, csv.rows[i]);
    if (categories.has(category)) {
      categories.get(category).push(csv.rows[i]);
    } else {
      categories.set(category, [csv.rows[i]]);
    }

    // While we're iterating here, also ensure that TotalKg and BodyweightKg are numbers.
    // This is needed for sorting below.
    if (isNaN(Number(csv.rows[i][totalIndex]))) {
      return `Error in 'TotalKg' row ${i+1}: '${csv.rows[i][totalIndex]}' not a number`;
    }
    if (bwIndex !== totalIndex && isNaN(Number(csv.rows[i][bwIndex]))) {
      return `Error in 'BodyweightKg' row ${i+1}: '${csv.rows[i][bwIndex]}' not a number`;
    }
  }

  // Sort the lifters in a category and assign their Place.
  for (const rows of categories.values()) {
    // By the previous iteration, TotalKg and BodyweightKg are guaranteed to be numbers.
    rows.sort((a, b) => {
      // Higher totals get ordered first.
      const aTotal = Number(a[totalIndex]);
      const bTotal = Number(b[totalIndex]);
      if (aTotal !== bTotal) return bTotal - aTotal;

      // If totals are equal, lower bodyweights get ordered first.
      return Number(a[bwIndex]) - Number(b[bwIndex]);
    });

    let nextPlace = 1; // Ticker for the next place to assign.
    for (let i = 0; i < rows.length; ++i) {
      if (Number(rows[i][totalIndex]) > 0) {
        rows[i][placeIndex] = csvString(nextPlace++);
      } else {
        rows[i][placeIndex] = csvString("DQ");
      }
    }
  }

  return csv;
};

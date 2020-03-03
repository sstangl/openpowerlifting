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

// Rounds lifting Kg columns to the nearest 0.5kg.

'use strict';

import { Csv, csvString } from "../csv";


// Creates a new Csv file with the attempts rounded.
// On success, returns the new Csv.
// On failure, returns a string describing the error.
export const csvRound = (source: Csv): Csv | string => {
  const csv = source.shallowClone();

  const liftingColumns = ["Squat1Kg", "Squat2Kg", "Squat3Kg", "Squat4Kg", "Best3SquatKg",
                          "Bench1Kg", "Bench2Kg", "Bench3Kg", "Bench4Kg", "Best3BenchKg",
                          "Deadlift1Kg", "Deadlift2Kg", "Deadlift3Kg", "Deadlift4Kg", "Best3DeadliftKg", 
                          "TotalKg"];
  const indices = liftingColumns.map(name => csv.index(name));

  for (let ii = 0; ii < csv.rows.length; ++ii) {
    const row = csv.rows[ii];
    for (let jj = 0; jj < indices.length; ++jj) {
        const columnName = liftingColumns[jj];
        const index = indices[jj];

        if (index < 0 || row[index] === '') continue;

        let liftAsNum = Number(row[index]);

        if (isNaN(liftAsNum)) {
          return `Error in '${columnName}' row ${ii+1}: '${csv.rows[ii][index]}' not a number`;
        }

        csv.rows[ii][index] = (0.5*Math.round(2*liftAsNum)).toString()
    }
  }

  return csv;
};

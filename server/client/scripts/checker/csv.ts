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

// Defines a general CSV manipulation class.
// This is a JS port of the Python "oplcsv.py" library used by the OpenPowerlifting
// main data project.

// Makes a string suitable for inclusion in a simple CSV file,
// by deleting all commas and double quotes.
export const csvString = (x?: number | string): string => {
  if (x === undefined) return "";
  let s = String(x);

  // The OpenPowerlifting format uses commas and disallow double-quotes.
  s = s.replace(/,/g, " ");
  s = s.replace(/"/g, " ");

  // The number "0" is also never written out explicitly; the empty string is preferred.
  if (s === "0") return "";

  // Clean up some formatting.
  s = s.replace(/ {2}/g, " ").trim();
  return s;
};

// Returns the in-spreadsheet name of a column. In standard spreadsheet software,
// rows are numeric (1, 2, 3, ...) and columns are alphabetic (A, B, C, ...).
// For errors, we'd like to report the column that's wrong.
//
// The 'index' input is zero-indexed.
export const getSpreadsheetColumnName = (index: number): string => {
  const alphabet: string = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

  // Column name accumulator.
  // It's built by taking repeated mods in base 26: therefore new characters
  // are appended to the left of the accumulator.
  let acc: string = "";
  let n: number = index;

  while (n >= alphabet.length) {
    const mod = n % alphabet.length;
    const remainder = Math.floor(n / alphabet.length);

    acc = alphabet[mod] + acc;

    // The subtraction is because this isn't a pure modulus operator:
    // by analogy to base-10, "AA" is equivalent to "00", and we want to render
    // that.
    n = remainder - 1;
  }

  return alphabet[n] + acc;
};

// A class for managing simple CSV files. Double-quotes and commas are disallowed.
export class Csv {
  fieldnames: Array<string>; // Column names.
  rows: Array<Array<string>>; // Individual rows.

  constructor() {
    this.fieldnames = [];
    this.rows = [];
  }

  shallowClone(): Csv {
    let csv = new Csv();
    csv.fieldnames = this.fieldnames.slice();
    csv.rows = this.rows.slice();
    return csv;
  }

  length(): number {
    return this.rows.length;
  }

  index(name: string): number {
    return this.fieldnames.indexOf(name);
  }

  appendColumn(name: string): void {
    this.fieldnames.push(name);
    for (let i = 0; i < this.rows.length; i++) {
      this.rows[i].push("");
    }
  }

  appendColumns(namelist: Array<string>): void {
    this.fieldnames = this.fieldnames.concat(namelist);
    for (let i = 0; i < this.rows.length; i++) {
      for (let j = 0; j < namelist.length; j++) {
        this.rows[i].push("");
      }
    }
  }

  insertColumn(index: number, name: string): void {
    this.fieldnames.splice(index, 0, name);
    for (let i = 0; i < this.rows.length; i++) {
      this.rows[i].splice(index, 0, "");
    }
  }

  removeColumnByIndex(index: number): void {
    this.fieldnames.splice(index, 1);
    for (let i = 0; i < this.rows.length; i++) {
      this.rows[i].splice(index, 1);
    }
  }

  removeColumnByName(name: string): void {
    for (let i = 0; i < this.fieldnames.length; i++) {
      if (this.fieldnames[i] === name) {
        this.removeColumnByIndex(i);
        return;
      }
    }
  }

  removeEmptyColumns(): void {
    for (let i = 0; i < this.fieldnames.length; i++) {
      let empty = true;
      for (let j = 0; j < this.rows.length; j++) {
        if (this.rows[j][i] !== "") {
          empty = false;
          break;
        }
      }
      if (empty === true) {
        this.removeColumnByIndex(i);
        this.removeEmptyColumns();
        return;
      }
    }
  }

  // Attempts to fill in this CSV object from a string.
  // On success, returns the `this` Csv object.
  // On failure, returns an error string with a user-presentable message.
  fromString(s: string): Csv | string {
    // The string cannot contain double-quotes: programs use those to allow
    // in-field commas, which we disallow.
    if (s.indexOf('"') !== -1) {
      let e = 'Double-quotes (") are disallowed.';
      e += " Double-quotes can be automatically inserted by spreadsheet software";
      e += " when a field contains a comma. Make sure to delete all in-field commas.";
      return e;
    }

    // Split by newline. It's OK for \r to remain: we'll strip each field later.
    // This always produces an array of length at least 1.
    //
    // Trim the string first: it's OK if files end with "\n".
    const lines = s.trim().split("\n");

    // The first row contains fieldnames.
    const fieldnames = lines[0].split(",").map(x => x.trim());

    // Any rows after the first contain data.
    const rows: string[][] = [];
    for (let i = 1; i < lines.length; ++i) {
      const row = lines[i].split(",").map(x => x.trim());
      rows.push(row);
    }

    // Sanity checking time!
    // Every column must be named.
    for (let i = 0; i < fieldnames.length; ++i) {
      if (fieldnames[i] === "") {
        const colname = getSpreadsheetColumnName(i);
        return "Column " + colname + " is missing a column name.";
      }
    }

    // Every row must have the same length as the fieldnames row.
    for (let i = 0; i < rows.length; ++i) {
      if (rows[i].length !== fieldnames.length) {
        const rownum = i + 2; // 1-indexed, and the first row is fieldnames.
        let e = "Row " + rownum + " has " + rows[i].length + " columns,";
        e += " but the first row has " + fieldnames.length + " columns.";
        return e;
      }
    }

    this.fieldnames = fieldnames;
    this.rows = rows;
    return this;
  }

  toString(): string {
    const headers = this.fieldnames.join(",");
    let strRows: Array<string> = [];
    for (let i = 0; i < this.rows.length; i++) {
      strRows.push(this.rows[i].join(","));
    }
    return headers + "\n" + strRows.join("\n") + "\n";
  }
}

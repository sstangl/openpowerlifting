// vim: set ts=4 sts=4 sw=4 et:
'use strict';

// Appease TypeScript compiler.
declare let opldb;
declare let meetdb;


export function db_make_indices_list(): number[] {
    let indices = Array(opldb.data.length);
    for (let i = 0; i < opldb.data.length; ++i) {
        indices[i] = i;
    }
    return indices;
}


export function db_filter(indices: number[], rowcmpfn): number[] {
    return indices.filter(function (e) {
        let row = opldb.data[e];
        return rowcmpfn(row);
    });
}


export function db_sort_numeric_minfirst(indices: number[], colidx): number[] {
    indices.sort(function (a, b) {
        let av = Number(opldb.data[a][colidx]);
        let bv = Number(opldb.data[b][colidx]);
        if (isNaN(av))
            av = Number.MAX_VALUE;
        if (isNaN(bv))
            bv = Number.MAX_VALUE;
        return av - bv;
    });
    return indices;
}


export function db_sort_numeric_maxfirst(indices: number[], colidx): number[] {
    indices.sort(function (a, b) {
        let av = Number(opldb.data[a][colidx]);
        let bv = Number(opldb.data[b][colidx]);
        if (isNaN(av))
            av = Number.MIN_VALUE;
        if (isNaN(bv))
            bv = Number.MIN_VALUE;
        return bv - av;
    });
    return indices;
}


// Keep only the first occurrence of NAME. The indices list should already be sorted.
export function db_uniq_lifter(indices: number[]): number[] {
    let seen = {};

    return indices.filter(function (e) {
        let name = opldb.data[e][opldb.NAME];
        if (seen[name])
            return false;
        seen[name] = true;
        return true;
    });
}


// Look up a meet by information.
// MeetID is not suitable for URLs since it may change on recompilation.
export function db_get_meetid(fed: string, date: string, meetname: string): number {
    for (let i = 0; i < meetdb.data.length; ++i) {
        let row = meetdb.data[i];
        if (row[meetdb.FEDERATION] === fed &&
            row[meetdb.DATE] === date &&
            row[meetdb.MEETNAME] === meetname)
        {
            return i;
        }
    }
    return -1;
}


// Look up a meet by meetpath.
export function db_get_meetid_by_meetpath(meetpath: string): number {
    for (let i = 0; i < meetdb.data.length; ++i) {
        let row = meetdb.data[i];
        if (row[meetdb.MEETPATH] === meetpath) {
            return i;
        }
    }
    return -1;
}

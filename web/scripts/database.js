// vim: set ts=4 sts=4 sw=4 et:
'use strict';

function db_make_indices_list() {
    var indices = Array(opldb.data.length);
    for (var i = 0; i < opldb.data.length; ++i) {
        indices[i] = i;
    }
    return indices;
}


function db_filter(indices, rowcmpfn) {
    return indices.filter(function (e) {
        var row = opldb.data[e];
        return rowcmpfn(row);
    });
}


function db_sort_numeric_minfirst(indices, colidx) {
    indices.sort(function (a, b) {
        var av = Number(opldb.data[a][colidx]);
        var bv = Number(opldb.data[b][colidx]);
        if (isNaN(av))
            av = Number.MAX_SAFE_INTEGER;
        if (isNaN(bv))
            bv = Number.MAX_SAFE_INTEGER;
        return av - bv;
    });
    return indices;
}


function db_sort_numeric_maxfirst(indices, colidx) {
    indices.sort(function (a, b) {
        var av = Number(opldb.data[a][colidx]);
        var bv = Number(opldb.data[b][colidx]);
        if (isNaN(av))
            av = Number.MIN_SAFE_INTEGER;
        if (isNaN(bv))
            bv = Number.MIN_SAFE_INTEGER;
        return bv - av;
    });
    return indices;
}


// Make a sorted list of indices unique on NAME, such that only the first
// occurrence is kept. Really this should be done from the end to make removal
// possible in a single iteration of the array, but it's nice to keep the array
// in HTML presentation order.
function db_uniq_lifter(indices) {
    var seen = {};
    var name;

    for (var i = 0; i < indices.length; ++i) {
        name = opldb.data[indices[i]][opldb.NAME];
        if (seen[name]) {
            indices[i] = -1;
        } else {
            seen[name] = true;
        }
    }

    return indices.filter(function (e) {
        return e >= 0;
    });
}


// Look up a meet by information.
// MeetID is not suitable for URLs since it may change on recompilation.
function db_get_meetid(fed, date, meetname) {
    for (var i = 0; i < meetdb.data.length; ++i) {
        var row = meetdb.data[i];
        if (row[meetdb.FEDERATION] === fed &&
            row[meetdb.DATE] === date &&
            row[meetdb.MEETNAME] === meetname)
        {
            return i;
        }
    }
    return -1;
}


// Get an array of indices 

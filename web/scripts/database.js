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


// Keep only the first occurrence of NAME. The indices list should already be sorted.
function db_uniq_lifter(indices) {
    var seen = {};

    return indices.filter(function (e) {
        var name = opldb.data[e][opldb.NAME];
        if (seen[name])
            return false;
        seen[name] = true;
        return true;
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


// Look up a meet by meetpath.
function db_get_meetid_by_meetpath(meetpath) {
    for (var i = 0; i < meetdb.data.length; ++i) {
        var row = meetdb.data[i];
        if (row[meetdb.MEETPATH] === meetpath) {
            return i;
        }
    }
    return -1;
}


// Get an array of indices 

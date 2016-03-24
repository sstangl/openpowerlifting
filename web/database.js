// vim: set ts=4 sts=4 sw=4 et:
'use strict';


function db_make_indices_list() {
    var indices = Array(opldb.length);
    for (var i = 0; i < opldb.length; ++i) {
        indices[i] = i;
    }
    return indices;
}


function db_filter(indices, rowcmpfn) {
    return indices.filter(function (e) {
        var row = opldb[e];
        return rowcmpfn(row);
    });
}


function db_sort_minfirst(indices, colidx) {
    indices.sort(function (a, b) { return opldb[a][colidx] - opldb[b][colidx]; });
    return indices;
}

function db_sort_maxfirst(indices, colidx) {
    indices.sort(function (a, b) { return opldb[b][colidx] - opldb[a][colidx]; });
    return indices;
}


// Make a sorted list of indices unique on NAME, such that only the first
// occurrence is kept. Really this should be done from the end to make removal
// possible in a single iteration of the array, but it's nice to keep the array
// in HTML presentation order.
function db_uniq_lifter(indices) {
    var seen = {}
    var name;

    for (var i = 0; i < indices.length; ++i) {
        name = opldb[indices[i]][NAME];
        if (seen[name]) {
            indices[i] = -1
        } else {
            seen[name] = true;
        }
    }

    return indices.filter(function (e) {
        return e >= 0;
    });
}

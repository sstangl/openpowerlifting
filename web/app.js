// vim: set ts=4 sts=4 sw=4 et:
'use strict';

(function () {
    var indices = db_make_indices_list();
    indices = db_filter(indices, function(row) { return row[EQUIPMENT] == "Raw"; });
    indices = db_sort_maxfirst(indices, WILKS);
    indices = db_uniq_lifter(indices);

    for (var i = 0; i < 100; i++) {
        var row = opldb[indices[i]];
        var str = "<p>" + row[NAME] + ", " + row[WILKS] + "</p>";
        document.body.innerHTML += str;
    }
})()

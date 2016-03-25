// vim: set ts=4 sts=4 sw=4 et:
'use strict';

var results = document.getElementById("results");

var usingLbs = true;


function weight(kg) {
    if (!usingLbs)
        return kg;
    return Math.round(kg * 2.2042262 * 100) / 100;
}


// Make the HTML for a single database row.
function makeentry(row, i) {
    var str = "<tr>";

    str = str + "<td>" + String(i) + "</td>";
    str = str + "<td>" + row[NAME] + "</td>";
    str = str + "<td>" + row[SEX] + "</td>";
    str = str + "<td>" + row[AGE] + "</td>";
    str = str + "<td>" + weight(row[BODYWEIGHTKG]) + "</td>";
    str = str + "<td>" + weight(row[BESTSQUATKG]) + "</td>";
    str = str + "<td>" + weight(row[BESTBENCHKG]) + "</td>";
    str = str + "<td>" + weight(row[BESTDEADLIFTKG]) + "</td>";
    str = str + "<td>" + weight(row[TOTALKG]) + "</td>";
    str = str + "<td>" + row[WILKS] + "</td>";
    str = str + "<td>" + row[MCCULLOCH] + "</td>";

    return str + "</tr>";
}

(function () {
    var indices = db_make_indices_list();
    indices = db_filter(indices, function(row) { return row[EQUIPMENT] == "Raw"; });
    indices = db_sort_numeric_maxfirst(indices, WILKS);
    indices = db_uniq_lifter(indices);

    var html = "<table>";

    html += "<td>Rank</td>";
    html += "<td>Name</td>";
    html += "<td>Sex</td>";
    html += "<td>Age</td>";
    html += "<td>Bodyweight</td>";
    html += "<td>Squat</td>";
    html += "<td>Bench</td>";
    html += "<td>Deadlift</td>";
    html += "<td>Total</td>";
    html += "<td>Wilks</td>";
    html += "<td>McCulloch</td>";

    for (var i = 0; i < indices.length; i++) {
        var row = opldb[indices[i]];
        html += makeentry(row, i);
    }

    html += "</table>";
    results.innerHTML = html;
})()

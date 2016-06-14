// vim: set ts=4 sts=4 sw=4 et:
// Common code across the various OpenPowerlifting pages.
'use strict';

var common = (function () {

return {
    kg2lbs: function(kg) {
        return Math.round(kg * 2.2042262 * 100) / 100;
    },
    
    // Roughly parse index.html?q=foo&a=bar into an object {q: foo, a: bar}.
    getqueryobj: function() {
        var url = document.location.href;
        var i = url.indexOf('?');
        var args = url.slice(i+1);

        var obj = {};
        var split = args.split('&');
        for (var j = 0; j < split.length; ++j) {
            var arg = split[j];
            if (arg.indexOf('=') >= 0) {
                var v = unescape(arg).split('=');
                obj[v[0]] = v[1];
            }
        }
        return obj;
    },
    
    number: function(num) {
        if (num === undefined)
            return '';
        return String(num);
    },

    string: function(str) {
        if (str === undefined)
            return '';
        return str;
    },
    
    parseEquipment: function(str) {
        if (str === "Raw")
            return "R";
        if (str === "Wraps")
            return "W";
        if (str === "Single-ply")
            return "S";
        if (str === "Multi-ply")
            return "M";
        return "";
    },

    colidToIndex: function(colid) {
        switch (colid) {
            case "fed": return meetdb.FEDERATION;
            case "date": return meetdb.DATE;
            case "age": return opldb.AGE;
            case "bw": return opldb.BODYWEIGHTKG;
            case "squat": return opldb.BESTSQUATKG;
            case "bench": return opldb.BESTBENCHKG;
            case "deadlift": return opldb.BESTDEADLIFTKG;
            case "total": return opldb.TOTALKG;
            case "wilks": return opldb.WILKS;
            case "mcculloch": return opldb.MCCULLOCH;
            default:
                console.log("Unknown: colidToIndex(" + name + ")");
                return undefined;
        }
    },

    getSortFn: function(colid, sortAsc) {
        var index = this.colidToIndex(colid);
        switch (colid) {
            // Columns that use the meetdb.
            case "fed":
            case "date":
                return function(a, b) {
                    var ameetid = opldb.data[a][opldb.MEETID];
                    var bmeetid = opldb.data[b][opldb.MEETID];
                    var adata = meetdb.data[ameetid][index];
                    var bdata = meetdb.data[bmeetid][index];
                    if (sortAsc)
                        return adata > bdata;
                    return adata <= bdata;
                };

            // Columns that use the opldb.
            case "age":
            case "bw":
            case "squat":
            case "bench":
            case "deadlift":
            case "total":
            case "wilks":
            case "mcculloch":
                return function(a, b) {
                    var adata = opldb.data[a][index];
                    var bdata = opldb.data[b][index];
                    if (adata === undefined)
                        adata = Number.MIN_SAFE_INTEGER;
                    if (bdata === undefined)
                        bdata = Number.MIN_SAFE_INTEGER;
                    if (sortAsc)
                        return adata > bdata;
                    return adata <= bdata;
                };

            default:
                console.log("Unknown: gotSortFn(" + colid + ", " + sortAsc + ")");
                return undefined;
        }
    }


};

})();

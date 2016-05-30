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
        for (let arg of args.split('&') ) {
            if (arg.indexOf('=') >= 0) {
                let v = unescape(arg).split('=');
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

    parseWeightClass: function(x) {
        if (x === undefined)
            return "";
        if (typeof x === "number")
            return weight(x);
        return weight(x.split('+')[0]) + '+';
    },
};

})();

// vim: set ts=4 sts=4 sw=4 et:
// Common code across the various OpenPowerlifting pages.
'use strict';

var common = (function () {

return {
    kg2lbs: function(kg) {
        return Math.round(kg * 2.20462262 * 100) / 100;
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

    // Return an object with properties set as strings to be presented.
    makeRowObj: function(row, index) {
        var meetrow = meetdb.data[row[opldb.MEETID]];

        var country = this.string(meetrow[meetdb.MEETCOUNTRY]);
        var state = this.string(meetrow[meetdb.MEETSTATE]);
        var location = country;
        if (country && state) {
            location = location + "-" + state;
        }

        var name = row[opldb.NAME];
        var fed = this.string(meetrow[meetdb.FEDERATION]);
        var date = this.string(meetrow[meetdb.DATE]);
        var meetname = this.string(meetrow[meetdb.MEETNAME]);
        var meeturl = this.makeMeetUrl(fed, date, meetname);

        return {
            rank:        index+1,
            place:       this.string(row[opldb.PLACE]),
            searchname:  name.toLowerCase(),
            name:        '<a href="' + this.makeLiftersUrl(name) + '">' + name + '</a>',
            fed:         fed,
            date:        '<a href="' + meeturl + '">' + date + '</a>',
            location:    location,
            division:    this.string(row[opldb.DIVISION]),
            meetname:    '<a href="' + meeturl + '">' + meetname + '</a>',
            sex:         this.string(row[opldb.SEX]),
            age:         this.string(row[opldb.AGE]),
            equip:       this.parseEquipment(row[opldb.EQUIPMENT]),
            bw:          weight(row[opldb.BODYWEIGHTKG]), // TODO: this.weight()
            weightclass: parseWeightClass(row[opldb.WEIGHTCLASSKG]), // TODO: this.parseWeightClass()
            squat:       this.weightMax(row, opldb.BESTSQUATKG, opldb.SQUAT4KG),
            bench:       this.weightMax(row, opldb.BESTBENCHKG, opldb.BENCH4KG),
            deadlift:    this.weightMax(row, opldb.BESTDEADLIFTKG, opldb.DEADLIFT4KG),
            total:       weight(row[opldb.TOTALKG]), // TODO: this.weight()
            wilks:       this.number(row[opldb.WILKS]),
            mcculloch:   this.number(row[opldb.MCCULLOCH])
        };

    },

    makeLiftersUrl: function(name) {
        return "lifters.html?q=" + escape(name);
    },

    makeMeetUrl: function(fed, date, meetname) {
        return "meet.html?f=" + escape(fed) +
                        "&d=" + escape(date) +
                        "&n=" + escape(meetname);
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

    // FIXME: Requires the enclosing page to define a weight() global.
    weightMax: function(row, cola, colb) {
        var a = row[cola];
        var b = row[colb];
        if (a === undefined)
            return weight(b);
        if (b === undefined)
            return weight(a);
        return weight(Math.max(a,b));
    },
    
    parseEquipment: function(str) {
        if (str === "Raw")
            return "Raw";
        if (str === "Wraps")
            return "Wraps";
        if (str === "Single-ply")
            return "Single";
        if (str === "Multi-ply")
            return "Multi";
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
    },

    // Adapted from SlickGrid's flashCell().
	flashRow: function(tr) {
		function toggleCellClass(node, times) {
			if (times === 0)
				return;
			setTimeout(
				function () {
                    var classes = node.getAttribute('class');
                    if (!classes)
                        classes = '';

                    // Simple toggleClass() since no JQuery in some files.
                    if (times % 2 == 0)
                        classes += ' searchflashing ';
                    else
                        classes = classes.replace(' searchflashing ','');

					node.setAttribute('class', classes);
					toggleCellClass(node, times - 1);
				},
				100 // ms
			);
		}

        // The flashing must be done by setting <td> classes, since the <tr>
        // nth-line-color CSS overrules any flashing we might add.
        for (var i = 0; i < tr.childNodes.length; ++i) {
            // Only consider element nodes.
            if (tr.childNodes[i].nodeType === 1) {
		        toggleCellClass(tr.childNodes[i], 4);
            }
        }
	},

    // A list of federations using drug-testing.
    testedFederationList: [
        'IPF',
        'PA',
        'USAPL'
    ]
};

})();

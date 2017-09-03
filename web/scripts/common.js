// vim: set ts=4 sts=4 sw=4 et:
// Common code across the various OpenPowerlifting pages.

'use strict';
const KG_CONVERSION = 2.20462262;

export var kg2lbs = kg => Math.round(kg * KG_CONVERSION * 10) / 10;
export var lbs2kg = lb => Math.round(lb / KG_CONVERSION * 10) / 10;

// Remembers the selected weight type.
// Values are "kg" or "lb".
var WeightTypeState = 'lb';
export function setWeightTypeState(state) {
    // state must be either "kg" or "lb".
    // TODO: Bring in an assertion library and assert in debug builds.
    WeightTypeState = state;
}
export function getWeightTypeState() {
    return WeightTypeState;
}

export function weight(kg) {
    if (kg === undefined)
        return '';
    if (getWeightTypeState() === "kg")
        return String(kg);
    return String(kg2lbs(kg));
}

export function parseWeightClass(x) {
    if (x === undefined)
        return '';
    if (getWeightTypeState() == "kg")
        return String(x)
    if (typeof x === "number")
        return String(Math.floor(kg2lbs(x)));
    return String(Math.floor(kg2lbs(x.split('+')[0]))) + '+';
}

// Roughly parse index.html?q=foo&a=bar into an object {q: foo, a: bar}.
export function getqueryobj() {
    var url = document.location.href;
    var i = url.indexOf('?');
    var args = url.slice(i+1);

    // Facebook mangles URLs, replacing ' ' with '+'.
    var pluses = new RegExp('\\+', 'g');

    var obj = {};
    var split = args.split('&');
    for (var j = 0; j < split.length; ++j) {
        var arg = split[j];
        if (arg.indexOf('=') >= 0) {
            var v = decodeURIComponent(arg).replace(pluses, ' ').split('=');
            obj[v[0]] = v[1];
        }
    }
    return obj;
}

// Return an object with properties set as strings to be presented.
export function makeRowObj(row, index) {
    var meetrow = meetdb.data[row[opldb.MEETID]];

    var country = this.string(meetrow[meetdb.MEETCOUNTRY]);
    var state = this.string(meetrow[meetdb.MEETSTATE]);
    var location = country;
    if (country && state) {
        location = location + "-" + state;
    }

    var fullname = row[opldb.NAME];
    var name = '<a href="' + this.makeLiftersUrl(fullname) + '">' + fullname + '</a>';

    // XXX: Bad hack to make Ben's name pink, per request.
    if (fullname === "Ben Gianacakos") {
        name = '<a style="text-decoration-color: #FF80AB;" href="' + this.makeLiftersUrl(fullname) + '"><span style="color: #FF80AB;">' + fullname + '</span></a>';
    }

    // Attempt to read in social media data, if present.
    if (window.socialmedia !== undefined) {
        var social = window.socialmedia[fullname];
        if (social !== undefined) {
            name = name + ' <a href="https://www.instagram.com/' + social[0] + '">'
                        + '<img class="instagram" src="images/ig-glyph-logo_May2016.png">'
                        + '</a>';
        }
    }

    var fed = this.string(meetrow[meetdb.FEDERATION]);
    var date = this.string(meetrow[meetdb.DATE]);
    var meetname = this.string(meetrow[meetdb.MEETNAME]);
    var meeturl = this.makeMeetUrl(meetrow[meetdb.MEETPATH]);
    var sex = (row[opldb.SEX] === 0) ? 'M' : 'F';

    // Age uses .5 to show imprecision. The lower bound is given.
    // Tilde is shown at the end so numbers continue to line up,
    // and as a hint to it being a lower bound.
    var age = this.number(row[opldb.AGE]);
    if (age.indexOf('.5') >= 0) {
        age = age.replace('.5','~');
    }

    return {
        rank:        index+1,
        place:       this.string(row[opldb.PLACE]),
        searchname:  name.toLowerCase(),
        name:        name,
        fed:         fed,
        date:        '<a href="' + meeturl + '">' + date + '</a>',
        location:    location,
        division:    this.string(row[opldb.DIVISION]),
        meetname:    '<a href="' + meeturl + '">' + meetname + '</a>',
        sex:         sex,
        age:         age,
        equip:       this.parseEquipment(row[opldb.EQUIPMENT]),
        bw:          this.weight(row[opldb.BODYWEIGHTKG]),
        weightclass: this.parseWeightClass(row[opldb.WEIGHTCLASSKG]),
        squat:       this.weightMax(row, opldb.BESTSQUATKG, opldb.SQUAT4KG),
        bench:       this.weightMax(row, opldb.BESTBENCHKG, opldb.BENCH4KG),
        deadlift:    this.weightMax(row, opldb.BESTDEADLIFTKG, opldb.DEADLIFT4KG),
        total:       this.weight(row[opldb.TOTALKG]),
        wilks:       this.number(row[opldb.WILKS]),
        mcculloch:   this.number(row[opldb.MCCULLOCH])
    };

}

export function makeLiftersUrl(name) {
    return "lifters.html?q=" + encodeURIComponent(name);
}

export function makeMeetUrl(meetpath) {
    return "meet.html?m=" + encodeURIComponent(meetpath);
}

export function number(num) {
    if (num === undefined)
        return '';
    return String(num);
}

export function string(str) {
    if (str === undefined)
        return '';
    return str;
}

// FIXME: Requires the enclosing page to define a weight() global.
export function weightMax(row, cola, colb) {
    var a = row[cola];
    var b = row[colb];
    if (a === undefined)
        return weight(b);
    if (b === undefined)
        return weight(a);
    return weight(Math.max(a,b));
}

export function parseEquipment(str) {
    // Values set by web/Makefile.
    if (str === 0)
        return "Raw";
    if (str === 1)
        return "Wraps";
    if (str === 2)
        return "Single";
    if (str === 3)
        return "Multi";
    if (str === 4) // For Yury Belkin.
        return "Straps";
    return "";
}

export function colidToIndex(colid) {
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
}

export function getSortFn(colid, sortAsc) {
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

// Adapted from SlickGrid's flashCell().
export function flashRow(tr) {
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
}

// Returns a (min,max] tuple for the values in templates/weightclass.frag,
// which controls the weightclass selector.
export function getWeightRange(sel) {
    switch (sel) {
        // Traditional weights.
        case 't44': return [0.0, 44.0];
        case 't48': return [44.0, 48.0];
        case 't52': return [48.0, 52.0];
        case 't56': return [52.0, 56.0];
        case 't60': return [56.0, 60.0];
        case 't67.5': return [60.0, 67.5];
        case 't75': return [67.5, 75.0];
        case 't82.5': return [75.0, 82.5];
        case 't90': return [82.5, 90.0];
        case 't90+': return [90.0, 999.0];
        case 't100': return [90.0, 100.0];
        case 't110': return [100.0, 110.0];
        case 't125': return [110.0, 125.0];
        case 't140': return [125.0, 140.0];
        case 't140+': return [140.0, 999.0];

        // IPF Men.
        case 'm53': return [0.0, 53.0];
        case 'm59': return [53.0, 59.0];
        case 'm66': return [59.0, 66.0];
        case 'm74': return [66.0, 74.0];
        case 'm83': return [74.0, 83.0];
        case 'm93': return [83.0, 93.0];
        case 'm105': return [93.0, 105.0];
        case 'm120': return [105.0, 120.0];
        case 'm120+': return [120.0, 999.0];

        // IPF Women.
        case 'f43': return [0.0, 43.0];
        case 'f47': return [43.0, 47.0];
        case 'f52': return [47.0, 52.0];
        case 'f57': return [52.0, 57.0];
        case 'f63': return [57.0, 63.0];
        case 'f72': return [63.0, 72.0];
        case 'f84': return [72.0, 84.0];
        case 'f84+': return [84.0, 999.0];

        default: return [0.0, 999.0];
    }
}

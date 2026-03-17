#!/usr/bin/env python

import sys
from csv import DictReader, DictWriter

# argv[1] is path to captured checker output
with open(sys.argv[1], "rt") as in_f:
    entries_file = None
    fix_map = {}
    for line in in_f:
        line = line.strip()
        if line.startswith("/") and line.endswith("entries.csv"):
            entries_file = line
            fix_map[entries_file] = []
        elif all((
            line.startswith("Line"),
            "Calculated TotalKg" in line,
            "but meet recorded" in line,
        )):
            fields = line.split(' ')
            fix_map[entries_file].append({
                "line_num": int(fields[1].replace(":", "")),
                "calc_total": fields[4].replace("'", "").replace(",", ""),
                "rec_total": fields[8].replace("'", "")
            })
for (entries_file, fixes,) in fix_map.items():
    fixed_entries = []
    print(f"Fixing {entries_file}")
    with open(entries_file, "rt") as entries_f:
        for (entry_i, entry,) in enumerate(DictReader(entries_f)):
            # 0th entry happens on line 2
            if len(fixes) > 0 and (entry_i + 2) == fixes[0]["line_num"]:
                fixed_entry = dict(entry)
                if fixed_entry["TotalKg"] != fixes[0]["rec_total"]:
                    print(
                        f'Expected to see {fixes[0]["rec_total"]} '
                        f'but saw {fixed_entry["TotalKg"]} on line {entry_i}'
                    )
                    continue
                else:
                    fixed_entry["TotalKg"] = fixes[0]["calc_total"]
                    print(
                        f'Fixed {fixed_entry["Name"]} - {fixes[0]["rec_total"]} '
                        f'to {fixed_entry["TotalKg"]}'
                    )
                    fixes.pop(0)
                    fixed_entries.append(fixed_entry)
            else:
                fixed_entries.append(entry)
    with open(entries_file, "wt") as entries_f:
        dw = DictWriter(entries_f, fixed_entries[0].keys())
        dw.writeheader()
        dw.writerows(fixed_entries)

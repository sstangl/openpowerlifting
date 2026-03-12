#!/usr/bin/env python

import sys

# argv[1] is path to captured checker output
with open(sys.argv[1], "rt") as in_f:
    entries_file = None
    fix_map = {}
    for line in in_f:
        line = line.strip()
        if line.startswith("/") and line.endswith("entries.csv"):
            entries_file = line 
            fix_map[entries_file] = []
        elif line.startswith("Line") and \
            "Calculated TotalKg" in line and \
            "but meet recorded" in line:
            fields = line.split(' ')
            fix_map[entries_file].append({
                # enumerate() will start from 0, but the checker doesn't
                "line_num": int(fields[1].replace(":", "")) - 1,
                "calc_total": fields[4].replace("'", "").replace(",", ""),
                "rec_total": fields[8].replace("'", "")
            })
for (entries_file, fixes,) in fix_map.items():
    with open(entries_file, "rt") as entries_f:
        fixed_entries_lines = []
        read_entries_lines = [line.strip() for line in entries_f]
        for (line_num, read_entries_line,) in enumerate(read_entries_lines):
            if len(fixes) > 0 and line_num == fixes[0]["line_num"]:
                fixed_entries_lines.append(
                    read_entries_line.replace(
                        fixes[0]["rec_total"],
                        fixes[0]["calc_total"]
                    )
                )
                fixes.pop(0)
            else:
                fixed_entries_lines.append(read_entries_line)    
                
    with open(entries_file, "wt") as entries_f:
        entries_f.write("\n".join(fixed_entries_lines))
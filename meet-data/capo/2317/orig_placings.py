from collections import defaultdict

entries_lines = []
orig_map = defaultdict(list)
with open("original.csv", "rt") as original_f:
    for line in original_f:
        fields = line.split(",")
        place = fields[0]
        name = fields[1]
        if place != "Place" and "(4th)" not in name:
            if name == "Erin Mccabe":
                name = "Erin McCabe"
            if name == "Madeleine Macintyre (G)":
                name = "Madeleine Macintyre"
                place = "G"
            if name == "Philip van der Hoek":
                name = "Philip Van Der Hoek"
            orig_map[name].append(place)

with open("entries.csv", "rt") as entries_f, open("entries.new.csv", "wt") as entries_new_f:
    for line in entries_f:
        fields = line.split(",")
        name = fields[1]
        if name != "Name":
            disambig_index = name.find("#")
            if disambig_index != -1:
                name = name[:(disambig_index - 1)]
            orig_place = orig_map[name]
            if len(orig_place) == 1:
                fields[0] = orig_place[0]
            else:
                # if there's multiple placings due to double entry
                # then just chuck the list in and work it out manually:w
                fields[0] = str(orig_place)
        entries_new_f.write(",".join(fields))


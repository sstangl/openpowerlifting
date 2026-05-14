from pathlib import Path
# oplcsv isn't suitable for this because it doesn't handle quoted commas
from csv import DictReader, DictWriter
import sys

#TODO - change oplcsv usage to mainstream csv


# this takes an original.csv translated to English, having previously been
# converted from the original.xlsx

# NAP run loads of different events and a lot of them are irrelevant.
# They all have section headers like:

# , Streetlifting repetition Standard. Bench+Deadlift,,,,,,,,,,,,,,,,,,,,,,,,,,,
# or
# ,Amateur. Military press repetition OWN ВЕС,,,,,,,,,,,,,,,,,,,,,,,,,,,
# or
# ,Amateur. Press bench. В multi-layer equipment,,,,,,,,,,,,,,,,,,,,,,,,,,,  

# when we find a relevant one, produce a Csv with the entries in that section
# and transform them based on the header and concatenate them together

def get_section_header(input_csv_row):
    # section headers have field 1 populated and no others
    if len(input_csv_row) < 1:
        return False
    for input_field_i, input_field in enumerate(input_csv_row):
        if input_field_i == 1 and input_field.strip() == "":
            return False
        if input_field_i != 1 and input_field.strip() != "":
            return False 
    return input_csv_row[1].strip()

def is_blank_row(input_row):
    for field in input_row:
        if len(field) > 0:
            return False
    return True

def gen_section_csvs(input_csv):
    # iterate over the rows in the input csv and yield a csv
    # from the rows under each section header, and the section header
    # skip past the first two rows, which is meet data
    include_section = False
    section_csv = None
    section_header = None
    for input_row in input_csv.rows[2:]:
        new_section_header = get_section_header(input_row)
        if new_section_header:

            #DEBUG
            print(f"new section header:{new_section_header}")
            print(f"section_csv:{section_csv}")
            if section_csv:
                print(f"section_csv:{section_csv} has {len(section_csv)} entries")
            print(f"section header:{section_header}")

            # if we had a section CSV from the previous section, yield it now
            if section_csv and section_header:

                #DEBUG
                print(f"about to yield csv with {len(section_csv)} entries")

                yield section_csv, section_header
                section_csv = None
            section_header = new_section_header
            lc_section_header = section_header.lower()
            include_section = False
            exclude_section = False
            csv_header_row = None
            # don't include "people's bench press" etc or "paired deadlift"
            # anything related to curl or military press
            # any "Russian" variant on an event (Russian press, Russian deadlift)
            # any streetlifting event (pullups etc)
            # overall winners standings
            # coaches standings
            # "bench+deadlift" isn't actually push/pull, it's some kind of max single
            # plus reps event
            for exclude_term in [
                "people", "paired", "curl", "military", "russian", "streetlifting",
                "overall", "coaches", "bench+deadlift"
            ]:
                if exclude_term in lc_section_header:

                    #DEBUG
                    print(f"Matched exclude term {exclude_term}")

                    exclude_section = True
                    break
            if not exclude_section:
                for include_term in [
                    "press bench", "bench press", "squat", "deadlift", "powerlifting"
                ]:
                    if include_term in lc_section_header:

                        #DEBUG
                        print(f"Matched include term {include_term}")

                        include_section = True
                        break
        elif include_section:

            #DEBUG
            print(f"include section, section_csv:{section_csv}")

            # got the section header, next one is CSV header
            # place, name (cyrillic), and sex aren't labelled, but they're the first three
            if section_csv is None:
                csv_header_row = ["Place", "CyrillicName", "Sex"] + input_row[3:]
                section_csv = Csv()
                section_csv.append_columns(csv_header_row)

                #DEBUG
                print("Created section CSV")

            # we're in the body of the section, accumulate rows
            elif section_csv is not None:
                # don't include blank rows

                #DEBUG
                print("including section and we have section CSV")

                if not is_blank_row(input_row):
                    section_csv.rows.append(input_row)

                    #DBEUG
                    print(f"Added row to section CSV {input_row}")

def parse_born(born):

    #DEBUG
    print(f"born: {born}")

    [orig_dob, age] = born.split("/")
    [dd, mm, yyyy] = orig_dob.split(".")
    opl_dob = f"{yyyy}-{mm}-{dd}"
    return opl_dob, age

def lift(weight):
    return weight if weight != "-" else ""

def make_opl_section_csv(section_csv, section_header):
    # return a csv with the OPL schema, from the data in the section
    # csv, and section header
    lc_section_header = section_header.lower()
    div_performance_class = None
    for pc in ["amateur", "pro", "elite"]:
        if pc in lc_section_header:
            div_performance_class = pc.title()
            break
    if "press bench" in lc_section_header or "bench press" in lc_section_header:
        event = "B"
        bench_columns = ["Bench1Kg", "Bench2Kg" "Bench3Kg", "Bench4Kg", "Best3BenchKg"]
        event_columns = bench_columns
    if "squat" in lc_section_header:
        event = "S"
        squat_columns = ["Squat1Kg", "Squat2Kg" "Squat3Kg", "Squat4Kg", "Best3SquatKg"]
        event_columns = squat_columns
    if "deadlift" in lc_section_header:
        event = "D"
        deadlift_columns = [
            "Deadlift1Kg", "Deadlift2Kg" "Deadlift3Kg", "Deadlift4Kg", "Best3DeadliftKg"
        ]
        event_columns = deadlift_columns
    if "powerlifting" in lc_section_header:
        event = "SBD"
        event_columns = squat_columns + bench_columns + deadlift_columns
    if "without equipment" in lc_section_header:
        equipment = "Raw"
    if "single-layer equipment" in lc_section_header:
        equipment = "Single-ply"
    if "multi-layer equipment" in lc_section_header:
        equipment = "Multi-ply"
    if "soft-equipment" in lc_section_header:
        if event == "S" or event == "SBD":
            equipment = "Wraps"
        else:
            equipment = "Unlimited"
    opl_csv = Csv()
    opl_csv.append_columns([
        "Place", "CyrillicName", "Sex", "BirthDate", "Age", "Equipment", "Division",
        "WeightClassKg", "BodyweightKg" 
    ] + event_columns + ["TotalKg"]
    )

    #DEBUG
    print(f"section_csv.fieldnames: {section_csv.fieldnames}")

    for sr in section_csv.rows:

        #DEBUG
        print(f"row: {sr}")

        birth_date, age = parse_born(sr[section_csv.index("Born")])
        age_div = sr[section_csv.index("Age Division")]
        division = f"{div_performance_class} {age_div}"
        opl_row = [
            sr[section_csv.index("Place")], sr[section_csv.index("CyrillicName")],
            sr[section_csv.index("Sex")], birth_date, age, equipment, division,
            sr[section_csv.index("Weight Class")], sr[section_csv.index("Bodyweight")],
        ]
        if event == "S" or event == "SBD":
            opl_row += [
                lift(sr[section_csv.index("A1")]), lift(sr[section_csv.index("A2")]),
                lift(sr[section_csv.index("A3")]), lift(sr[section_csv.index("A4(R)")]),
                lift(sr[section_csv.index("Squats")])
            ]
        if event == "B" or event == "SBD":
            opl_row += [
                lift(sr[section_csv.index("B1")]), lift(sr[section_csv.index("B2")]),
                lift(sr[section_csv.index("B3")]), lift(sr[section_csv.index("B4(R)")]),
                lift(sr[section_csv.index("Press bench")])
            ]
        if event == "D" or event == "SBD":
            opl_row += [
                lift(sr[section_csv.index("C1")]), lift(sr[section_csv.index("C2")]),
                lift(sr[section_csv.index("C3")]), lift(sr[section_csv.index("C4(R)")]),
                lift(sr[section_csv.index("Deadlift pull")])
            ]
        opl_row += ["Total"]
        opl_csv.rows.append(opl_row)
    return opl_csv

def make_meet_csv():
    # return a meet csv with the data common to all meets.  The rest will need
    # to be filled in manually from original_translated.csv
    meet_csv = Csv()
    meet_csv.append_columns([
        "Federation","Date","MeetCountry","MeetState","MeetTown","MeetName"
    ])
    meet_csv.rows.append([
        "NAP","","Russia","","",""
    ])
    return meet_csv

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} translated_original_csv", file=sys.stderr)
        sys.exit(1)
    input_path_str = sys.argv[1]
    input_dir_path = Path(input_path_str).parent
    input_csv = Csv(input_path_str)
    entries_csv = Csv()
    for section_csv, section_header in gen_section_csvs(input_csv):
        section_opl_entries_csv = make_opl_section_csv(section_csv, section_header)
        entries_csv.cat(section_opl_entries_csv)
    meet_csv = make_meet_csv()
    meet_csv.write_filename(str(input_dir_path / "meet.csv"))
    entries_csv.write_filename(str(input_dir_path / "entries.csv"))

from pathlib import Path
from io import StringIO
# oplcsv isn't suitable for this because it doesn't handle quoted commas
from csv import DictReader, DictWriter

import sys
import logging
import os

log_level = os.getenv("LOG_LEVEL") or "INFO"
logging.basicConfig(level=log_level.upper())
logger = logging.getLogger(__name__)

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

def get_section_header(input_line):
    # section headers have field 1 populated and no others
    input_csv_row = input_line.split(",")
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

def get_section_metadata(section_header_str):
    logger.debug(f"Getting section metadata from section header:{section_header_str}")
    # each section header contains data about its entries that we need in
    # the OPL schema
    section_d = {}
    lc_section_header = section_header_str.lower()
    for pc in ["amateur", "pro", "elite"]:
        if pc in lc_section_header:
            section_d["div_performance_class"] = pc.title()
            break
    if "press bench" in lc_section_header or "bench press" in lc_section_header:
        section_d["event"] = "B"
    if "squat" in lc_section_header:
        section_d["event"] = "S"
    if "deadlift" in lc_section_header:
        section_d["event"] = "D"
    if "powerlifting" in lc_section_header:
        section_d["event"] = "SBD"
    if "without equipment" in lc_section_header:
        section_d["equipment"] = "Raw"
    if "single-layer equipment" in lc_section_header:
        section_d["equipment"] = "Single-ply"
    if "multi-layer equipment" in lc_section_header:
        section_d["equipment"] = "Multi-ply"
    if "soft-equipment" in lc_section_header:
        if section_d["event"] == "S" or section_d["event"] == "SBD":
            section_d["equipment"] = "Wraps"
        else:
            section_d["equipment"] = "Unlimited"
    # it's unclear what "combined" means but it seems to combine performance tiers
    # and equipment types. For now, call it amateur / unlimited
    if "combined" in lc_section_header:
        section_d["div_performance_class"] = "Amateur"
        section_d["equipment"] = "Unlimited"
    if not section_d.get("equipment") and section_d["div_performance_class"] == "Elite":
        section_d["equipment"] = "Unlimited"
    logger.debug(f"Determined section metadata:{section_d}") 
    return section_d

def transform_lifter_name(cyrillic_name):
    # expect "family given [patronymic]", return "given family"
    # "DC-" is either "Defending Champion" or "Doping Control"
    # either way, we're stripping it for now
    cyrillic_name = cyrillic_name.replace("DC- ", "")
    elements = cyrillic_name.split(" ")
    if len(elements) < 2:
        return cyrillic_name
    return f"{elements[1]} {elements[0]}"

def fix_weight_class(weight_class):
    # turn "+x" into "x+"
    if weight_class.startswith("+"):
        return f"{weight_class[1:]}+"
    return weight_class

def make_opl_entry(section_entry_d, section_metadata_d):
    logger.debug(f"Original entry:{section_entry_d}")
    logger.debug(f"Section metadata:{section_metadata_d}")
    opl_entry_d = {}
    opl_entry_d["BirthDate"], opl_entry_d["Age"] = parse_born(section_entry_d["Born"])
    opl_entry_d["Division"] = \
        f'{section_metadata_d["div_performance_class"]} {section_entry_d["Age Division"]}'
    opl_entry_d["Place"] = \
        section_entry_d["Place"] if section_entry_d["Place"] != "-" else "DQ"
    opl_entry_d["CyrillicName"] = transform_lifter_name(section_entry_d["CyrillicName"])
    opl_entry_d["Sex"] = section_entry_d["Sex"]
    opl_entry_d["Equipment"] = section_metadata_d["equipment"]
    opl_entry_d["WeightClassKg"] = fix_weight_class(section_entry_d["Weight Class"])
    opl_entry_d["BodyweightKg"] = section_entry_d["Weight"]
    if section_metadata_d["event"] in ["S", "SBD"]:
        opl_entry_d["Squat1Kg"] = lift(section_entry_d["A1"])
        opl_entry_d["Squat2Kg"] = lift(section_entry_d["A2"])
        opl_entry_d["Squat3Kg"] = lift(section_entry_d["A3"])
        opl_entry_d["Squat4Kg"] = lift(section_entry_d["A4(R)"])
        opl_entry_d["Best3SquatKg"] = lift(section_entry_d["Squats"])
    if section_metadata_d["event"] in ["B", "SBD"]:
        opl_entry_d["Bench1Kg"] = lift(section_entry_d["B1"])
        opl_entry_d["Bench2Kg"] = lift(section_entry_d["B2"])
        opl_entry_d["Bench3Kg"] = lift(section_entry_d["B3"])
        opl_entry_d["Bench4Kg"] = lift(section_entry_d["B4(R)"])
        opl_entry_d["Best3BenchKg"] = lift(section_entry_d["Press bench"])
    #NOTE: in the untranslated original, these use Cyrillic C's ("Es"), not
    # Latin C's.  The Russian dictionary the translation script uses should
    # take care of these
    if section_metadata_d["event"] in ["D", "SBD"]:
        opl_entry_d["Deadlift1Kg"] = lift(section_entry_d["C1"])
        opl_entry_d["Deadlift2Kg"] = lift(section_entry_d["C2"])
        opl_entry_d["Deadlift3Kg"] = lift(section_entry_d["C3"])
        opl_entry_d["Deadlift4Kg"] = lift(section_entry_d["C4(R)"])
        opl_entry_d["Best3DeadliftKg"] = lift(section_entry_d["Deadlift pull"])
    opl_entry_d["TotalKg"] = section_entry_d["Total"]
    opl_entry_d["Event"] = section_metadata_d["event"]
    logger.debug(f"OPL entry:{opl_entry_d}")
    return opl_entry_d

def gen_opl_entries(input_f):
    # iterate over the rows in the input csv and yield
    # dicts in OPL entries file schema. We don't pass a DictReader
    # because each section has its own schema
    include_section = False
    section_csv_entry_lines = []
    section_header = None
    section_entry_header = None
    for input_line in input_f:
        input_line = input_line.strip()
        # ignore "blank" lines of all commas
        if input_line.count(",") == len(input_line):
            continue
        new_section_header = get_section_header(input_line)
        if new_section_header:
            logger.debug(f"Got new section header:{new_section_header}")
            # new section header and we've already accumulated lines in this section
            if len(section_csv_entry_lines) > 0:
                logger.debug(f"{len(section_csv_entry_lines)} entries in previous section")
                section_io = StringIO("\n".join([section_entry_header] + section_csv_entry_lines))
                section_dr = DictReader(section_io)
                section_metadata_d = get_section_metadata(section_header)
                logger.debug(f"Got section header metadata:{section_metadata_d}")
                for section_entry_d in section_dr:
                    yield make_opl_entry(section_entry_d, section_metadata_d)
            section_header = new_section_header
            section_entry_header = None
            section_csv_entry_lines = []
            include_section = False
            exclude_section = False
            lc_section_header = section_header.lower()
            # explicitly exclude:
            # - "people's bench press" etc or "paired deadlift"
            # - anything related to curl or military press
            # - any "Russian" variant on an event (Russian press, Russian deadlift)
            # - any streetlifting event (pullups etc)
            # - overall winners standings
            # - coaches standings
            # - "bench+deadlift" - isn't actually push/pull, it's some kind of max single
            # plus reps event
            for exclude_term in [
                "people", "paired", "curl", "military", "russian", "streetlifting",
                "overall", "coaches", "bench+deadlift"
            ]:
                if exclude_term in lc_section_header:
                    exclude_section = True
                    logger.debug(f"Excluding section with header {section_header} matching term {exclude_term}")
                    break
            # include PL events unless already explicitly excluded,
            # anything we don't explicitly include is excluded        
            if not exclude_section:
                for include_term in [
                    "press bench", "bench press", "squat", "deadlift", "powerlifting"
                ]:
                    if include_term in lc_section_header:
                        include_section = True
                        logger.debug(f"Including section with header {section_header} matching term {include_term}")
                        break
        # we're inside a relevant section
        if not new_section_header and include_section:
            # the first line in the section is the column header line
            if not section_entry_header:
                section_entry_header = input_line
                # First 3 fields are blank, but they are:
                # Place, CyrillicName, Sex
                section_entry_header = f"Place,CyrillicName,Sex,{input_line[3:]}"
                logger.debug(f"Got section entry header:{section_entry_header}")
            else:
                # now we're accumulating entries
                section_csv_entry_lines.append(input_line)
                logger.debug("Added entry to current section")

def parse_born(born):
    [orig_dob, age] = born.split("/")
    [dd, mm, yyyy] = orig_dob.split(".")
    opl_dob = f"{yyyy}-{mm}-{dd}"
    return opl_dob, age

def lift(weight):
    return weight if weight != "-" else ""

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} translated_original_csv", file=sys.stderr)
        sys.exit(1)
    input_path_str = sys.argv[1]
    input_dir_path = Path(input_path_str).parent
    meet_path_str = str(input_dir_path / "meet.csv")
    entries_path_str = str(input_dir_path / "entries.csv")

    with open(meet_path_str, "wt") as meet_f:
        # make a meet CSV with data common to all meets.  The rest of
        # the fields will need to be filled in manually from
        # original_translated.csv
        meet_d = {
            "Federation": "NAP", "Date": "", "MeetCountry": "Russia", "MeetState": "",
            "MeetTown": "", "MeetName": ""
        }
        meet_dw = DictWriter(meet_f, meet_d.keys(), lineterminator="\n")
        meet_dw.writeheader()
        meet_dw.writerow(meet_d)
        logger.info("Wrote default meet.csv file, this will need manual filling-in")

    with open(input_path_str, "rt") as input_f:
        with open(entries_path_str, "wt") as entries_f:
            entries_dw = DictWriter(
                entries_f,
                [
                    "Place", "CyrillicName", "Sex", "BirthDate", "Age", "Equipment",
                    "Division", "WeightClassKg", "BodyweightKg", "Squat1Kg",
                    "Squat2Kg", "Squat3Kg", "Squat4Kg", "Best3SquatKg",
                    "Bench1Kg", "Bench2Kg", "Bench3Kg", "Bench4Kg", "Best3BenchKg",
                    "Deadlift1Kg", "Deadlift2Kg", "Deadlift3Kg", "Deadlift4Kg",
                    "Best3DeadliftKg", "TotalKg", "Event"
                ],
                lineterminator="\n"
            )
            entries_dw.writeheader()
            entry_i = None
            for entry_i, entry_d in enumerate(gen_opl_entries(input_f)):
                logger.debug(f"Adding OPL entry:{entry_d}")
                entries_dw.writerow(entry_d)
            entry_count = entry_i + 1 if entry_i else 0
            logger.info(f"Added {entry_count} OPL entries")

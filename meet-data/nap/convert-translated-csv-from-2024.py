import sys

from oplcsv import Csv

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
    if len(input_csv_row < 1):
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
    for input_row in input_csv.rows[2:]:
        new_section_header = get_section_header(input_row)
        if new_section_header:
            # if we had a section CSV from the previous section, yield it now
            if section_csv and section_header:
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
            for exclude_term in [
                "people", "paired", "curl", "military", "russian", "streetlifting",
                "overall", "coaches"
            ]:
                if exclude_term in lc_section_header:
                    exclude_section = True
                    break
            if not exclude_section:
                for include_term in [
                    "press bench", "bench press", "squat", "deadlift", "powerlifting"
                ]:
                    if include_term in lc_section_header:
                        include_section = True
                        break
        # got the section header, next one is CSV header
        # place, name (cyrillic), and sex aren't labelled, but they're the first three
        if include_section and not csv_header_row:
            csv_header_row = ["Place", "CyrillicName", "Sex"] + input_row
            section_csv = Csv()
            section_csv.append_columns(csv_header_row)
        # we're in the body of the section, accumulate rows
        if include_section and section_csv:
            # don't include blank rows
            if not is_blank_row(input_row):
                section_csv.rows.append(input_row)


def make_opl_section_csv(section_csv, section_header):
    #TODO - return a csv with the OPL schema, from the data in the section
    # csv, and section header
    pass

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
    input_csv = Csv(input_path_str)
    entries_csv = Csv()
    for section_csv, section_header in gen_section_csvs(input_csv):
        section_entries_csv = make_opl_section_csv(section_csv, section_header)
        entries_csv.cat(section_entries_csv)
    meet_csv = make_meet_csv()
    #TODO - output entries csv filename is entries.csv in same dir
    #TODO - write meet.csv in same dir
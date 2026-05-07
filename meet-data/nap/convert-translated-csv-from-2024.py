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

def gen_section_csvs(input_csv):
    #TODO - iterate over the rows in the input csv and yield a csv
    # from the rows under each section header, and the section header
    pass

def make_opl_section_csv(section_csv, section_header):
    #TODO - return a csv with the OPL schema, from the data in the section
    # csv, and section header
    pass

def make_meet_csv():
    #TODO - return a meet csv with the data common to all meets
    pass

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
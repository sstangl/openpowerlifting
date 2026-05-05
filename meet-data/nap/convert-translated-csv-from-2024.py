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

#TODO - functions

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} translated_original_csv", file=sys.stderr)
        sys.exit(1)
    input_path_str = sys.argv[1]
    input_csv = Csv(input_path_str)
    #TODO - output entries csv filename is entries.csv in same dir
    #TODO - write a meet.csv with blank fields as appropriate 
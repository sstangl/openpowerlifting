#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Partially populates meet.csv from the original file

import sys
import csv

# Lets us share a codebase for all the feds that use this format
fed_dict = {'INTERNATIONAL POWERLIFTING FEDERATION': 'IPF',
            'EUROPEAN POWERLIFTING FEDERATION': 'EPF'}


# Splits a string by split_char, only splitting if not within brackets -
# for countries like USA
def split_no_brackets(s, split_char):
    brackets = 0
    split_str = []
    curr_str = ''

    for c in s:
        curr_str += c
        if c == '(':
            brackets += 1
        elif c == ')':
            brackets -= 1
        elif c == split_char and brackets == 0:
            split_str.append(curr_str[:-1].strip())
            curr_str = ''

    if len(curr_str):
        split_str.append(curr_str.strip())

    return split_str


def remove_all(s, substrs):
    removed_str = s
    for substr in substrs:
        removed_str = removed_str.replace(substr, "")
    return removed_str


def main(*filenames):

    original_file = open(filenames[0], 'r')
    original_reader = csv.reader(original_file)
    original = list(original_reader)
    original_file.close()

    fed = original[0][0]

    # Try and get the OPL code
    try:
        fed_code = fed_dict[fed.upper()]
    except Exception:
        fed_code = ''

    meet_info = []
    # Have a hunt around for the meet info
    for ii in range(0, 3):
        if len(original[ii]) != 0:
            meet_info = split_no_brackets(original[ii][0], '.')
            if len(meet_info) > 3:  # Looks like we've found the meet info
                break

    # Write the header
    meet_file = csv.writer(sys.stdout, lineterminator="\n")

    fieldnames = []
    fieldnames.append('Federation')
    fieldnames.append('Date')
    fieldnames.append('MeetCountry')
    fieldnames.append('MeetState')
    fieldnames.append('MeetTown')
    fieldnames.append('MeetName')

    meet_file.writerow(fieldnames)

    if len(meet_info) > 3:  # Then this is probably meet information

        if len(meet_info) == 6:  # older meets split up city & country
            meet_info[2] = meet_info[2] + ' ' + \
                '(' + meet_info[1].strip() + ')'
            meet_info.pop(1)

        meet_name = meet_info[0]
        meet_location = meet_info[1].strip()

        # Need to deal with meets that cross over months
        # Date starts from first day
        meet_day = meet_info[2].split('-')[0].strip()
        meet_date = meet_info[-1] + '-' + meet_info[-2] + '-' + meet_day

        meet_country = meet_location[meet_location.find(
            '(') + 1:meet_location.rfind(')')].strip()
        meet_state = ''  # This needs to be done manually
        meet_town = meet_location[0:meet_location.rfind('(')].strip()

        # If more than one original file has been passed, then see if we can
        # extract more information
        if len(filenames) > 1:
            for filename in filenames[1:]:
                original_file = open(filename, 'r')
                original_reader = csv.reader(original_file)
                original = list(original_reader)

                for ii in range(0, 5):
                    meet_info = split_no_brackets(original[ii][0], '.')
                    if len(meet_info) > 3:  # Looks like we've found the meet info
                        break
                if meet_info != []:
                    # Men and womens original are named under different meets
                    # even though they were the same
                    new_meet_name = meet_info[0]
                    if new_meet_name != meet_name:
                        men_text = [" mens", " Mens",
                                    " Men's", " men's", "men"]
                        women_text = [" womens", " Womens",
                                      " Women's", " women's", "women"]
                        if (any(text in meet_name for text in men_text) and
                                any(text in new_meet_name for text in women_text)):
                            meet_name = remove_all(meet_name, men_text)
                        elif (any(text in meet_name for text in women_text) and
                                any(text in new_meet_name for text in men_text)):
                            meet_name = remove_all(meet_name, women_text)
                    # Still need to deal with meets that cross over months
                    new_meet_day = meet_info[2].split(
                        '-')[0].strip()  # Date starts from first day
                    new_meet_date = meet_info[-1] + '-' + \
                        meet_info[-2] + '-' + new_meet_day

                    if new_meet_date < meet_date:
                        meet_date = new_meet_date

                original_file.close()

        meet_file.writerow([fed_code, meet_date, meet_country,
                            meet_state, meet_town, meet_name])


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print(" Usage: %s original.csv" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(*sys.argv[1:])

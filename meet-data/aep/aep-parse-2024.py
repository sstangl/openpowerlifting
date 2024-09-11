#!/usr/bin/python3
import argparse
import requests
from bs4 import BeautifulSoup
import pandas as pd
import dateparser
import os
from math import isnan
import numpy as np


# For checking whether a parameter is a pandas df, let's use some abbreviation
df_type = pd.core.frame.DataFrame
# Same for pandas series
series_type = pd.core.series.Series

# TODO: deal with junior/subjunior extra weightclasses
# Add 43 or 53 for them
weight_class_limits_f = [47, 52, 57, 63, 69, 76, 84]
weight_class_limits_m = [59, 66, 74, 83, 93, 105, 120]

# Helper dict mapping openpowerlifting expected fields and columns in xls('s)
dict_colname_colnum = {
        "Place": 0,
        "Name": 1,
        "BirthYear": 2,
        "Team": 3,
        # "Division": not included, but will be hardcoded to "Open"
        # "Sex": needs to be added manually
        # "WeightClassKg": needs to be added manually
        "BodyweightKg": 4,
        "Squat1Kg": 7,
        "Squat2Kg": 8,
        "Squat3Kg": 9,
        # "Best3SquatKg": needs to be added manually
        "Bench1Kg": 11,
        "Bench2Kg": 12,
        "Bench3Kg": 13,
        # "Best3BenchKg": needs to be added manually
        "Deadlift1Kg": 15,
        "Deadlift2Kg": 16,
        "Deadlift3Kg": 17,
        # "Best3DeadliftKg": needs to be added manually
        "TotalKg": 19,
        # "Goodlift": 20,: would love to keep this field though
        # "Event": look up "POWERLIFTING" in row 5, column 0
        # "Equipment":  look up "RAW" in row 5, column 0
        # "BirthDate": not included
        }


def get_command_line_args():
    """
    Returns the command line arguments
    """
    parser = argparse.ArgumentParser(
        prog='aep-parse-2024.py',
        description='Process xls results from an AEP meet page')
    parser.add_argument('--url', nargs='+', type=str, required=True,
                        help="URL(s) of the meet main page")
    # So far no other arguments needed, but leaving this as an example just in case
    # parser.add_argument('-l', '--lines', type=int, default=10)
    return parser.parse_args()


def get_meet_xls_urls(urls: list[str]):
    """
    Returns a list of urls to the xls files in each of the main page urls
    """
    # args is a list with arguments
    for url in urls:
        response = requests.get(url)
        meet_page = response.text
        soup = BeautifulSoup(meet_page, "html.parser")
        xls_urls = [link.get("href") for link in soup.select(selector="h3>a")
                    if link.get("href").endswith(".xls")]
    return xls_urls


def read_sheet(url: str, sheet_name: str):
    """
    Reads the sheet with sheet_name from the xls file in the url.
    Returns a pandas DF
    """
    return pd.DataFrame(pd.read_excel(url,
                                      sheet_name=sheet,
                                      decimal=',',
                                      header=None,
                                      usecols=dict_colname_colnum.values()))


def get_row_by_index(df: df_type, index: int):
    """
    Return the raw in 'index' place from the dataframe
    Indexes start in 0
    """
    return df.iloc[index]


def check_empty_cell(cell_content):
    """
    Check if a cell is empty.
    As type can either be numeric type or string alike, condition is complex
    """
    if type(cell_content) is np.float64:
        cell_content = float(cell_content)
    if (
            (type(cell_content) is str and cell_content == "") or
            (type(cell_content) is float and isnan(cell_content))):
        return_value = True
    else:
        return_value = False
    return return_value


def get_meet_data(df: df_type):
    """
    Return a dictionary with the required fields regarding the information of a
    meet, such as federation, date, country, state, town and meet name
    """
    date_row = get_row_by_index(df, 3)
    for col_index, cell_content in date_row.items():
        if not (check_empty_cell(cell_content)):
            # Date
            date_string = cell_content.split(' - ')[-1]
            date = dateparser.parse(date_string)
            date_string = date.strftime("%Y-%m-%d")
            # Country/State/Town
            town = cell_content.split(' - ')[0].split(',')[0]
            # Federation
    name_row = get_row_by_index(df, 1)
    for col_index, cell_content in name_row.items():
        if not (check_empty_cell(cell_content)):
            # Name
            name = cell_content
    meet_data = {
            'Federation': 'AEP',  # Hardcoded
            'Date': date_string,
            'MeetCountry': 'Spain',  # Hardcoded
            'MeetState': '',  # Empty
            'MeetTown': town,
            'MeetName': name,
            }
    return meet_data


def get_newdir_paths(year: str):
    """
    Returns the path for the directory to create for the new meet
    """
    # Get absolute path of the script
    script_path = os.path.abspath(os.path.dirname(__file__))
    # Get all directories for the current year
    directories = [file for file in os.listdir(script_path)
                   if os.path.isdir(file) and file.isdigit() and
                   file.startswith(year)]
    # Sort the list, get the last element, and add 1 for the new dir
    newdirname = str(int(sorted(directories)[-1]) + 1)
    # Return the full path of the previous calculation
    return script_path + '/' + newdirname


def get_year_from_date_string(date: str):
    """
    Get the year in 2 digit format from a date string
    Date is separated by dashes, with year in the first field
    """
    return date.split('-')[0][-2:]


def create_meet_dir(directory_path: str):
    """
    Create directory for the meet
    """
    os.makedirs(directory_path)


def write_meet_csv(meet_data: dict, filename: str):
    """
    Write the meet data info from the dictionary into a csv file
    """
    with open(filename, "w", encoding='utf-8') as file:
        # Write header
        header = ','.join(meet_data.keys()) + '\n'
        file.write(header)
        # Write data
        row = ','.join(str(meet_data[key]) for key in meet_data.keys()) + '\n'
        file.write(row)


def write_url_file(url: str, filename: str):
    """
    Write the URL file containing the url of the meet
    """
    with open(filename, "w") as file:
        file.write(url)


def name_columns(df: df_type, col_dict: dict):
    """
    Renames the columns of the DF according to the rules in the dict
    """
    i = 0
    keys = list(col_dict.keys())
    columns = list(df.columns)
    for column in columns:
        df.rename(columns={column: keys[i]}, inplace=True)
        i += 1


def populate_meet_equipment_types(df: df_type, meet_types: dict, equipment_types: dict):
    """
    Populates the two dictionaries with the type of meet & equipment.
    Dict keys are the names of the sheet.
    Values are the expect values for such dictionary:
    - 'SBD' or 'B' for the meet_types (meets taking place in AEP)
    - 'Raw' or 'Single-ply' for the equipment_types (again, for AEP specific
      case)
    """
    # The sixth raw of the xls file contains a string to identify everything
    # Number 5 because it is 0-indexed
    meet_sheet_row_index = 5
    # Info seems to be in row 5, column 0 (indexed to 0)
    # Just to play safe, let's loop through all row 5
    # row = df.iloc[meet_sheet_row_index]
    row = get_row_by_index(df, meet_sheet_row_index)
    for col_index, cell_content in row.items():
        if not (check_empty_cell(cell_content)):
            if 'powerlifting' in cell_content.lower():
                meet_types[sheet] = 'SBD'
            elif 'banca' in cell_content.lower():
                meet_types[sheet] = 'B'
            if 'raw' in cell_content.lower():
                equipment_types[sheet] = 'Raw'
            elif 'equipado' in cell_content.lower():
                # According to AEP rulebook, suits are single-ply
                equipment_types[sheet] = 'Single-ply'


def remove_duplicate_names(df: df_type):
    """
    Removes duplicates from the DF
    In the end of the sheet, all weight categories get sorted by ipfgl
    This generates duplicates.
    Moreover, that trailing entries don't have all fields.
    In addition, existing fields are in a different order
    The only useful info here is the weight category
    And anyway, we're generating that manually
    This way we avoid dealing with duplicates
    """
    # Column 1 is lifter's name
    liftername_column = 1
    df.drop_duplicates(subset=dfs[sheet].columns[liftername_column],
                       inplace=True, ignore_index=True)


def remove_nondata_rows(df: df_type):
    """
    Remove senseless entries by checking if there's a year in the year column
    Convert BirthYear to numeric and also reset indexes
    """
    dfs[sheet] = dfs[sheet][pd.to_numeric(
        dfs[sheet]['BirthYear'],
        errors='coerce').notnull()].reset_index(drop=True)


def switch_firstname_order_to_beginning(fullname: str):
    """
    Switches the first name from the end of the string to the beginning.
    It contains a lot of logic for corner cases. It's been tested for most
    Spanish names. It also works with British/American names, even if including
    the middle name with a dot. It won't work for full American names, with the
    middle name in full, without a dot, but that was not easily fixable. I hope
    to be right not to expect Americans signing up for a meet with the full
    middle name, and in any case, hopefully not so many for a Spanish meet.
    Happy to get corrections.
    Parameter: a name string with the first name in the end, can be a
    compounded name.
    Returns: a name string with the first name at the beginning.
    """
    words_for_compounds = ['da', 'de', 'del', 'la', 'las', 'los', 'mac', 'mc', 'van',
                           'von', 'y', 'i', 'san', 'santa']
    # Split full name into words
    fullname_words = fullname.split(' ')

    # Identify if the last component is a compound name
    def is_compound_name(name_parts):
        return len(name_parts) > 1 and all(part.lower() not in words_for_compounds
                                           for part in name_parts)

    # Handle compound surnames and names
    processed_name = []
    prev_word = ""
    for word in fullname_words:
        if word.lower() in words_for_compounds:
            if processed_name:
                prev_word += processed_name.pop() + ' ' + word + ' '
            else:
                prev_word += word + ' '
        else:
            processed_name.append(prev_word.strip() + ' ' + word if prev_word else word)
            prev_word = ""

    # Check if the last part is a compound name
    if processed_name:
        if is_compound_name(processed_name[-1].split()):
            # Swap first name and last part
            first_name = processed_name.pop(0)
            processed_name.append(first_name)

    # Special handling for initials
    processed_name_with_initials = []
    for part in processed_name:
        if len(part) == 2 and part[1] == '.':
            if processed_name_with_initials:
                processed_name_with_initials[-1] += ' ' + part
            else:
                processed_name_with_initials.append(part)
        else:
            processed_name_with_initials.append(part)

    # If there is one first name and one surname, just reverse them
    if len(processed_name_with_initials) == 2:
        processed_name_with_initials = list(reversed(processed_name_with_initials))

    # For other cases, adjust positions accordingly
    elif len(processed_name_with_initials) >= 3:
        for i in range(len(processed_name_with_initials) - 2):
            processed_name_with_initials.insert(0, processed_name_with_initials.pop())

    return " ".join(processed_name_with_initials)


def process_names_in_df(df: df_type):
    df["Name"] = df["Name"].apply(switch_firstname_order_to_beginning)


def insert_fixed_column_into_df(df: df_type, position: int, col_name: str, value):
    """
    Insert a column into the df in "position" with some common value
    """
    dfs[sheet].insert(position, col_name, value)


def populate_sex_weightclass(df: df_type, sheet_name: str):
    """
    Add M/F/MX as Sex field, position 5 -> No idea how AEP deals with Mx sex
    Add WeightClassKg field, position 6
    This field should be a string, because of m120+ and f84+ exceptions
    """
    if "fem" in sheet_name.lower() or "muj" in sheet_name.lower():
        insert_fixed_column_into_df(df, 5, 'Sex', 'F')
        insert_fixed_column_into_df(df, 6, 'WeightClassKg',
                                    [str(weight_class_limits_f[0])
                                     if weight <= weight_class_limits_f[0]
                                     else (str(weight_class_limits_f[1]))
                                     if weight <= weight_class_limits_f[1]
                                     else (
                                         str(weight_class_limits_f[2]))
                                     if weight <= weight_class_limits_f[2]
                                     else (
                                         str(weight_class_limits_f[3]))
                                     if weight <= weight_class_limits_f[3]
                                     else (
                                         str(weight_class_limits_f[4]))
                                     if weight <= weight_class_limits_f[4]
                                     else (
                                         str(weight_class_limits_f[5]))
                                     if weight <= weight_class_limits_f[5]
                                     else (
                                         str(weight_class_limits_f[6]))
                                     if weight <= weight_class_limits_f[6]
                                     else str(weight_class_limits_f[6]) + "+"
                                     for weight in dfs[sheet]['BodyweightKg']]
                                    )
    elif "mas" in sheet_name.lower() or "hom" in sheet_name.lower():
        insert_fixed_column_into_df(df, 5, 'Sex', 'M')
        insert_fixed_column_into_df(df, 6, 'WeightClassKg',
                                    [str(weight_class_limits_m[0])
                                     if weight <= weight_class_limits_f[0]
                                     else (
                                         str(weight_class_limits_m[1]))
                                     if weight <= weight_class_limits_m[1]
                                     else (
                                         str(weight_class_limits_m[2]))
                                     if weight <= weight_class_limits_m[2]
                                     else (
                                         str(weight_class_limits_m[3]))
                                     if weight <= weight_class_limits_m[3]
                                     else (
                                         str(weight_class_limits_m[4]))
                                     if weight <= weight_class_limits_m[4]
                                     else (
                                         str(weight_class_limits_m[5]))
                                     if weight <= weight_class_limits_m[5]
                                     else (
                                         str(weight_class_limits_m[6]))
                                     if weight <= weight_class_limits_m[6]
                                     else str(weight_class_limits_m[6]) + "+"
                                     for weight in dfs[sheet]['BodyweightKg']]
                                    )


def populate_bestlifts_fields(df: df_type):
    """
    Check each lift and select the best one (max) of each in a right new field
    """
    # Fields in which the new fields will be inserted into the DF
    dict_best_lifts = {
            "Best3SquatKg": 11,
            "Best3BenchKg": 15,
            "Best3DeadliftKg": 19,
            }

    for name, position in dict_best_lifts.items():
        columns = list(df.columns)
        # convert to numeric the columns we'll be working with
        for i in range(3, 0, -1):
            df[columns[position-i]] = pd.to_numeric(df[columns[position-i]],
                                                    errors="coerce")
            # Skipped lifts (NaN) are already properly written in the csv
            # No further action needed

        # Insert the column
        insert_fixed_column_into_df(df, position, name,
                                    df.iloc[:, position-3:position].max(axis=1))


def col_tonumeric(df: df_type, colname: str):
    """
    Convert the column of the DF to numeric. Selected by name.
    """
    df[colname] = pd.to_numeric(df[colname])


def process_totalkg(df: df_type):
    """
    Deal with TotalKg not being a number sometimes
    """
    # Check Place, look for "--" for DQ lifters (squat drop, 3 failed lifts...)
    # In addition, for such lifters, total should be empty (NaN)
    df.loc[df.Place == '--', ['Place', 'TotalKg']] = ['DQ', float(np.nan)]
    # Now it is possible to convert TotalKg into a float
    df['TotalKg'] = pd.to_numeric(df['TotalKg'])


def remove_unneeded_trailing_decimals(df: df_type):
    """
    Remove .0 from the numeric columns
    """
    pd.options.display.float_format = '{:g}'.format


def df_to_csv(df: df_type, filename: str):
    """
    Write the entries from the df into a csv
    Using (default) write mode to overwrite the file
    """
    df.to_csv(filename, index=False, mode='w', float_format='{:g}'.format)


# When run at the command line with python "top.py --lines=5 --urls url1 url2"
# the script sets args.lines=5 and args.urls=['url1', 'url2']
# args is a list with console arguments
# args.urls is a list of url's passed in --urls parameter
args = get_command_line_args()

# Check the list is not empty
# if args.urls is not None:

# Get the general url of an AEP meet from the console arguments
xls_url = get_meet_xls_urls(args.url)[0]

# Load the excelfile
xl = pd.ExcelFile(xls_url)

sheets = []
meet_types = {}
equipment_types = {}
dfs = {}

# Loop through the multiple sheets (usually, men/women = 2 sheets)
# But there could be more: bench only, raw/equipped...
for sheet in xl.sheet_names:

    # Get the sheet into a pandas DF
    dfs[sheet] = read_sheet(xls_url, sheet)

    # Get meet info
    # This info gets overwritten with each sheet, but should remain the same
    meet_data = get_meet_data(dfs[sheet])

    # Name columns, as they don't have a name
    name_columns(dfs[sheet], dict_colname_colnum)

    # Fill in the values for the required dicts about meet/equipment types
    populate_meet_equipment_types(dfs[sheet], meet_types, equipment_types)

    # Remove duplicates (because of the bottom "best lifters ranking")
    remove_duplicate_names(dfs[sheet])

    # Remove senseless entries (checking year column)
    remove_nondata_rows(dfs[sheet])

    # Process name field, moving name from the end to the beginning
    process_names_in_df(dfs[sheet])

    # Add Open as a Division field, position 4
    insert_fixed_column_into_df(dfs[sheet], 4, 'Division', 'Open')

    # Populate sex and weightclass fields
    populate_sex_weightclass(dfs[sheet], sheet)

    # Add S/B/D best as a number in fields 11, 15, 19
    populate_bestlifts_fields(dfs[sheet])

    # Convert bodyweight column into a number (float)
    col_tonumeric(dfs[sheet], "BodyweightKg")

    # Deal with TotalKg not being a number sometimes
    process_totalkg(dfs[sheet])

    # Add the right type of Event (pos 21) according to the value read in the sheet
    insert_fixed_column_into_df(dfs[sheet], 21, 'Event',
                                meet_types[sheet])

    # Add the right type of Equipment (pos 22) according to the value read in the sheet
    insert_fixed_column_into_df(dfs[sheet], 22, 'Equipment',
                                equipment_types[sheet])

    # "BirthDate": not included -> position 23 -> needs to be an empty field
    # Add BirthDate as an empty field (pos 23) as it seems to be required
    insert_fixed_column_into_df(dfs[sheet], 23, 'BirthDate', None)

    # Process numeric columns to remove unwanted trailing .0
    remove_unneeded_trailing_decimals(dfs[sheet])

# Combine all df's in one
meet_entries = pd.concat(dfs)

# Print the output to console before writing it to csv's
print(meet_entries.to_string(index=False))
print(meet_data)

# Get the year of the meet in two digit format
year_two_digits = get_year_from_date_string(meet_data["Date"])

# Get new program directory absolute path
directory_path = get_newdir_paths(year_two_digits)

# Create that directory
create_meet_dir(directory_path)

# GENERATE THE CSVs!

# Entries
df_to_csv(meet_entries, directory_path + '/' + 'entries.csv')

# Meet data
write_meet_csv(meet_data, directory_path + '/' + 'meet.csv')

# Write URL file
write_url_file(xls_url, directory_path + '/' + 'URL')

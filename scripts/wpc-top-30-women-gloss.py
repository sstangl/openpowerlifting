#!/usr/bin/env python

import sys

from csv import DictReader


def get_meet_data(meet_file_path):

    wpc_affiliate_list = [
        'WPC',
        'APF',
        'BPU',
        'CAPO',
        'CPC',
        'CPO',
        'HPC',
        'WPCCP',
        'WPC',
        'WPC-Egypt',
        'WPC-Finland',
        'WPC-France',
        'WPC-Germany',
        'WPC-Iceland',
        'WPC-India',
        'WPC-Israel',
        'WPC-Italy',
        'WPC-KAZ',
        'WPC-KGZ',
        'WPC-Latvia',
        'WPC-Moldova',
        'WPC-Portugal',
        'WPC-Poland',
        'WPC-RUS',
        'WPC-SA',
        'WPC-SVK',
        'WPC-UKR',
    ]

    meet_data_dict = dict()

    with open(meet_file_path, 'rt') as meet_f:
        for meet_row in DictReader(meet_f):
            if meet_row['Date'].startswith('2022-') and \
               meet_row['Federation'] in wpc_affiliate_list:
                meet_data_dict[meet_row['MeetID']] = meet_row

    return meet_data_dict


def get_lifter_data(lifter_file_path):

    lifter_data_dict = dict()

    with open(lifter_file_path, 'rt') as lifter_f:
        for lifter_row in DictReader(lifter_f):
            lifter_data_dict[lifter_row['LifterID']] = lifter_row

    return lifter_data_dict


def get_augment_entries(entry_file_path, meet_data_dict, lifter_data_dict):

    # key by lifter ID so we get each lifter's best
    entry_dict = dict()

    with open(entry_file_path, 'rt') as entry_f:
        for entry_row in DictReader(entry_f):
            if all([
                entry_row['MeetID'] in meet_data_dict,
                entry_row['Sex'] == 'F',
                entry_row['TotalKg'] != '',
                entry_row['BodyweightKg'] != '',
            ]):
                lifter_id = entry_row['LifterID']
                meet_id = entry_row['MeetID']
                entry_row['Name'] = lifter_data_dict[lifter_id]['Name']
                entry_row['Date'] = meet_data_dict[meet_id]['Date']
                entry_row['Federation'] = meet_data_dict[meet_id]['Federation']
                entry_row['MeetCountry'] = meet_data_dict[meet_id]['MeetCountry']
                entry_row['MeetName'] = meet_data_dict[meet_id]['MeetName']

                if (not entry_dict.get(lifter_id)) or \
                   (
                    float(entry_row['Glossbrenner']) >
                    float(entry_dict[lifter_id]['Glossbrenner'])
                   ):
                    entry_dict[lifter_id] = entry_row

    return entry_dict


if __name__ == '__main__':
    meet_data_dict = get_meet_data(sys.argv[1])
    lifter_data_dict = get_lifter_data(sys.argv[2])
    entry_dict = get_augment_entries(sys.argv[3], meet_data_dict, lifter_data_dict)
    for (lifter_id, entry,) in sorted(
        entry_dict.items(),
        key=lambda i: float(i[1]['Glossbrenner']),
        reverse=True
    )[:30]:
        print(entry['Name'], entry['Date'], entry['Federation'],
              entry['MeetCountry'], entry['MeetName'],
              entry['TotalKg'], entry['BodyweightKg'],
              entry['Glossbrenner'], entry['Equipment'])

#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# Usage: gitlab-import <USPA issue number>
#
# The USPA files issues with us in a standard format: structured text plus XLSX.
# This script uses the `glab` CLI tool to automate repetitive actions when dealing
# with USPA data, allowing for faster importation.

import subprocess
import sys
import os

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv

###############################################################################
# The GitLab issue number is taken as an argument.
###############################################################################

if len(sys.argv) != 2:
    print(" Usage: gitlab-import <USPA issue number>")
    sys.exit(1)
issue = sys.argv[1]

###############################################################################
# Use the `glab` CLI to download the issue text from GitLab.
###############################################################################

# Connect to GitLab and download the textual description of the issue.
res = subprocess.run(("glab issue view " + issue).split(), capture_output=True)
assert res.returncode == 0
body = res.stdout.decode('utf-8')

# GitLab responds with some textual headers, which look like the following:
# ```
# title:	USPA Drug Tested North Dakota Open
# state:	open
# author:	support-bot
# labels:
# comments:	0
# assignees:
# --
# ```
# Use this to extract the title, like "USPA Drug Tested North Dakota Open".
title = body.split('\n')[0].split('\t')[1]
# Split into federation and MeetName: (USPA, Drug Tested North Dakota Open).
(federation, meetname) = (title.split()[0], ' '.join(title.split()[1:]))

# Look at the body to determine location and date information, for the meet.csv.
# From the above, the body looks like this:
# ```
# --
# Hi All,
#
#
#
# The Excel score sheet for the USPA Drug Tested North Dakota Open meet held
# on October 2, 2022 in Fargo, ND is attached. The PDF can be found on the USPA
# Competition Results Page <https://uspa.net/past-events/>.
#
#
#
# Tested lifters: Tyra Allen, Edwad Maisey, Caden Schoffelman, Evan Counts,
# Blessed Vargas, David Koch
# ```
# So we can extract this information by splitting into paragraphs.
paragraphs = body.replace("\n\n\n", "\n\n").replace("\n\nHi ", "\nHi").split("\n\n")
info = ' '.join(paragraphs[2].split('\n')).strip()  # Compress into one line.

# Extract out from "... meet held on October 2, 2022 in Fargo, ND is attached ..."
# Sometimes the "m" in "meet" is capitalized, so we just ignore that character.
info = info.split('eet held on')[1].strip()
info = info.split('is attached')[0].strip()  # "October 2, 2022 in Fargo, ND"
info = info.replace('  ', ' ')  # Dates are typed by hand.

(human_date, location) = info.split(" in ")
(meettown, meetstate) = location.split(",")

# Map the human date to its start date. It may look like "October 2-3, 2022".
(month, day, year) = human_date.split(' ')
months = {
    "January": "01",
    "February": "02",
    "March": "03",
    "April": "04",
    "May": "05",
    "June": "06",
    "July": "07",
    "August": "08",
    "September": "09",
    "October": "10",
    "November": "11",
    "December": "12"
}
month = months[month]

day = day.strip(',')
if '-' in day:
    day = day.split('-')[0]
if len(day) == 1:
    day = "0" + day

###############################################################################
# Determine the target directory name and chdir into it.
###############################################################################

# USPA meets are numbered sequentially. Numbers <= 2565 were assigned by the USPA.
# At that point, they redesigned their website and made those numbers difficult to find.
# So now, we just make up our own by adding one each time.
res = subprocess.run("ls | sort -n | tail -n 1", shell=True, capture_output=True)
folder_id = int(res.stdout.decode('utf-8').strip()) + 1
os.system("mkdir " + str(folder_id))
os.chdir(str(folder_id))
print(" importing into " + str(folder_id))

###############################################################################
# Get the URL of the XLSX attachment and download it.
###############################################################################

# The spreadsheet itself is an attachment at the end of the message, like:
# [USPA_Example.xlsx](/uploads/.../USPA_Example.xlsx)
attachments = [x for x in body.split('\n') if x.find("/uploads/") >= 0]
assert len(attachments) == 1
attachment = attachments[0]
# Get just the URL part, in between () brackets.
xlsx_url = attachment.split('](')[1].strip(')')
# Make it non-relative.
xlsx_url = "https://gitlab.com/openpowerlifting/opl-data" + xlsx_url
xlsx_filename = xlsx_url.split('/')[-1]

# Download the spreadsheet.
res = subprocess.run(["wget", xlsx_url])
assert res.returncode == 0

###############################################################################
# Write the meet.csv.
###############################################################################

meetcsv = Csv()
meetcsv.append_columns(["Federation", "Date", "MeetCountry",
                        "MeetState", "MeetTown", "MeetName"])
meetcsv.rows = [[""] * len(meetcsv.fieldnames)]

meetrow = meetcsv.rows[0]
meetrow[meetcsv.index('Federation')] = federation.strip()
meetrow[meetcsv.index('Date')] = year + "-" + month + "-" + day
meetrow[meetcsv.index('MeetCountry')] = "USA"
meetrow[meetcsv.index('MeetState')] = meetstate.strip()
meetrow[meetcsv.index('MeetTown')] = meettown.strip()
meetrow[meetcsv.index('MeetName')] = meetname.strip()

meetcsv.write_filename("meet.csv")

###############################################################################
# Run our XLSX importation scripts.
###############################################################################

res = subprocess.run(["../uspa-xlsx.sh", xlsx_filename])
if res.returncode == 0:
    os.system("rm " + xlsx_filename)

# If this is a tested meet, insert a tested column.
if 'Drug Tested' in meetname:
    csv = Csv("entries.csv")
    csv.append_column('Tested')
    for row in csv.rows:
        row[csv.index('Tested')] = 'Yes'
    csv.write_filename("entries.csv")
    csv = None

###############################################################################
# Leave the user with some helpful output.
###############################################################################

print(" imported into " + str(folder_id))

#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# SPF finally updated their website and started reporting results using
# a standardized table instead of freeform text! So now it's easier to import
# future results.

from bs4 import BeautifulSoup
import os
import sys
import urllib.request

try:
    from oplcsv import Csv
except ImportError:
    sys.path.append(os.path.join(os.path.dirname(os.path.dirname(
        os.path.dirname(os.path.realpath(__file__)))), "scripts"))
    from oplcsv import Csv


def gethtml(url):
    with urllib.request.urlopen(url) as r:
        return r.read()


def error(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


# Mapping from SPF-labeled columns to our labels.
columnmap = {
    "Name": "Name",
    "Division": "Division",
    "Equipment": "Equipment",
    "Class": "WeightClassLBS",
    "Bodyweight": "BodyweightLBS",
    "Event": "Event",
    "Squat": "Best3SquatLBS",
    "Bench": "Best3BenchLBS",
    "Deadlift": "Best3DeadliftLBS",
    "Total": "TotalLBS"
}


def gettable(soup):
    # The page isn't really standardized, but make sure that we understand all
    # the headers.
    tables = soup.find_all("table", {"class": "tablesorter"})
    if len(tables) == 0:
        error("Table not found.")
    if len(tables) > 1:
        error("More than one table found.")
    table = tables[0]
    thead = table.find('thead')
    tbody = table.find('tbody')

    ths = thead.find_all('th')
    headers = []

    csv = Csv()

    for header in ths:
        text = header.text.strip()
        headers.append(text)
        csv.append_column(text)

    for row in tbody.find_all('tr'):
        tds = row.find_all('td')
        r = []
        for td in tds:
            r.append(td.text.strip().replace('  ', ' '))
        csv.rows.append(r)

    return csv


def fixcolumnnames(csv):
    for i, x in enumerate(csv.fieldnames):
        csv.fieldnames[i] = columnmap[x]


# SPF doesn't have a column for Sex, but puts "Women" in the Division.
def sexfromdivision(csv):
    assert "Division" in csv.fieldnames
    assert "Sex" not in csv.fieldnames

    csv.append_column("Sex")
    sexidx = csv.index("Sex")
    dividx = csv.index("Division")

    for row in csv.rows:
        # Fill in the sex column.
        if 'women' in row[dividx].lower():
            row[sexidx] = 'F'
        else:
            row[sexidx] = 'M'

        # Also remove sex information from the Division column.
        row[dividx] = row[dividx].replace("Womens", "").strip()


def fixevent(csv):
    idx = csv.index("Event")

    for row in csv.rows:
        if row[idx] == 'Full Powerlifting':
            row[idx] = 'SBD'
        elif row[idx] == 'Squat Only':
            row[idx] = 'S'
        elif row[idx] == 'Bench Press Only':
            row[idx] = 'B'
        elif row[idx] == 'Deadlift Only':
            row[idx] = 'D'
        elif row[idx] == 'Push/Pull':
            row[idx] = 'BD'
        elif row[idx] == 'Bench for Reps':
            row[idx] = 'reps-deleteme'
        elif row[idx] == 'Strict Curls':
            row[idx] = 'curls-deleteme'
        else:
            row[idx] = '???'


# Depends on fixevent(), so that bench-only isn't marked Wraps.
def fixequipment(csv):
    eqidx = csv.index("Equipment")
    evtidx = csv.index("Event")

    for row in csv.rows:
        eq = row[eqidx]
        if eq == "Raw – No Wraps":
            row[eqidx] = 'Raw'
        elif eq == "Raw":
            if 'S' in row[evtidx]:
                row[eqidx] = 'Wraps'
            else:
                row[eqidx] = 'Raw'
        elif eq == "Multi-Ply":
            row[eqidx] = "Multi-ply"
        elif eq == "Single-Ply":
            row[eqidx] = "Single-ply"
        elif eq in ["Slingshot", "Unlimited"]:
            row[eqidx] = "Unlimited"
        elif eq == "Straps":
            row[eqidx] = "Straps"
        elif eq == '':
            pass  # This means copy from the row above, usually.
        else:
            error("Unknown equipment: %s" % row[eqidx])


def fixdivision(csv):
    dividx = csv.index("Division")

    for row in csv.rows:
        div = row[dividx]
        if div == "Sub-Masters":
            row[dividx] = "Submasters"


def main(url):
    html = gethtml(url)
    soup = BeautifulSoup(html, 'html.parser')

    csv = gettable(soup)
    fixcolumnnames(csv)
    sexfromdivision(csv)

    fixevent(csv)
    fixequipment(csv)
    fixdivision(csv)

    csv.append_column('BirthDate')

    with open('entries.csv', 'w') as fd:
        csv.write(fd)
    with open('URL', 'w') as fd:
        print(url, file=fd)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print("Usage: %s url" % sys.argv[0])
    main(sys.argv[1])

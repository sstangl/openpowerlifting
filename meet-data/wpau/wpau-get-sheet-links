#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# With results.html downloaded from the WPA-UKR website,
# get the sheet paths and write them to standard out
#

from bs4 import BeautifulSoup
import sys


def error(msg):
    print("Error: %s" % msg, file=sys.stderr)
    sys.exit(1)


def main(filename):
    with open(filename, 'r') as fd:
        html = fd.read()

    soup = BeautifulSoup(html, 'html.parser')

    # Get the path to the sheets
    sheet_links = soup.find_all("link", {"id": "shLink"})
    sheet_links = [lnk['href'] for lnk in sheet_links if lnk.has_attr('href')]

    # Now get the name of the sheets
    jscript = soup.find("script", {"language": "JavaScript"})

    sheet_names = [line for line in str(jscript).split("\n")
                   if " c_rgszSh[" in line]
    sheet_names = [name[name.find(
        '"') + 1:name.rfind('"')].replace('\xa0', ' ') for name in sheet_names]

    for ii in range(len(sheet_links)):
        print(sheet_names[ii])
        print(sheet_links[ii])


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s results.html" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])

#!/usr/bin/env python3
# vim: set ts=8 sts=4 et sw=4 tw=99:
#
# With results.html downloaded from the RPS website,
# extract only the results table, and write it to
# stdout.
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

    tables = soup.find_all('table', {'class': 'resultsTable'})
    if len(tables) < 1:
        error("No tables found in the HTML.")

    for table in tables:
        print(table)


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(" Usage: %s results.html" % sys.argv[0], file=sys.stderr)
        sys.exit(1)
    main(sys.argv[1])

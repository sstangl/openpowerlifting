#!/bin/bash
# Main driver for parsing an XLSX file into entries.csv.
# Steve Denison sends in these files. Don't know why they don't just upload
# them to the website, but it skips all the PDF -> CSV parsing fuzziness.

set -e

if [ $# -ne 1 ]; then
	echo " Usage: $0 results.xlsx"
	exit 1
fi

SCRIPTDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd)"
REPOSCRIPTDIR="${SCRIPTDIR}/../../scripts"

# Convert the XLSX to CSV. Note that this only works if it has a ".xls" extension.
if [ "$1" != "original.xls" ]; then
	cp -f "$1" original.xls
fi
LANG=en_US.UTF-8 libreoffice --headless --convert-to csv original.xls
rm -f original.txt

# Parse the CSV to our entries.csv format.
${SCRIPTDIR}/uspa-xlsx-csv-to-final.py original.csv > entries.csv
${REPOSCRIPTDIR}/calc-best-lifts entries.csv
if [ "$1" != "original.xls" ]; then
	rm -f original.xls
fi

#!/bin/bash
#
# USPA submits results with separate Tested and Open spreadsheets,
# but they should be combined. This script automates the steps of combining them.
#
# It should be executed within a new, empty directory.

set -e

if [ $# -ne 2 ]; then
  echo "Usage: $0 <tested-link> <open-link>"
  exit 1
fi

USPADIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd)"
REPOSCRIPTDIR="${USPADIR}/../../scripts"

mkdir tested open

cd tested
wget "$1"
${USPADIR}/uspa-xlsx *.xlsx
${USPADIR}/uspa-mark-tested.py entries.csv
cd ..

cd open
wget "$2"
${USPADIR}/uspa-xlsx *.xlsx
cd ..

${REPOSCRIPTDIR}/csv-cat tested/entries.csv open/entries.csv > entries.csv

mv tested/original.csv original1.csv
mv open/original.csv original2.csv
cp open/meet.csv meet.csv

rm -rf tested open

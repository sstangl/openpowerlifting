.PHONY: builddir csvfile check

DATADIR = meet-data
BUILDDIR = build

PLFILE = openpowerlifting.csv
JSFILE = openpowerlifting.js
MEETFILE = meets.csv

all: csvfile web

builddir:
	mkdir -p '${BUILDDIR}'

# Cram all the data into a single, huge CSV file.
csvfile: builddir
	scripts/compile "${BUILDDIR}" "${DATADIR}"
	scripts/csv-rmcol "${BUILDDIR}/${PLFILE}" Team School Country-State Country College/University Category State
	scripts/csv-wilks "${BUILDDIR}/${PLFILE}"

web: csvfile
	scripts/csv-to-js "${BUILDDIR}/${PLFILE}" opldb > "${BUILDDIR}/${JSFILE}"

# Make sure that all the fields in the CSV files are in expected formats.
check:
	find '${DATADIR}' -name lifters.csv -exec 'scripts/check-lifters-csv' '{}' ';'
	find '${DATADIR}' -name meet.csv -exec 'scripts/check-meet-csv' '{}' ';'

clean:
	rm -rf '${BUILDDIR}'
	rm -rf 'scripts/__pycache__'

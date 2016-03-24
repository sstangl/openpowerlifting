.PHONY: builddir csvfile check

DATADIR = meet-data
BUILDDIR = build

PLFILE = openpowerlifting.csv

all: csvfile

builddir:
	mkdir -p '${BUILDDIR}'

# Cram all the data into a single, huge CSV file.
csvfile: builddir
	find '${DATADIR}' -name lifters.csv -print0 | xargs -0 'scripts/csv-cat' > "${BUILDDIR}/${PLFILE}"
	scripts/csv-rmcol "${BUILDDIR}/${PLFILE}" Team School Country-State Country College/University Category State
	scripts/calculate-wilks "${BUILDDIR}/${PLFILE}"

# Make sure that all the fields in the CSV files are in expected formats.
check:
	find '${DATADIR}' -name lifters.csv -exec 'scripts/check-lifters-csv' '{}' ';'
	find '${DATADIR}' -name meet.csv -exec 'scripts/check-meet-csv' '{}' ';'

clean:
	rm -rf '${BUILDDIR}'
	rm -rf 'scripts/__pycache__'

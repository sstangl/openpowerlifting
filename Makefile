.PHONY: builddir csvfile check

DATADIR = meet-data
BUILDDIR = build

PLFILE = openpowerlifting.csv
PLFILEJS = openpowerlifting.js
MEETFILE = meets.csv
MEETFILEJS = meets.js

all: csvfile web

builddir:
	mkdir -p '${BUILDDIR}'

# Cram all the data into a single, huge CSV file.
csvfile: builddir
	scripts/compile "${BUILDDIR}" "${DATADIR}"
	scripts/csv-rmcol "${BUILDDIR}/${PLFILE}" Team School Country-State Country College/University Category State BirthYear
	scripts/csv-rmcol "${BUILDDIR}/${PLFILE}" Squat1Kg Squat2Kg Squat3Kg Bench1Kg Bench2Kg Bench3Kg Deadlift1Kg Deadlift2Kg Deadlift3Kg Event
	scripts/csv-bodyweight "${BUILDDIR}/${PLFILE}"
	scripts/csv-wilks "${BUILDDIR}/${PLFILE}"

web: csvfile
	$(MAKE) -C web

# Make sure that all the fields in the CSV files are in expected formats.
check:
	scripts/check-lifters-csv
	scripts/check-meet-csv

clean:
	rm -rf '${BUILDDIR}'
	rm -rf 'scripts/__pycache__'
	rm -rf 'meet-data/rps/__pycache__'
	rm -rf 'meet-data/usapl/__pycache__'
	$(MAKE) -C web clean

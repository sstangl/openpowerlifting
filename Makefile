.PHONY: builddir csvfile benchdata check probe travis

DATADIR := meet-data
BUILDDIR := build

PLFILE := openpowerlifting.csv
PLFILEJS := openpowerlifting.js
MEETFILE := meets.csv
MEETFILEJS := meets.js

all: csvfile web

builddir:
	mkdir -p '${BUILDDIR}'

# Cram all the data into a single, huge CSV file.
csvfile: builddir
	scripts/compile "${BUILDDIR}" "${DATADIR}" "lifter-data"
	scripts/csv-bodyweight "${BUILDDIR}/${PLFILE}"
	scripts/csv-wilks "${BUILDDIR}/${PLFILE}"

# Generate a large amount of test data: entries.csv with > 10million entries.
benchdata: builddir
	mkdir -p "${BUILDDIR}/bench-data"
	scripts/compile-for-benchmarking "${BUILDDIR}/bench-data" "${DATADIR}" "lifter-data"
	scripts/csv-bodyweight "${BUILDDIR}/bench-data/${PLFILE}"
	scripts/csv-wilks "${BUILDDIR}/bench-data/${PLFILE}"

# Optionally build an SQLite3 version of the database.
sqlite: csvfile
	scripts/prepare-for-sqlite
	scripts/compile-sqlite

web: csvfile
	$(MAKE) -C web

# Make sure that all the fields in the CSV files are in expected formats.
check:
	tests/check-meet-csv
	tests/check-entries-csv
	tests/check-sex-consistency
	tests/check-lifter-data
	tests/check-duplicates
	tests/check-python-style

# Run all probes in a quick mode that only shows a few pending meets.
probe-quick:
	find "${DATADIR}" -name "*-probe" | sort | parallel --keep-order --will-cite "{} --quick"

# Run all probes.
probe:
	find "${DATADIR}" -name "*-probe" | sort | parallel --keep-order --will-cite

clean:
	rm -rf '${BUILDDIR}'
	rm -rf 'scripts/__pycache__'
	rm -rf 'tests/__pycache__'
	rm -rf '${DATADIR}/apf/__pycache__'
	rm -rf '${DATADIR}/cpu/__pycache__'
	rm -rf '${DATADIR}/ipf/__pycache__'
	rm -rf '${DATADIR}/nasa/__pycache__'
	rm -rf '${DATADIR}/nipf/__pycache__'
	rm -rf '${DATADIR}/nsf/__pycache__'
	rm -rf '${DATADIR}/pa/__pycache__'
	rm -rf '${DATADIR}/rps/__pycache__'
	rm -rf '${DATADIR}/spf/__pycache__'
	rm -rf '${DATADIR}/thspa/__pycache__'
	rm -rf '${DATADIR}/usapl/__pycache__'
	rm -rf '${DATADIR}/wrpf/__pycache__'
	$(MAKE) -C server clean
	$(MAKE) -C web clean

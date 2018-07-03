.PHONY: builddir csv sqlite check probe-quick probe deploy clean

DATADIR := meet-data
BUILDDIR := build

PLFILE := entries.csv
PLFILEJS := openpowerlifting.js
MEETFILE := meets.csv
MEETFILEJS := meets.js

all: csv server

builddir:
	mkdir -p '${BUILDDIR}'

# Cram all the data into a single, huge CSV file.
csv: builddir
	scripts/compile "${BUILDDIR}" "${DATADIR}" "lifter-data"
	scripts/csv-bodyweight "${BUILDDIR}/${PLFILE}"
	scripts/csv-wilks "${BUILDDIR}/${PLFILE}"

# Optionally build an SQLite3 version of the database.
sqlite: csv
	scripts/prepare-for-sqlite
	scripts/compile-sqlite

server: csv
	$(MAKE) -C server

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

# Push the current version to the webservers.
deploy:
	$(MAKE) -C server/ansible

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

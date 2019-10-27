.PHONY: datadist sqlite check probe-quick probe deploy clean

MEETDATADIR := meet-data
BUILDDIR := build

PLFILE := entries.csv
MEETFILE := meets.csv
MEETFILEJS := meets.js

DATE := $(shell date --iso-8601)
DATADIR := ${BUILDDIR}/openpowerlifting-${DATE}

all: csv server

# Cram all the data into huge CSV files. New hotness.
csv:
	cargo run --bin checker -- --compile

# Build the CSV file hosted on the Data page for use by humans.
# The intention is to make it easy to use for people on Windows.
data:
	cargo run --bin checker -- --compile-onefile
	mkdir -p "${DATADIR}"
	mv "${BUILDDIR}/openpowerlifting.csv" "${DATADIR}/openpowerlifting-${DATE}.csv"
	cp LICENSE-DATA '${DATADIR}/LICENSE.txt'
	cp docs/data-readme.md '${DATADIR}/README.txt'
	rm -f "${BUILDDIR}/openpowerlifting-latest.zip"
	cd "${BUILDDIR}" && zip -r "openpowerlifting-latest.zip" "openpowerlifting-${DATE}"

# Optionally build an SQLite3 version of the database.
sqlite: csv
	scripts/prepare-for-sqlite
	scripts/compile-sqlite

server: csv
	$(MAKE) -C server

# Make sure that all the fields in the CSV files are in expected formats.
check:
	cargo run --bin checker
	tests/check-sex-consistency
	tests/check-lifter-data
	tests/check-duplicates
	tests/check-python-style

# Run all probes in a quick mode that only shows a few pending meets.
probe-quick:
	find "${MEETDATADIR}" -name "*-probe" | sort | parallel --timeout 5m --keep-order --will-cite "{} --quick"

# Run all probes.
probe:
	find "${MEETDATADIR}" -name "*-probe" | sort | parallel --timeout 5m --keep-order --will-cite

# Push the current version to the webservers.
deploy:
	$(MAKE) -C server/ansible

clean:
	rm -rf '${BUILDDIR}'
	rm -rf 'scripts/__pycache__'
	rm -rf 'tests/__pycache__'
	rm -rf '${MEETDATADIR}/apf/__pycache__'
	rm -rf '${MEETDATADIR}/cpu/__pycache__'
	rm -rf '${MEETDATADIR}/ipf/__pycache__'
	rm -rf '${MEETDATADIR}/nasa/__pycache__'
	rm -rf '${MEETDATADIR}/nipf/__pycache__'
	rm -rf '${MEETDATADIR}/nsf/__pycache__'
	rm -rf '${MEETDATADIR}/pa/__pycache__'
	rm -rf '${MEETDATADIR}/rps/__pycache__'
	rm -rf '${MEETDATADIR}/spf/__pycache__'
	rm -rf '${MEETDATADIR}/thspa/__pycache__'
	rm -rf '${MEETDATADIR}/usapl/__pycache__'
	rm -rf '${MEETDATADIR}/wrpf/__pycache__'
	$(MAKE) -C server clean
	rm -rf target

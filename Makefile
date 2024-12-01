MEETDATADIR := meet-data
BUILDDIR := build

DATE := $(shell date '+%Y-%m-%d') # Updated to work on macOS.
COMMIT := $(shell git rev-parse --short HEAD)
DATADIR := ${BUILDDIR}/openpowerlifting-${DATE}-${COMMIT}

# Default build target.
# Compiles all data into CSV files, and builds the server that loads the CSV files.
.PHONY: all
all: csv server

# Combines raw data files into "build/{entries,meets,lifters}.csv".
.PHONY: csv
csv:
	tests/check --compile

# Builds the CSV file hosted on the Data page for use by humans.
# This is a single huge "build/openpowerlifting.csv" that's intended to be easily used by humans.
.PHONY: data
data:
	tests/check --compile-onefile
	mkdir -p "${DATADIR}"
	mv "${BUILDDIR}/openpowerlifting.csv" "${DATADIR}/openpowerlifting-${DATE}-${COMMIT}.csv"
	cp LICENSE-DATA '${DATADIR}/LICENSE.txt'
	cp docs/data-readme.md '${DATADIR}/README.txt'
	rm -f "${BUILDDIR}/openpowerlifting-latest.zip"
	cd "${BUILDDIR}" && zip -r "openpowerlifting-latest.zip" "openpowerlifting-${DATE}-${COMMIT}"

# Builds an SQLite3 version of the database.
# Nothing in the project uses this file, but it might be interesting for data analysis.
.PHONY: sqlite
sqlite: csv
	scripts/prepare-for-sqlite
	scripts/compile-sqlite

# Builds the OpenPowerlifting HTTP server.
.PHONY: server
server: csv
	$(MAKE) -C server

# Makes sure that all the fields in the CSV files are in expected formats.
.PHONY: check
check:
	tests/check --timing
	tests/check-lifter-data
	tests/check-line-endings

# Checks all the CSV files, but additionally validate the Python scripts too
.PHONY: check-all
check-all: check
	tests/check-python-style

# Runs all probes in a quick mode that only shows a few pending meets.
.PHONY: probe-quick
probe-quick:
	find "${MEETDATADIR}" -name "*-probe.py" | sort | parallel --timeout 5m --keep-order --will-cite "{} --quick"

# Runs all probes.
.PHONY: probe
probe:
	find "${MEETDATADIR}" -name "*-probe.py" | sort | parallel --timeout 5m --keep-order --will-cite

# Pushes the current version to the webservers.
.PHONY: deploy
deploy:
	$(MAKE) -C server/ansible

# Removes all known temporary and build files from the directory tree.
.PHONY: clean
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

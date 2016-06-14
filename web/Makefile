.PHONY: builddir csvfile check

BUILDDIR = build
SCRIPTDIR = ../scripts

PLFILE = ../build/openpowerlifting.csv
PLFILEJS = openpowerlifting.js
MEETFILE = ../build/meets.csv
MEETFILEJS = meets.js

all: database

builddir:
	mkdir -p '${BUILDDIR}'

# Depends on the main project already being built.
database: builddir
	${SCRIPTDIR}/csv-to-js "${PLFILE}" opldb > "${BUILDDIR}/${PLFILEJS}"
	${SCRIPTDIR}/csv-to-js "${MEETFILE}" meetdb > "${BUILDDIR}/${MEETFILEJS}"

check:
	echo "Nothing to check for the moment."

clean:
	rm -rf '${BUILDDIR}'

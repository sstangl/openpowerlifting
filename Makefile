.PHONY: builddir csvfile check

BUILDDIR = build

all: csvfile

builddir:
	mkdir -p ${BUILDDIR}

# Cram all the data into a single, huge CSV file.
csvfile: builddir
	find . -name lifters.csv -print0 | xargs -0 'scripts/csv-cat' > "${BUILDDIR}/openpowerlifting.csv"

# Make sure that all the fields in the CSV files are in expected formats.
check:
	find . -name lifters.csv -exec 'scripts/check-lifters-csv' '{}' ';'
	find . -name meet.csv -exec 'scripts/check-meet-csv' '{}' ';'

clean:
	rm -rf ${BUILDDIR}

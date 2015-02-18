# Make sure that all the fields in the CSV files are in expected formats.
.PHONY: check

check:
	find . -name lifters.csv -type f -print -exec 'scripts/check-lifters-csv' '{}' ';'

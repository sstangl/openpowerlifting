.PHONY: check

# Make sure that all the fields in the CSV files are in expected formats.
check:
	find . -name lifters.csv -type f -exec 'scripts/check-lifters-csv' '{}' ';'
	find . -name meet.csv -type f -exec 'scripts/check-meet-csv' '{}' ';'

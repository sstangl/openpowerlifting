#!/bin/bash
#This script takes a file containing a list of URLs of nasa meet results.
#One by one, it fetches the pdf, creates a new folder, moves the pdf to that folder, applies tabula to it
#runs nasa-standardize to it, applies check-entries and stores the output.
input="pdf_small"
COUNTER=1
while IFS= read -r var
do
  mkdir /Users/blerner/openpowerlifting/meet-data/nasa//140"$COUNTER"
	wget $var
	mv *.pdf /Users/blerner/openpowerlifting/meet-data/nasa/140"$COUNTER"
	cd /Users/blerner/openpowerlifting/meet-data/nasa/140"$COUNTER"
	for f in *;
	do
		java -jar /Users/blerner/openpowerlifting/tabula-1.0.1-jar-with-dependencies.jar -l -p all $f >"results.csv"
		python /Users/blerner/openpowerlifting/meet-data/nasa/nasa-standardize-csv "results.csv" > "entries.csv"
		python /Users/blerner/openpowerlifting/tests/check-entries-csv "entries.csv"> check_entries
	done
	echo $var > URL
	cd ..
	let COUNTER=COUNTER+1
done < "$input"

#!/bin/bash
#This script takes a file containing a list of URLs of nasa meet results.
#One by one, it fetches the pdf, creates a new folder, moves the pdf to that folder, applies tabula to it
#runs nasa-standardize to it, applies check-lifters and stores the output.
input="pdf_small"
COUNTER=10
while IFS= read -r var
do
  mkdir /Users/blerner/openpowerlifting/meet-data/nasa/todo/"$COUNTER"
	wget $var
	mv *.pdf /Users/blerner/openpowerlifting/meet-data/nasa/todo/"$COUNTER"
	cd /Users/blerner/openpowerlifting/meet-data/nasa/todo/"$COUNTER"
	for f in *;
	do
		java -jar /Users/blerner/openpowerlifting/tabula-1.0.1-jar-with-dependencies.jar -l $f >"results.csv"
		python /Users/blerner/openpowerlifting/meet-data/nasa/nasa-standardize-csv "results.csv" > "lifters.csv"
		python /Users/blerner/openpowerlifting/tests/check-lifters-csv "lifters.csv"> check_lifters
	done
	echo $var > URL
	cd ..
	let COUNTER=COUNTER+1
done < "$input"

#!/bin/bash
#Script for reapplying nasa-standardize-csv to a bunch if pdfs.
COUNTER=2
while [ $COUNTER -le 2 ]
do
        cd /Users/blerner/openpowerlifting/meet-data/nasa/todo/170"$COUNTER"
        for f in *.pdf
        do
                java -jar /Users/blerner/openpowerlifting/tabula-1.0.1-jar-with-dependencies.jar -l -p all $f >"original.csv"
                python /Users/blerner/openpowerlifting/meet-data/nasa/nasa-standardize-csv "original.csv" > "entries.csv"
                bash /Users/blerner/openpowerlifting/tests/check > check_entries
        done
        let COUNTER=COUNTER+1
done

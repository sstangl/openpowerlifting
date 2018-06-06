# Results Format

This file describes how meet results are stored and how they are formatted.

Each meet is described by a number of CSV files, a convenient format for editing using normal spreadsheet software, like LibreOffice.

All CSV files are comma-separated and do not allow quotation marks.

During the build step, all of the different CSV files are combined together to form the one OpenPowerlifting database.

## Where are the Meet Results stored?

All meet data is stored in the folder [meet-data](https://gitlab.com/openpowerlifting/opl-data/tree/master/meet-data).

### Federation Directories

Within `meet-data`, each federation is given at least one directory. For example, there are `spf`, `usapl`, and `uspa` folders. Sometimes an additional directory will be created to catch "older" meets that the federation no longer reports on their website. For example, there is the `spf` directory (for new meets, tracking the SPF website) and the `spf-archive` directory (for old meets that were lost from the SPF website).

### Meet Directories

Each meet is represented by a folder inside the federation folder. For example, there is a meet under the folder `meet-data/spf/1625`.

The name given to the meet folder is extremely important: taken together with the federation folder, it forms the `MeetPath` that uniquely identifies the meet throughout the project.

So for example, a meet that is stored in `meet-data/fpo/1502` will have the `MeetPath` of `fpo/1502`, and on the website it will be accessible at the address [http://www.openpowerlifting.org/meet.html?m=fpo/1502](http://www.openpowerlifting.org/meet.html?m=fpo/1502).

For most federations, each meet folder is named YYxx, where YY is the last two digits of the year in which the meet was held, and xx is a counter starting at 01. If a federation had three meets in 2015, they would receive folders named `1501`, `1502`, and `1503`, respectively. The order does not matter: it is OK for `1503` to have taken place before `1501`.

Some federations give "sanction numbers" to their meets. In that case, the sanction number is used instead of the format described above. For example, the USPA numbers their meets, so we use their numbers, giving `uspa/0001` through `uspa/0884`; the USAPL gives sanction IDs, giving `usapl/PA-2014-01`; the CPU uses a database with calculated directory names, giving `cpu/2008-01-26-20a31723`.

### Meet Files

Each meet is described completely by files named `entries.csv` and `meet.csv`, the internal structure of which is specified below.

In addition, there are some required-if-applicable files that are not currently used to build the database, but provide helpful information:

- A file named `URL` includes a list of URLs, one on each line, pointing to the original source of the results.
- A file named `results.txt` (if source was PDF) or `results.csv` (if source was a spreadsheet) documents the original, un-edited version of the results as downloaded from the source. This is important in case of error.


## Structure of meet.csv

The file meet.csv always contains the same structure. Instead of typing out the columns yourself, it is easiest to make a copy of the file `meet-data/meet.template` and then simply fill in the second line.

Here is an example rendering, from `meet-data/uspa/0880/meet.csv`:

| Federation | Date       | MeetCountry | MeetState | MeetTown  | MeetName             |
|------------|------------|-------------|-----------|-----------|----------------------|
| USPA       | 2017-06-24 | USA         | AR        | Texarkana | Battle on the Border |

The fields are specified as follows:

- `Federation` is the approved acronym for a powerlifting federation. It must be part of the `KnownFederations` list specified in the file `tests/check-meet-csv`.
- `Date` is the start date of the meet in [ISO 8601 format](https://en.wikipedia.org/wiki/ISO_8601), so YYYY-MM-DD.
- `MeetCountry` is the name of the country in which the meet was held.
- `MeetState` is the abbreviation of the state/province in which the meet was held. If this is not known, or the country does not have states, it may be left blank.
- `MeetTown` is the name of the city/town in which the meet was held. If this is not known, it may be left blank.
- `MeetName` is the name of the competition. To avoid repetition, remove year and federation information from the name. So "2016 SPF Iron Classic" would have the `MeetName` of "Iron Classic".

The validity of meet.csv files is checked by the script `tests/check-meet-csv`.

## Structure of entries.csv

The file entries.csv contains varying columns based on the information provided by the federation. Some columns are mandatory, and some columns are optional. The order of the columns does not matter, because the build step knows how to reorder them.

Here is an example rendering, with illustrative data:

| Place | Name              | Sex | BodyweightKg | WeightClassKg | Division   | Equipment  | BestSquatKg | BestBenchKg | BestDeadliftKg | TotalKg | Event |
|-------|-------------------|-----|--------------|---------------|------------|------------|-------------|-------------|----------------|---------|-------|
| 1     | Adam Blank        | M   | 98.8         | 100           | Open       | Raw        | 192.5       | 140         | 222.5          | 555     | SBD   |
| 1     | Nicole Wallace    | F   | 73.2         | 75            | Masters 1  | Wraps      | 150         | 82.5        | 175            | 407.5   | SBD   |
| 1     | Ed Mulligan       | M   | 155          | 140+          | Submasters | Multi-ply  |             | 330         |                | 330     | B     |
| DQ    | Skeeter Valentine | M   | 63.2         | 67.5          | Juniors    | Single-ply | 140         |             | 130            |         | SBD   |

TODO

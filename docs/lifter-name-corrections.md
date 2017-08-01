# Lifter Name Corrections

This document describes how to edit lifter names in the database.

Because the project is organized as a set of flat CSV files, lifter names are constantly repeated in many places throughout the project. Manually editing all those files would be difficult and error-prone.

Additionally, we want to be able to remember the edits we've made, so that they can be applied automatically when new meet results are added.

There is a helper mechanism to automatically make the changes and remember them. The two pieces are the data file `lifter-data/name-corrections.dat` and the script `scripts/standardize-names`.

## Adding a new Name Correction

To add a new name correction, open the file `lifter-data/name-corrections.dat` in a text editor.

The format of the file is:

| (No Header)      |                  |                     |
|------------------|------------------|---------------------|
| Mike Womack      | Michael Womack   |                     |
| Jon Gruden       | Jon David Gruden | Jon David Gruden II |
| LaRodrick Duncan | Larodrick Duncan | LaRodic Duncan      |
| Gabrielle Mamani | Gabrielle Tucker |                     |

The first column contains the *correct name*. All columns after that one contain *incorrect names*, which will be automatically changed to the *correct name*. The number of columns is variable per-row.

Note that although the file contains rows with fields separated by commas, it is not a CSV file.

When adding a new row, make sure that none of the *correct names* or *incorrect names* occur elsewhere in the file. For example, adding the following two lines to `lifter-data/name-corrections.dat` would be erroneous:

> `John Snow,John Ashcroft`
> `John Ashcroft,John Snow`

This would cause the names to constantly switch back and forth every time `scripts/standardize-names` is run.

## Applying Name Corrections

There is a script that will open every `lifters.csv` file in the directory tree, look for *incorrect names*, and automatically fix them to their associated *correct name*.

To apply name corrections, execute the following command from the project root:

`./scripts/standardize-names lifter-data/name-corrections.dat`

To see what files were changed by the script, execute `git status`.

To commit the name changes, execute `git commit -a`. The `-a` flag automatically adds changed files that were already tracked by git from previous commits.

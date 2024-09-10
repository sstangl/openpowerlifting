# Configuring a Federation

## Enumerating the federation

The federation must be added to the `pub enum Federation` in `opl-data/crates/opltypes/src/federation.rs`.  This 
contains valid values for the `Federation` column in the `meets` table of the database.

For example:

```
    /// Australian Amateur Powerlifting Federation, defunct IPF affiliate.
    #[strum(to_string = "AAPLF", serialize = "aaplf")]
    AAPLF,
```

The `to_string` parameter to the `strum` attribute indicates how the fed name will be displayed.  The 
`serialize` parameter indicates how it will be stored.  Follow the existing conventions in the code regarding
names and add the new federation in alphabetical order.

## Adding the federation's drug testing status

Still in `federation.rs`, find the implementation of the `Federation` enum (`impl Federation { ... }`), and the `is_fully_tested()` function.
This function has a very long `match self { ... }` expression at the top.   Write a rust match arm to return
the new federation's drug testing status, following the guidelines in the comments.

For example:

```
            Federation::AAPLF => FULLY_TESTED,
```

## Adding the federation's nationality

Still in `federation.rs`, and still in the `Federation` enum's implementation, go to the next function `home_country()` and add a similar
match arm to the `match self { ... }` expression, eg:

```
            Federation::AAPLF => Some(Country::Australia),
```

For international federations, eg: IPF, the match arm should return `None`.

## Adding the federation's parent affiliation

Still in `federation.rs`, still in the `Federation` enum implementation, go to the `sanctioning_body()` function and add a match arm
to return the parent federation, if applicable, eg:

```
            Federation::AAPLF => Some(Federation::IPF),
```

Independent federations (eg: AAU) or top-level federations (eg: IPF) should return `None` here.

Federations that have changed affiliation over time can use a more complex expression, eg:

```
            Federation::PA => {
                // PA lost IPF affiliation in 2018, replaced by the APU.
                if date.year() >= 2018 {
                    Some(Federation::WP)
                } else {
                    Some(Federation::IPF)
                }
            }
```

For federations that maintain or have maintained multiple affiliations simultaneously (eg: CAPO), consult the team chat for a
recommendation.

## Adding the federation's best lifter formula

Still in `federation.rs` and the `Federation` enum implementation, go to the `default_points()` function and add a match arm
to the `match self { ... }` expression to return the formula used by the new federation.   For example:

```
            Federation::APF => PointsSystem::Glossbrenner,
```

Similarly to parent affiliations, when a federation changes its formula over time, more complex expressions can be used.

Federations that are affiliated with certain parent federations should return a call to an appropriate helper function, eg: 
`Federation::ipf_rules_on(date)`, which selects the correct IPF formula based on the date, or `Federation::ipl_rules_on(date)` 
which does similarly for IPL affiliates.


## Adding the federation to the fed selector widget

Add an appropriate pair of `<option ... > ... </option>` tags to `opl-data/server/templates/openpowerlifting/widget-fedselect.tera`,
within the appropriate `<optgroup ... > ... </optgroup>` tags.  The option grouping is based on region/nationality.  For example:

```
  <option value="adfpf" {% if selection.federation == "ADFPF" %}selected{% endif %}>ADFPF - American Drug Free Powerlifting Federation</option>
```


## Creating the federation's meet data directory

Create a new directory in `opl-data/meet-data`.  The name of this directory should be the `serialize` parameter to the `strum` attribute
added earlier to the `Federation` enum.


## Editing the federation's configuration file

Federations need to be configured for our Checker to know whether entries are valid for the federation.

All federations will have a preliminary `_CONFIG.toml` file. Editing this file is what we have been calling "Configuration".
A fully configured federation will have a `CONFIG.toml` file.  The removal of the underscore tells the Checker to use that file.

When a federation is configured, it means that this file adequately describes the metadata of the federation.
This means information such as divisions and weight classes are enumerated.
Also, all `entries.csv` files for that federation match the format specified in the configuration file.
This is what the Checker is checking for.

## Adding the federation's probe script (optional)

If the federation consistently publishes results at predictable URLs, you may wish to write a probe script to automatically
find meets that are not in the database, and potentially add them automatically, if the scoresheet format is also
predictable and easily machine-readable.

The probe script lives in the federation's meet data directly created in the previous step, and its name must end
in `-probe`, so that it will be included in the probe scripts specified by the `Makefile` and run during a `make`.

How to write a probe script is beyond the scope of this document.

## Running the Checker

These steps assume you already have the project set up.

You can run the Checker with this command:
```cargo run -p checker```

You may also run the checker for a specific federation, e.g. USAPL

```
cargo run -p checker ./meet-data/usapl
```

The checker will look for a valid `CONFIG.toml`. You will need to make sure you remove the `_` if you want the Checker to run that file.

It will check that all `entries.csv` files are in accordance with the structure described in the `CONFIG.toml` file.

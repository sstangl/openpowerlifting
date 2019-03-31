# Configuring a Federation

Federations need to be configured for our Checker to know whether entries are valid for the federation.

All federations will have a preliminary `_CONFIG.toml` file. Editing this file is what we have been calling "Configuration".
A fully configured federation will have a `CONFIG.toml` file.  The removal of the underscore tells the Checker to use that file.

When a federation is configured, it means that this file adequately describes the metadata of the federation.
This means information such as divisions and weight classes are enumerated.
Also, all `entries.csv` files for that federation match the format specified in the configuration file.
This is what the Checker is checking for.


# Running the Checker

These steps assume you already have the project set up.

You can run the Checker with this command:
```cargo run -p checker```

You may also run the checker for a specific federation, e.g. USAPL

```
cargo run -p checker ./meet-data/usapl
```

The checker will look for a valid `CONFIG.toml`. You will need to make sure you remove the `_` if you want the Checker to run that file.

It will check that all `entries.csv` files are in accordance with the structure described in the `CONFIG.toml` file.
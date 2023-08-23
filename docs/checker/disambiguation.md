# Introduction to Disambiguations

## Our Meet is Ready
We have finished adding our meet and we think it's ready to go, so we run the checker

`
user@machine:~/opl-data/meet-data/spf/2315$ ll -h
total 4.0K
drwxr-xr-x 1 dex dex 4.0K Aug 23 19:24 ./
drwxr-xr-x 1 dex dex 4.0K Aug 22 20:56 ../
-rw-r--r-- 1 dex dex   63 Aug 22 20:40 URL
-rw-r--r-- 1 dex dex 1.8K Aug 23 19:24 entries.csv
-rw-r--r-- 1 dex dex   98 Aug 22 20:46 meet.csv
`

## Running the Makefile
In the `opl-data/` base directory, run

```make``` 

This will run the Makefile, building the project.

## [Optional] Building the Backend

Openpowerlifting is currently developing a backend in Rust using the Rocket web
framework.  To install this subproject, see the following steps.

### Installing Rust and Cargo

Visit [rustup](https://www.rustup.rs/) and download/run `rustup`, the Rust installer.

### Building the server
In the `server/` directory, run

```cargo build```

### Running the server
In the `server/` directory, run 

```cargo run```

The project should now be viewable at the default location of `localhost:8000`

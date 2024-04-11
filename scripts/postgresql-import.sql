-- Imports {entries.csv, meets.csv, lifters.csv} into PostgreSQL.
--
-- To build those files, run 'make'.
-- This script should be run from the opl-data root, as:
--   psql --file scripts/postgresql-import.sql
--

\set ON_ERROR_STOP true

-- All data goes into an "opldb" schema.
-- This allows dropping all types related to OpenPowerlifting at once:
--   DROP SCHEMA opldb CASCADE;
CREATE SCHEMA opldb;

-- Define types. Enums save space by using a u32 internally.
CREATE TYPE opldb.equipment AS ENUM ('Raw', 'Wraps', 'Single-ply', 'Multi-ply', 'Unlimited', 'Straps');

-- Structure of meets.csv.
CREATE TABLE opldb.opl_meets (
	id INTEGER PRIMARY KEY NOT NULL,
	path VARCHAR NOT NULL,
	federation VARCHAR NOT NULL,
	date DATE NOT NULL,
	country VARCHAR NOT NULL,
	state VARCHAR,
	town VARCHAR,
	name VARCHAR NOT NULL,
	ruleset VARCHAR,
	sanctioned VARCHAR
);
\copy opldb.opl_meets FROM 'build/meets.csv' DELIMITER ',' CSV HEADER

CREATE TABLE opldb.opl_lifters (
	id INTEGER PRIMARY KEY NOT NULL,
	name VARCHAR NOT NULL,
	cyrillic_name VARCHAR,
	chinese_name VARCHAR,
	greek_name VARCHAR,
	japanese_name VARCHAR,
	korean_name VARCHAR,
	username VARCHAR NOT NULL,
	instagram VARCHAR,
	color VARCHAR
);
\copy opldb.opl_lifters FROM 'build/lifters.csv' DELIMITER ',' CSV HEADER

CREATE TABLE opldb.opl_entries (
	meet_id INTEGER REFERENCES opldb.opl_meets(id) ON DELETE CASCADE,
	lifter_id INTEGER REFERENCES opldb.opl_lifters(id) ON DELETE CASCADE,
	sex CHAR(2) NOT NULL,
	event CHAR(3) NOT NULL,
	equipment opldb.equipment NOT NULL,
	age NUMERIC,
	age_class VARCHAR,
	birthyear_class VARCHAR,
	division VARCHAR,
	bodyweightkg NUMERIC,
	weightclasskg VARCHAR, -- Non-numeric due to SHW classes (140+).
	squat1kg NUMERIC,
	squat2kg NUMERIC,
	squat3kg NUMERIC,
	squat4kg NUMERIC,
	best3squatkg NUMERIC,
	bench1kg NUMERIC,
	bench2kg NUMERIC,
	bench3kg NUMERIC,
	bench4kg NUMERIC,
	best3benchkg NUMERIC,
	deadlift1kg NUMERIC,
	deadlift2kg NUMERIC,
	deadlift3kg NUMERIC,
	deadlift4kg NUMERIC,
	best3deadliftkg NUMERIC,
	totalkg NUMERIC,
	place VARCHAR, -- Non-numeric due to DQ, G, NS, etc.
	wilks NUMERIC,
	mcculloch NUMERIC,
	glossbrenner NUMERIC,
	goodlift NUMERIC,
	ipf_points NUMERIC,
	dots NUMERIC,
	tested VARCHAR,
	lifter_country VARCHAR,
	lifter_state VARCHAR
);
\copy opldb.opl_entries FROM 'build/entries.csv' DELIMITER ',' CSV HEADER

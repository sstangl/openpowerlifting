//! Tests for entries.csv files.

extern crate checker;
extern crate csv;

use checker::checklib::entries::do_check;
use checker::{Meet, Report};
use opltypes::{Country, Date, Federation, RuleSet};

use std::path::PathBuf;

/// Executes checks against a string representation of a CSV,
/// returning the number of errors.
fn check(csv: &str) -> usize {
    let report = Report::new(PathBuf::from("[inline_parent]/[inline]"));
    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .from_reader(csv.as_bytes());

    // A non-existent meet. The details aren't important to the tests below.
    let meet = Meet {
        path: "testing/mymeet".to_string(),
        federation: Federation::USAUA,
        date: Date::from_parts(2016, 01, 01),
        country: Country::USA,
        state: None,
        town: None,
        name: "Test Meet For Example Purposes".to_string(),
        ruleset: RuleSet::default(),
        sanctioned: true,
        allow_duplicates: false,
    };

    let checkresult = do_check(&mut rdr, &meet, None, None, report).unwrap();
    checkresult.report.count_messages().errors()
}

#[test]
fn test_empty_file() {
    assert!(check("") > 0);
}

#[test]
fn test_invalid_headers() {
    // This should pass tests.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 0);

    // Add an extra column "X".
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place,X\n\
                Test User,90,M,100,100,Raw,B,1,X";
    assert_eq!(check(data), 1);

    // Duplicate the Sex column. The error message should only occur once.
    let data = "Name,WeightClassKg,Sex,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);

    // The Name column is mandatory.
    let data = "WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);

    // There must be either (or both) of WeightClassKg and BodyweightKg.
    let data = "Name,BodyweightKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 0);
    let data = "Name,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);

    // The Sex column is mandatory.
    let data = "Name,WeightClassKg,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,100,100,Raw,B,1";
    assert_eq!(check(data), 1);

    // The Equipment column is mandatory.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Event,Place\n\
                Test User,90,M,100,100,B,1";
    assert_eq!(check(data), 1);

    // The TotalKg column is mandatory.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,Equipment,Event,Place\n\
                Test User,90,M,100,Raw,B,1";
    assert_eq!(check(data), 1);

    // The Place column is mandatory.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event\n\
                Test User,90,M,100,100,Raw,B";
    assert_eq!(check(data), 1);

    // The Event column is mandatory.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Place\n\
                Test User,90,M,100,100,Raw,1";
    assert_eq!(check(data), 1);

    // If there's a data column for a lift, the Best column must exist.
    let data = "Name,WeightClassKg,Sex,Squat1Kg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);
    let data = "Name,WeightClassKg,Sex,Bench1Kg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);
    let data = "Name,WeightClassKg,Sex,Deadlift1Kg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);
}

#[test]
fn test_column_sex() {
    // The sex column cannot be empty.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,,100,100,Raw,B,1";
    assert_eq!(check(data), 1);

    // The sex column cannot be something invalid.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,Z,100,100,Raw,B,1";
    assert_eq!(check(data), 1);
}

#[test]
fn test_column_cyrillicname() {
    // Cyrillic should pass.
    let data = "Name,CyrillicName,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,Тест Юзр,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 0);

    // Non-Cyrillic should fail.
    let data = "Name,CyrillicName,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,Test User,90,M,100,100,Raw,B,1";
    assert_eq!(check(data), 1);
}

#[test]
fn test_column_event() {
    // Squat event, but no Best3SquatKg.
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,S,1";
    assert!(check(data) > 0);

    // Bench event, but no Best3BenchKg.
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,B,1";
    assert!(check(data) > 0);

    // Deadlift event, but no Best3DeadliftKg.
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,Raw,D,1";
    assert!(check(data) > 0);
}

#[test]
fn test_event_consistency() {
    // Bench-only lifter with valid data, but marked SBD.
    let data = "Name,Division,BirthDate,WeightClassKg,BodyweightKg,Sex,Tested,\
                Squat1Kg,Squat2Kg,Squat3Kg,Squat4Kg,Best3SquatKg,Bench1Kg,Bench2Kg,\
                Bench3Kg,Bench4Kg,Best3BenchKg,Deadlift1Kg,Deadlift2Kg,Deadlift3Kg,\
                Deadlift4Kg,Best3DeadliftKg,TotalKg,Place,Event,Equipment,Country\n\
                Sergei Molchanov,O,1973-03-15,125,124.6,M,No,,,,,,-230,230,240,,240\
                ,,,,,,240,5,SBD,Raw,Russia";
    assert!(check(data) > 1);
}

#[test]
fn test_column_equipment() {
    // Invalid equipment
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,Best3BenchKg,Best3DeadliftKg,TotalKg,Equipment,Event,Place\n\
                Test User,90,M,100,100,100,300,ABCD,SBD,1";
    assert_eq!(check(data), 1);

    // Invalid Squat equipment
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,TotalKg,Equipment,SquatEquipment,Event,Place\n\
                Test User,90,M,100,100,Multi-ply,Straps,S,1";
    assert_eq!(check(data), 2);

    // Invalid Bench equipment
    let data = "Name,WeightClassKg,Sex,Best3BenchKg,TotalKg,Equipment,BenchEquipment,Event,Place\n\
                Test User,90,M,100,100,Multi-ply,Wraps,B,1";
    assert_eq!(check(data), 1);

    // Invalid Deadlift equipment
    let data =
        "Name,WeightClassKg,Sex,Best3DeadliftKg,TotalKg,Equipment,DeadliftEquipment,Event,Place\n\
                Test User,90,M,100,100,Multi-ply,Wraps,D,1";
    assert_eq!(check(data), 1);

    // Squat equipment greater than equipment, this should fail
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,Best3BenchKg,Best3DeadliftKg,TotalKg,Equipment,SquatEquipment,Event,Place\n\
                Test User,90,M,100,100,100,300,Raw,Wraps,SBD,1";
    assert_eq!(check(data), 1);

    // Bench equipment greater than equipment, this should fail
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,Best3BenchKg,Best3DeadliftKg,TotalKg,Equipment,BenchEquipment,Event,Place\n\
                Test User,90,M,100,100,100,300,Raw,Multi-ply,SBD,1";
    assert_eq!(check(data), 1);

    // Deadlift equipment greater than equipment, this should fail
    let data = "Name,WeightClassKg,Sex,Best3SquatKg,Best3BenchKg,Best3DeadliftKg,TotalKg,Equipment,DeadliftEquipment,Event,Place\n\
                Test User,90,M,100,100,100,300,Raw,Multi-ply,SBD,1";
    assert_eq!(check(data), 1);
}

#[test]
fn regression_tests() {
    // Calculated total should be compared even if a lift is missing an attempt.
    let data = "Place,Name,Sex,Event,Division,WeightClassKg,Equipment,BirthYear,Team,BodyweightKg,Squat1Kg,Squat2Kg,Squat3Kg,Best3SquatKg,Bench1Kg,Bench2Kg,Bench3Kg,Best3BenchKg,Deadlift1Kg,Deadlift2Kg,Deadlift3Kg,Best3DeadliftKg,TotalKg\n\
                1,Chad Cooper,M,SBD,R-O,93,Raw,1998,SC,88.4,170,180,,180,92.5,-97.5,-97.5,92.5,192.5,212.5,215,215,522.5";
    assert_eq!(check(data), 1);

    // Bench is a failure, but the lifter is given a total and not marked DQ.
    let data = "Name,Sex,BodyweightKg,Deadlift1Kg,Deadlift2Kg,Deadlift3Kg,Best3DeadliftKg,Deadlift4Kg,TotalKg,WeightClassKg,Event,Equipment,Tested,Bench1Kg,Bench2Kg,Bench3Kg,Best3BenchKg,Bench4Kg,Squat1Kg,Squat2Kg,Squat3Kg,Best3SquatKg,Squat4Kg,Division,Age,Place\n\
                Aleksey Krasnoshchekov,M,86,,,,,,170,90,SBD,Raw,No,-135,-135,-135,,,160,165,170,170,,Pro Masters 40-44,42,1";
    assert_eq!(check(data), 1);
}

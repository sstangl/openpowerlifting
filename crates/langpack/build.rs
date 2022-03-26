//! Parses JSON translation files into Rust structs at compile time.
//!
//! This avoids two problems:
//!  1. Run-time parsing of static files is inefficient.
//!  2. Asking serde to parse JSON into huge strongly-typed structs causes extreme
//!     code generation overhead. This was about 30s with rustc 1.58, but ballooned
//!     to about 15m with rustc 1.59. We avoid this by deserializing into a Value,
//!     then translating those Values directly into struct output. With rust 1.59,
//!     this takes about 11 seconds.
//!
//! This build script works by translating each translations JSON file into a separate
//! Rust source file within the $OUT_DIR provided by Cargo. The library defines the
//! shape of this struct, then uses `include!` to load the build-time definitions.
//!
//! Because the build-time definition contains type information, compilation of the
//! library itself asserts that the shape of each translation file matches expectation.

use serde_json::{Map, Value};

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Map from JSON key to the internal struct name.
///
/// This corresponds to the definitions of the following structs:
///  1. `Translations`, when `parent == None`.
///  2. `SelectorTranslations`, when `parent == Some("selectors")`.
///
fn struct_identifier(parent: Option<&str>, key: &str) -> &'static str {
    match parent {
        // Outermost object definitions, from `Translations`.
        None => match key {
            "units" => "UnitsTranslations",
            "equipment" => "EquipmentTranslations",
            "sex" => "SexTranslations",
            "page_titles" => "PageTitleTranslations",
            "header" => "HeaderTranslations",
            "html_header" => "HtmlHeaderTranslations",
            "columns" => "ColumnTranslations",
            "country" => "CountryTranslations",
            "buttons" => "ButtonTranslations",
            "labels" => "LabelTranslations",
            "selectors" => "SelectorTranslations",
            "lifter_page" => "LifterPageTranslations",
            _ => panic!("Unexpected key '{key}'"),
        },

        // Definitions for the `SelectorTranslations` struct.
        Some("selectors") => match key {
            "equipment" => "EquipmentSelectorTranslations",
            "weightclass" => "WeightClassSelectorTranslations",
            "sort" => "SortSelectorTranslations",
            "year" => "YearSelectorTranslations",
            "sex" => "SexSelectorTranslations",
            "event" => "EventSelectorTranslations",
            "fed" => "FedSelectorTranslations",
            "ageclass" => "AgeClassSelectorTranslations",
            _ => panic!("Unexpected 'selectors' sub-key '{key}'"),
        },

        _ => panic!("Unexpected parent {}", parent.unwrap()),
    }
}

/// Given a JSON object, recursively translate into a struct body.
///
/// ## Output
/// This function outputs just the struct body, i.e., just the `{ contents }`,
/// without the identifier `MyStruct` (as in `MyStruct { contents }`).
///
fn process_obj(parent: Option<&str>, object: &Map<String, Value>) -> String {
    let mut acc: String = "{".to_owned(); // Open new struct body.

    for (key, value) in object.iter() {
        if value.is_string() {
            let translation = value.as_str().unwrap();
            let defn = format!("{key}: \"{translation}\", ");
            acc.push_str(&defn);
        } else {
            let identifier = struct_identifier(parent, key);
            let inner_object = value.as_object().unwrap();
            let inner_object_body = process_obj(Some(key), inner_object);
            let defn = format!("{key}: {identifier} {inner_object_body}, ");
            acc.push_str(&defn);
        }
    }

    acc.push('}'); // Close new struct body.
    acc
}

/// Given the path to a JSON translations file, parse it into Rust source.
fn process_json(json_file: PathBuf) -> String {
    // Ensure to the best of our ability that only JSON files are processed.
    assert_eq!("json", json_file.extension().unwrap());

    // Parse the file to an untyped Value, intentionally avoiding struct ser/de.
    let raw_json: String = fs::read_to_string(&json_file).unwrap();
    let json: Value = serde_json::from_str(&raw_json).unwrap();

    // The translations file must contain a single object.
    let outer_obj: &Map<String, Value> = json.as_object().unwrap();

    // Recursively process the object, producing a string like "{ inner }".
    let translations_body = process_obj(None, outer_obj);
    format!("Translations {translations_body}")
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    // Cargo always sets the current directory to the crate root.
    let crate_dir = env::current_dir().unwrap();

    for entry in fs::read_dir(crate_dir.join("translations")).unwrap() {
        let entry = entry.unwrap();
        let as_struct = process_json(entry.path());

        // Convert "{locale}.json" to "{locale}.rs".
        let rust_path = entry.path().with_extension("rs");
        let rust_filename = rust_path.file_name().unwrap();

        // Write as "{locale}.rs" in the approved OUT_DIR.
        // The lib.rs can `include!` these files directly as expressions.
        let dest_path = Path::new(&out_dir).join(&rust_filename);
        fs::write(&dest_path, as_struct).unwrap();
    }
}

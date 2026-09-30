//! Implements a compact map from Username to LifterId using a Finite State Transducer.

use std::error::Error;
use std::fs;
use std::path::Path;

use fst::{IntoStreamer, Map, Streamer};

/// A compact map from Username to LifterId.
///
/// Based on a Finite State Transducer, it supports operations similar to a BTreeMap.
#[derive(Debug)]
pub struct UsernameMap {
    map: Map<Vec<u8>>,
}

impl UsernameMap {
    // Creates a new UsernameMap by loading a precalculated FST file into memory.
    pub fn new(username_map_fst: &Path) -> Result<Self, Box<dyn Error>> {
        let as_bytes: Vec<u8> = fs::read(username_map_fst)?;
        let map = Map::new(as_bytes)?;
        Ok(UsernameMap { map })
    }

    // Looks up a single LifterId by Username.
    //
    // TODO: Convert this to take a &Username as an argument.
    pub fn get(&self, username: &str) -> Option<u32> {
        self.map.get(username).map(|id| id as u32)
    }

    /// Looks up all lifters sharing the same disambiguation base.
    ///
    /// For example, looking up "johnsmith" would return IDs for:
    /// - "johnsmith"
    /// - "johnsmith1"
    /// - "johnsmith113"
    /// - "johnsmith21"
    ///
    /// but not:
    /// - "johnsmithjr"
    pub fn get_with_base(&self, username_base: &str) -> Vec<u32> {
        let mut stream = self
            .map
            .range()
            .ge(format!("{username_base}0")) // >= all numbers.
            .lt(format!("{username_base}:")) // <= all numbers (`:` is the next ASCII char after `9`).
            .into_stream();

        let mut acc = Vec::new();

        // Handle the case of non-disambiguation also existing.
        if let Some(id) = self.get(username_base) {
            acc.push(id);
        }

        // Handle all disambiguations.
        while let Some((_key, id)) = stream.next() {
            acc.push(id as u32);
        }
        acc
    }
}

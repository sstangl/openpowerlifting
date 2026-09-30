//! Implements a compact map from Username to LifterId using a Finite State Transducer.

use std::error::Error;
use std::fs;
use std::path::Path;

/// A compact map from Username to LifterId.
///
/// Based on a Finite State Transducer, it supports operations similar to a BTreeMap.
#[derive(Debug)]
pub struct UsernameMap {
    map: fst::Map<Vec<u8>>,
}

impl UsernameMap {
    // Creates a new UsernameMap by loading a precalculated FST file into memory.
    pub fn new(username_map_fst: &Path) -> Result<Self, Box<dyn Error>> {
        let as_bytes: Vec<u8> = fs::read(username_map_fst)?;
        let map = fst::Map::new(as_bytes)?;
        Ok(UsernameMap { map })
    }

    // Looks up a single LifterId by username.
    pub fn get(&self, username: &str) -> Option<u32> {
        self.map.get(username).map(|id| id as u32)
    }
}

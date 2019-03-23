//! Defines MeetData, the owner of all meet-related data produced by the
//! Checker.

use crate::checklib::{Entry, Meet};

/// All checker-generated data structures for a single meet.
pub struct SingleMeetData {
    pub meet: Meet,
    pub entries: Vec<Entry>,
}

/// Permanent owner of all data from all meets.
pub struct AllMeetData {
    /// An owned vector of all meet data.
    ///
    /// Once assigned, the Vec may not be resized. To enforce this invariant,
    /// getters are used that only provide a slice.
    meets: Vec<SingleMeetData>,
}

impl From<Vec<SingleMeetData>> for AllMeetData {
    fn from(v: Vec<SingleMeetData>) -> AllMeetData {
        AllMeetData { meets: v }
    }
}

impl AllMeetData {
    /// Borrows the meet data immutably.
    pub fn get_meets(&self) -> &[SingleMeetData] {
        self.meets.as_slice()
    }

    /// Borrows the meet data mutably. The underlying vector remains immutable.
    pub fn get_meets_mut(&mut self) -> &mut [SingleMeetData] {
        self.meets.as_mut_slice()
    }
}

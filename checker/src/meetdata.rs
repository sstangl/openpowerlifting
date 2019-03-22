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
    pub meets: Vec<SingleMeetData>,
}

impl From<Vec<SingleMeetData>> for AllMeetData {
    fn from(v: Vec<SingleMeetData>) -> AllMeetData {
        AllMeetData { meets: v }
    }
}

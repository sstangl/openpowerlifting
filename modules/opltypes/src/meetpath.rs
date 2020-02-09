//! Helper functions for getting the MeetPath from a filesystem path.

use std::path::Path;

/// The name of the folder in which meet data resides.
const MEETDATADIR: &'static str = "meet-data";

/// Possible failures when constructing a MeetPath.
#[derive(Debug, Eq, PartialEq, ToString)]
pub enum MeetPathError {
    /// The MeetPath contained non-ASCII characters.
    ///
    /// ASCII is enforced because the MeetPath is used in server URLs.
    NonAsciiError,

    /// There was an internal error parsing filesystem paths as UTF-8.
    FilesystemUTF8Error,

    /// There was a failure looking up a parent directory.
    ParentLookupError,

    /// The MEETDATADIR does not appear to be in the path.
    ///
    /// MeetPaths are always constructed relative to the MEETDATADIR.
    MeetDataDirNotFoundError,
}

/// Gets the MeetPath from a string representing a filesystem path.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// # use opltypes::file_to_meetpath;
/// let file = PathBuf::from("/home/opl-data/meet-data/rps/1924/meet.csv");
/// assert_eq!(file_to_meetpath(&file).unwrap(), "rps/1924");
/// ```
pub fn file_to_meetpath<'a>(filepath: &'a Path) -> Result<&'a str, MeetPathError> {
    let parent = filepath.parent().ok_or(MeetPathError::ParentLookupError)?;
    let parent_str = parent.to_str().ok_or(MeetPathError::FilesystemUTF8Error)?;

    // Index from the last occurrence of MEETDATADIR in the path string.
    let index = parent_str
        .rfind(&MEETDATADIR)
        .ok_or(MeetPathError::MeetDataDirNotFoundError)?;

    let meetpath = &parent_str[index..];

    // Each character must be alphanumeric ASCII, a UNIX path separator, or a dash.
    for c in meetpath.chars() {
        if !c.is_ascii_alphanumeric() && c != '/' && c != '-' {
            return Err(MeetPathError::NonAsciiError);
        }
    }

    Ok(meetpath)
}

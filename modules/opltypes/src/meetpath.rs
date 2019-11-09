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
pub fn file_to_meetpath(filepath: &Path) -> Result<String, MeetPathError> {
    if let Some(parent) = filepath.parent() {
        if let Some(s) = parent.to_str() {
            let meetpath: String = match s.rfind(&MEETDATADIR) {
                // Look up the path relative to the last occurrence of MEETDATADIR.
                Some(i) => s.chars().skip(i + MEETDATADIR.len() + 1).collect(),
                None => {
                    return Err(MeetPathError::MeetDataDirNotFoundError);
                }
            };

            // Each character may only be alphanumeric ASCII or "/".
            for c in meetpath.chars() {
                if !c.is_ascii_alphanumeric() && c != '/' && c != '-' {
                    return Err(MeetPathError::NonAsciiError);
                }
            }

            Ok(meetpath)
        } else {
            Err(MeetPathError::FilesystemUTF8Error)
        }
    } else {
        Err(MeetPathError::ParentLookupError)
    }
}

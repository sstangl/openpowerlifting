//! Helper functions for getting the MeetPath from a filesystem path.

use std::path::Path;

/// The name of the folder in which meet data resides.
const MEETDATADIR: &str = "meet-data";

/// Possible failures when constructing a MeetPath.
#[derive(Debug, Display, Eq, PartialEq)]
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

/// Gets the MeetPath from a string representing a filepath.
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
    let parent = filepath.parent().ok_or(MeetPathError::ParentLookupError)?;
    dir_to_meetpath(parent)
}

/// Gets the MeetPath from a string representing a directory.
///
/// Returns a String for the benefit of Windows, which requires
/// changing the path separator.
///
/// # Examples
///
/// ```
/// # use std::path::PathBuf;
/// # use opltypes::dir_to_meetpath;
/// let dir = PathBuf::from("/home/opl-data/meet-data/rps/1924");
/// assert_eq!(dir_to_meetpath(&dir).unwrap(), "rps/1924");
/// ```
pub fn dir_to_meetpath(dirpath: &Path) -> Result<String, MeetPathError> {
    let dir_str = dirpath.to_str().ok_or(MeetPathError::FilesystemUTF8Error)?;

    // Index from the last occurrence of MEETDATADIR in the path string.
    let index = dir_str
        .rfind(&MEETDATADIR)
        .ok_or(MeetPathError::MeetDataDirNotFoundError)?;

    let meetpath = dir_str[(index + MEETDATADIR.len() + 1)..].to_string();

    #[cfg(target_family = "windows")]
    let meetpath = meetpath.replace("\\", "/");

    // Each character must be alphanumeric ASCII, a UNIX path separator, or a dash.
    for c in meetpath.chars() {
        if !c.is_ascii_alphanumeric() && c != '/' && c != '-' {
            return Err(MeetPathError::NonAsciiError);
        }
    }

    Ok(meetpath)
}

#[cfg(target_family = "windows")]
#[cfg(test)]
mod windows_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn windows_meet_path() {
        let file = PathBuf::from("C:\\meet-data\\mags\\aus-assorted\\CONFIG.toml");
        assert_eq!(file_to_meetpath(&file).unwrap(), "mags/aus-assorted");
    }
}

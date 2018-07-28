//! Defines the Sex field.

/// The Sex field.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Sex {
    M,
    F,
}

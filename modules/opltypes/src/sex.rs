//! Defines the Sex field.

/// The Sex field.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
pub enum Sex {
    M,
    F,
}

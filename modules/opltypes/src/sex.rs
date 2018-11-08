//! Defines the Sex field.

/// The Sex column.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
pub enum Sex {
    M,
    F,
}

impl Default for Sex {
    fn default() -> Sex {
        Sex::M
    }
}

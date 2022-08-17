//! Defines the Sex field.

/// The Sex column.
#[derive(Clone, Copy, Debug, Deserialize, Display, EnumString, PartialEq, Eq, Serialize)]
pub enum Sex {
    /// Male.
    M,
    /// Female.
    F,
    /// A gender-neutral title, including non-binary lifters.
    Mx,
}

impl Default for Sex {
    fn default() -> Sex {
        Sex::M
    }
}

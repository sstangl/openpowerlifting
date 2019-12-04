//! Defines the Sex field.

/// The Sex column.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
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

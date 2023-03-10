//! Defines the Sex field.

/// The Sex column.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Display, EnumString, PartialEq, Eq, Serialize,
)]
pub enum Sex {
    /// Male.
    #[default]
    M,
    /// Female.
    F,
    /// A gender-neutral title, including non-binary lifters.
    Mx,
}

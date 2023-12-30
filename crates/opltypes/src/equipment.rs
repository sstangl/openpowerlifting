//! Defines the Equipment field.

use std::fmt;

/// The Equipment field.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, EnumString, PartialEq, Eq, Serialize, PartialOrd,
)]
pub enum Equipment {
    /// No supportive material (sleeves allowed).
    #[serde(alias = "Bare", alias = "Sleeves")]
    #[strum(serialize = "Raw", serialize = "Bare", serialize = "Sleeves")]
    Raw,

    /// Knee wraps.
    Wraps,

    /// Single-ply, non-rubberized fabric.
    #[serde(rename = "Single-ply")]
    #[strum(serialize = "Single-ply")]
    Single,

    /// Multi-ply, non-rubberized fabric.
    #[serde(rename = "Multi-ply")]
    #[strum(serialize = "Multi-ply")]
    Multi,

    /// Equipment more supportive than Multi-ply.
    #[default]
    Unlimited,

    /// Wrist straps for deadlifts.
    Straps,
}

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Equipment::Raw => write!(f, "Raw"),
            Equipment::Wraps => write!(f, "Wraps"),
            Equipment::Single => write!(f, "Single-ply"),
            Equipment::Multi => write!(f, "Multi-ply"),
            Equipment::Unlimited => write!(f, "Unlimited"),
            Equipment::Straps => write!(f, "Straps"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialization() {
        // Parse using the EnumString implementation.
        assert_eq!("Raw".parse::<Equipment>(), Ok(Equipment::Raw));
        assert_eq!("Bare".parse::<Equipment>(), Ok(Equipment::Raw));
        assert_eq!("Sleeves".parse::<Equipment>(), Ok(Equipment::Raw));
    }
}

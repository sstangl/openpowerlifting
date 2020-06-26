//! Defines the Equipment field.

use std::fmt;

/// The Equipment field.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize, PartialOrd)]
pub enum Equipment {
    /// No supportive material (sleeves allowed).
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
    Unlimited,

    /// Wrist straps for deadlifts.
    Straps,
}

impl Default for Equipment {
    fn default() -> Equipment {
        Equipment::Unlimited
    }
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

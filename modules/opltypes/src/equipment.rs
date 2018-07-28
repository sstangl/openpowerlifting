//! Defines the Equipment field.

/// The Equipment field.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename(deserialize = "Single-ply"))]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    Multi,
    Straps,
}

use std::fmt;

use serde::de::{Deserializer, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Attempt {
    Success(f32),
    Failure(f32),
    Skip,
}

impl Attempt {
    pub fn was_successful(&self) -> bool {
        matches!(self, Attempt::Success(_))
    }
}

struct AttemptVisitor;

impl<'de> Visitor<'de> for AttemptVisitor {
    type Value = Attempt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid attempt or empty string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // Added logic to handle empty strings
        if value.is_empty() {
            return Ok(Attempt::Skip);
        }

        if value == "0" || value == "0.0" {
            return Ok(Attempt::Skip);
        }

        fn parser<E: serde::de::Error, F: FnOnce(f32) -> Attempt>(
            value: &str,
            mapper: F,
        ) -> Result<Attempt, E> {
            value
                .parse::<f32>()
                .map(mapper)
                .map_err(|_| E::custom(format!("invalid floating point value {value}")))
        }

        value
            .strip_prefix('-')
            .map(|miss| parser(miss, Attempt::Failure))
            .unwrap_or_else(|| parser(value, Attempt::Success))
    }
}

impl<'de> Deserialize<'de> for Attempt {
    fn deserialize<D>(deserializer: D) -> Result<Attempt, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(AttemptVisitor)
    }
}

impl Serialize for Attempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Attempt::Success(value) => serializer.serialize_f32(*value),
            Attempt::Failure(value) => {
                let repr = format!("-{value}");
                serializer.serialize_str(&repr)
            }
            Attempt::Skip => serializer.serialize_str(""),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Placing {
    Disqualified,
    Rank(u16),
}

struct PlacingVisitor;

impl<'de> Visitor<'de> for PlacingVisitor {
    type Value = Placing;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid placing")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if value == "DSQ" {
            return Ok(Placing::Disqualified);
        }

        value
            .parse::<u16>()
            .map(Placing::Rank)
            .map_err(|_| E::custom(format!("invalid placing {value}")))
    }
}

impl<'de> Deserialize<'de> for Placing {
    fn deserialize<D>(deserializer: D) -> Result<Placing, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(PlacingVisitor)
    }
}

impl Serialize for Placing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Placing::Disqualified => serializer.serialize_str("DQ"),
            Placing::Rank(value) => serializer.serialize_u16(*value),
        }
    }
}

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use serde::de::value::Error as SerdeError;
    use serde::de::Visitor;

    use crate::types::Attempt;

    use super::AttemptVisitor;

    #[test]
    fn can_parse_skipped_lifts() -> Result<()> {
        let value = AttemptVisitor.visit_str::<SerdeError>("0.0")?;

        assert_eq!(value, Attempt::Skip);

        Ok(())
    }

    #[test]
    fn can_parse_successful_lifts() -> Result<()> {
        let value = AttemptVisitor.visit_str::<SerdeError>("250.0")?;

        assert_eq!(value, Attempt::Success(250.0));

        Ok(())
    }

    #[test]
    fn can_parse_failed_lifts() -> Result<()> {
        let value = AttemptVisitor.visit_str::<SerdeError>("-250.0")?;

        assert_eq!(value, Attempt::Failure(250.0));

        Ok(())
    }
}

//! Defines the `RuleSet` field for the `meets` table and CONFIG files.

use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::fmt;
use std::str::FromStr;

/// A rule of competition.
///
/// By default, all equipment divisions are assumed to be separate.
#[derive(Copy, Clone, Debug, Deserialize, Display, EnumString, PartialEq, Eq, Serialize)]
pub enum Rule {
    /// Lifters in "Raw" and "Wraps" compete in the same category.
    CombineRawAndWraps,

    /// Lifters in "Single-ply" and "Multi-ply" compete in the same category.
    CombineSingleAndMulti,

    /// There was no equipment category: everyone competed together.
    CombineAllEquipment,

    /// Fourth attempts can be lower than other attempts.
    FourthAttemptsMayLower,
}

/// Packed storage for all active RuleSet.
///
/// There are two equivalent ways RuleSet may be stored:
///
/// 1. When stored textually, rules should be in one string, separated by spaces.
///    This is how rules are specified in meet.csv files, for example.
///
/// 2. When serialized by the compiler for the server, rules may be stored as
///    a simple number, representing a bitfield of Rules. This is to save space,
///    since the RuleSet field will be attached to each meet, and the Rule strings
///    themselves are long.
///
/// It's expected that the human-consumable openpowerlifting.csv will not include
/// the RuleSet of each meet, and therefore it's safe to serialize to a number.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RuleSet(u32);

#[derive(Copy, Clone, Debug, Display, PartialEq, Eq)]
pub enum RuleSetParseError {
    UnknownRule,
}

impl RuleSet {
    /// Whether a given Rule is active.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Rule, RuleSet};
    /// let ruleset = "CombineRawAndWraps".parse::<RuleSet>().unwrap();
    /// assert!(ruleset.contains(Rule::CombineRawAndWraps));
    /// assert!(!ruleset.contains(Rule::CombineSingleAndMulti));
    /// ```
    pub fn contains(self, rule: Rule) -> bool {
        self.0 & (1 << (rule as u32)) != 0
    }

    /// Adds a given Rule to the set.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::{Rule, RuleSet};
    /// let mut ruleset = RuleSet::default();
    /// ruleset.add(Rule::CombineSingleAndMulti);
    /// assert!(!ruleset.contains(Rule::CombineRawAndWraps));
    /// assert!(ruleset.contains(Rule::CombineSingleAndMulti));
    /// ```
    pub fn add(&mut self, rule: Rule) {
        self.0 |= 1 << (rule as u32);
    }
}

impl Serialize for RuleSet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            // Output nothing instead of zero to save some space.
            serializer.serialize_str("")
        } else {
            serializer.serialize_u32(self.0)
        }
    }
}

impl FromStr for RuleSet {
    type Err = RuleSetParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // The empty string corresponds to default rules.
        if s.is_empty() {
            return Ok(RuleSet::default());
        }

        // If specifed as a number, import the number directly.
        if let Ok(n) = s.parse::<u32>() {
            return Ok(RuleSet(n));
        }

        // Otherwise assume it's a space-delimited string.
        let mut ruleset = RuleSet::default();
        for substr in s.split(' ') {
            if let Ok(rule) = substr.parse::<Rule>() {
                ruleset.add(rule);
            } else {
                return Err(RuleSetParseError::UnknownRule);
            }
        }
        Ok(ruleset)
    }
}

struct RuleSetVisitor;

impl Visitor<'_> for RuleSetVisitor {
    type Value = RuleSet;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a space-separated list of rules")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<RuleSet, E> {
        RuleSet::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for RuleSet {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<RuleSet, D::Error> {
        deserializer.deserialize_str(RuleSetVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rule_basic() {
        let rule = "CombineRawAndWraps".parse::<Rule>().unwrap();
        assert_eq!(rule, Rule::CombineRawAndWraps);
        let rule = "CombineSingleAndMulti".parse::<Rule>().unwrap();
        assert_eq!(rule, Rule::CombineSingleAndMulti);
    }

    #[test]
    fn basic() {
        let ruleset = "CombineRawAndWraps".parse::<RuleSet>().unwrap();
        assert_eq!(ruleset.contains(Rule::CombineRawAndWraps), true);
        assert_eq!(ruleset.contains(Rule::CombineSingleAndMulti), false);

        let s = "CombineRawAndWraps CombineSingleAndMulti";
        let ruleset = s.parse::<RuleSet>().unwrap();
        assert_eq!(ruleset.contains(Rule::CombineRawAndWraps), true);
        assert_eq!(ruleset.contains(Rule::CombineSingleAndMulti), true);

        let ruleset = "".parse::<RuleSet>().unwrap();
        assert_eq!(ruleset.contains(Rule::CombineRawAndWraps), false);
        assert_eq!(ruleset.contains(Rule::CombineSingleAndMulti), false);
    }

    /// This test hardcodes the ordering of the Rule enum, so it may break.
    #[test]
    fn parses_from_u32() {
        let ruleset = "2".parse::<RuleSet>().unwrap();
        assert_eq!(ruleset.contains(Rule::CombineRawAndWraps), false);
        assert_eq!(ruleset.contains(Rule::CombineSingleAndMulti), true);
    }

    #[test]
    fn errors() {
        let s = "CombineFloobAndBleeb";
        assert!(s.parse::<RuleSet>().is_err());

        let s = "CombineRawAndWraps CombineFloobAndBleeb";
        assert!(s.parse::<RuleSet>().is_err());

        let s = " CombineRawAndWraps";
        assert!(s.parse::<RuleSet>().is_err());

        let s = "-0";
        assert!(s.parse::<RuleSet>().is_err());
    }
}

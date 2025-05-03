//! Defines fields that represent points.

use arrayvec::ArrayString;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::f32;
use std::fmt::{self, Write};
use std::num;
use std::str::FromStr;

/// Represents numbers describing points, like Wilks and Glossbrenner.
///
/// The database only tracks points to two decimal places.
/// Instead of storing as `f32`, we can store as `u32 * 100`,
/// allowing the use of normal registers for what are effectively
/// floating-point operations, and removing all `dtoa()` calls.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialOrd, PartialEq)]
pub struct Points(i32);

/// Enum of known powerlifting points systems, like Wilks and Glossbrenner.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PointsSystem {
    AH,
    Dots,
    Glossbrenner,
    Goodlift,
    IPFPoints,
    McCulloch,
    NASA,
    Reshel,
    SchwartzMalone,
    Wilks,
    Wilks2020,

    /// Expresses that the meet doesn't use a points system.
    Total,
}

impl From<f32> for Points {
    fn from(f: f32) -> Points {
        if f.is_finite() {
            Points((f * 100.0).round() as i32)
        } else {
            Points(0)
        }
    }
}

impl From<f64> for Points {
    fn from(f: f64) -> Points {
        if f.is_finite() {
            Points((f * 100.0).round() as i32)
        } else {
            Points(0)
        }
    }
}

impl From<Points> for f64 {
    fn from(p: Points) -> f64 {
        f64::from(p.0) / 100.0
    }
}

impl Points {
    #[inline]
    pub fn from_i32(i: i32) -> Points {
        Points(i * 100)
    }

    // TODO: Remove this duplicate code.
    pub fn format_comma(self) -> String {
        // Don't display empty points.
        if self.0 == 0 {
            String::new()
        } else {
            // Displaying points always shows two decimal places.
            let integer = self.0 / 100;
            let decimal = self.0.abs() % 100;
            format!("{integer},{decimal:02}")
        }
    }
}

impl fmt::Display for Points {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Don't display empty points.
        if self.0 == 0 {
            Ok(())
        } else {
            // Displaying points always shows two decimal places.
            let integer = self.0 / 100;
            let decimal = self.0.abs() % 100;
            write!(f, "{integer}.{decimal:02}")
        }
    }
}

impl FromStr for Points {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(Points(0))
        } else {
            Ok(Points::from(s.parse::<f32>()?))
        }
    }
}

impl Serialize for Points {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // 10 characters for the non-decimal number (-536870912).
        // 3 characters for the '.' plus 2 fractional digits.
        let mut buf = ArrayString::<13>::new();
        write!(buf, "{self}").expect("ArrayString overflow");
        serializer.serialize_str(&buf)
    }
}

struct PointsVisitor;

impl Visitor<'_> for PointsVisitor {
    type Value = Points;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A floating-point value or the empty string.")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Points, E> {
        Points::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Points {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Points, D::Error> {
        deserializer.deserialize_str(PointsVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let w = "".parse::<Points>().unwrap();
        assert!(w.0 == 0);

        let w = "789".parse::<Points>().unwrap();
        assert!(w.0 == 78900);

        let w = "123.45".parse::<Points>().unwrap();
        assert!(w.0 == 12345);

        let w = "-123.45".parse::<Points>().unwrap();
        assert!(w.0 == -12345);
    }

    #[test]
    fn f32_edgecases() {
        // Test some special f32 values.
        let w = "-0".parse::<Points>().unwrap();
        assert!(w.0 == 0);

        let w = "NaN".parse::<Points>().unwrap();
        assert!(w.0 == 0);

        let w = format!("{}", f32::INFINITY).parse::<Points>().unwrap();
        assert!(w.0 == 0);

        let w = format!("{}", f32::NEG_INFINITY).parse::<Points>().unwrap();
        assert!(w.0 == 0);
    }

    #[test]
    fn rounding() {
        // If extra decimal numbers are reported, round appropriately.
        let w = "123.456".parse::<Points>().unwrap();
        assert!(w.0 == 12346);
        let w = "-123.456".parse::<Points>().unwrap();
        assert!(w.0 == -12346);
    }

    #[test]
    fn errors() {
        assert!("..".parse::<Points>().is_err());
        assert!("123.45.6".parse::<Points>().is_err());
        assert!("notafloat".parse::<Points>().is_err());
        assert!("--123".parse::<Points>().is_err());
    }

    #[test]
    fn display() {
        let w = "123.456".parse::<Points>().unwrap();
        assert_eq!(format!("{w}"), "123.46");

        let w = "100.456".parse::<Points>().unwrap();
        assert_eq!(format!("{w}"), "100.46");

        let w = "100.056".parse::<Points>().unwrap();
        assert_eq!(format!("{w}"), "100.06");

        let w = "-123.456".parse::<Points>().unwrap();
        assert_eq!(format!("{w}"), "-123.46");

        let w = "-123.000".parse::<Points>().unwrap();
        assert_eq!(format!("{w}"), "-123.00");

        let w = "-0.000".parse::<Points>().unwrap();
        assert_eq!(format!("{w}"), "");
    }

    #[test]
    fn ordering() {
        let w1 = "100".parse::<Points>().unwrap();
        let w2 = "200".parse::<Points>().unwrap();
        assert!(w1 < w2);
        assert!(w1.lt(&w2));
    }
}

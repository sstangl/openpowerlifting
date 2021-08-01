//! Defines fields that represent weights.

use crate::Points;

use arrayvec::ArrayString;
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::f32;
use std::fmt::{self, Write};
use std::num;
use std::ops;
use std::str::FromStr;

use crate::WeightUnits;

/// Represents numbers describing absolute weights.
///
/// The database only tracks weights to two decimal places.
/// Instead of storing as `f32`, we can store as `i32 * 100`,
/// allowing the use of normal registers for what are effectively
/// floating-point operations, and removing all `dtoa()` calls.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct WeightKg(i32);

/// Represents numbers describing absolute weights in their final
/// format for printing (either Kg or Lbs).
///
/// Because the type of the weight is forgotten, these weights
/// are incomparable with each other.
#[derive(Copy, Clone, Debug)]
pub struct WeightAny(i32);

impl Serialize for WeightKg {
    /// Serialize with two decimal places, exactly as in the original.
    ///
    /// This is intended for use by the compiler when writing the entries.csv.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Zero serializes as the empty string.
        if self.0 == 0 {
            return serializer.serialize_str("");
        }

        // 10 characters for the non-decimal number (-536870912).
        // 3 characters for the '.' plus 2 fractional digits.
        let mut buf = ArrayString::<13>::new();

        let integer = self.0 / 100;
        let fraction = self.0.abs() % 100;

        write!(buf, "{}", integer).expect("ArrayString overflow");
        if fraction != 0 {
            if fraction % 10 == 0 {
                // Serialize "50" as ".5".
                write!(buf, ".{}", fraction / 10).expect("ArrayString overflow");
            } else {
                // Serialize "5" as ".05".
                write!(buf, ".{:0>2}", fraction).expect("ArrayString overflow");
            }
        }

        serializer.serialize_str(&buf)
    }
}

impl Serialize for WeightAny {
    /// Serialize with one decimal place, pretty-printed.
    ///
    /// This is valid since WeightAny is intended only for situations
    /// in which pretty weights should be displayed.
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0 == 0 {
            return serializer.serialize_str("");
        }

        // Over-estimate of space required, just to future-proof it.
        let mut buf = ArrayString::<13>::new();
        write!(buf, "{}", self).expect("ArrayString overflow");

        serializer.serialize_str(&buf)
    }
}

impl From<WeightKg> for f32 {
    fn from(w: WeightKg) -> f32 {
        (w.0 as f32) / 100.0
    }
}

impl From<WeightKg> for f64 {
    fn from(w: WeightKg) -> f64 {
        f64::from(w.0) / 100.0
    }
}

impl From<WeightKg> for Option<f64> {
    fn from(w: WeightKg) -> Option<f64> {
        if w.is_zero() {
            None
        } else {
            Some(w.into())
        }
    }
}

impl From<WeightAny> for f32 {
    fn from(w: WeightAny) -> f32 {
        (w.0 as f32) / 100.0
    }
}

impl From<WeightAny> for f64 {
    fn from(w: WeightAny) -> f64 {
        f64::from(w.0) / 100.0
    }
}

impl From<WeightAny> for Option<f64> {
    fn from(w: WeightAny) -> Option<f64> {
        if w.is_zero() {
            None
        } else {
            Some(w.into())
        }
    }
}

impl WeightKg {
    #[inline]
    pub fn max_value() -> WeightKg {
        WeightKg(<i32>::max_value())
    }

    #[inline]
    pub const fn from_i32(i: i32) -> WeightKg {
        WeightKg(i * 100)
    }

    // This only exists because from_f32() can't be const fn at the moment.
    #[inline]
    pub const fn from_raw(i: i32) -> WeightKg {
        WeightKg(i)
    }

    #[inline]
    pub fn from_f32(f: f32) -> WeightKg {
        if f.is_finite() {
            WeightKg((f * 100.0).round() as i32)
        } else {
            WeightKg(0)
        }
    }

    /// Whether the weight is negative, representing a failed lift.
    #[inline]
    pub fn is_failed(self) -> bool {
        self < WeightKg::from_i32(0)
    }

    /// Whether the weight is zero, representing a lift not taken.
    #[inline]
    pub fn is_zero(self) -> bool {
        self == WeightKg::from_i32(0)
    }

    /// Whether the weight is not zero, representing a taken lift.
    #[inline]
    pub fn is_non_zero(self) -> bool {
        self != WeightKg::from_i32(0)
    }

    pub fn as_kg(self) -> WeightAny {
        WeightAny(self.0)
    }

    pub fn as_lbs(self) -> WeightAny {
        let f = (self.0.abs() as f32) * 2.2046225; // Max precision for f32.

        // Round to the hundredth place.
        // Half-way cases are rounded away from zero.
        let mut rounded = f.round() as i32;

        // Pounds values tend to be reported only to the nearest tenth.
        // If the fractional part is close to another tenth, add a correction.
        if (rounded % 10) == 9 {
            rounded += 1;
        }

        if self.0.is_positive() {
            WeightAny(rounded)
        } else {
            WeightAny(-rounded)
        }
    }

    /// Report as the "common name" of the weight class.
    pub fn as_lbs_class(self) -> WeightAny {
        let lbs = self.as_lbs();
        let truncated: i32 = (lbs.0 / 100) * 100;

        match truncated {
            182_00 => WeightAny(183_00),
            _ => WeightAny(truncated),
        }
    }

    pub fn as_type(self, unit: WeightUnits) -> WeightAny {
        match unit {
            WeightUnits::Kg => self.as_kg(),
            WeightUnits::Lbs => self.as_lbs(),
        }
    }
}

impl fmt::Display for WeightKg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        WeightAny(self.0).fmt(f)
    }
}

// Operators for WeightKg.

/// Addition between WeightKg objects.
impl ops::Add<WeightKg> for WeightKg {
    type Output = WeightKg;

    fn add(self, _rhs: WeightKg) -> WeightKg {
        WeightKg(self.0 + _rhs.0)
    }
}

/// += operator for WeightKg.
impl ops::AddAssign for WeightKg {
    fn add_assign(&mut self, other: WeightKg) {
        *self = *self + other
    }
}

/// Subtraction between WeightKg objects.
impl ops::Sub<WeightKg> for WeightKg {
    type Output = WeightKg;

    fn sub(self, _rhs: WeightKg) -> WeightKg {
        WeightKg(self.0 - _rhs.0)
    }
}

/// -= operator for WeightKg.
impl ops::SubAssign for WeightKg {
    fn sub_assign(&mut self, other: WeightKg) {
        *self = *self - other
    }
}

/// Absolute value.
impl WeightKg {
    pub fn abs(self) -> WeightKg {
        WeightKg(self.0.abs())
    }
}

impl WeightAny {
    /// Whether the weight is zero, representing a lift not taken.
    #[inline]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Whether the weight is not zero, representing a taken lift.
    #[inline]
    pub fn is_non_zero(self) -> bool {
        !self.is_zero()
    }

    // FIXME -- remove code duplication with fmt() somehow.
    pub fn format_comma(self) -> String {
        // Don't display empty weights.
        if self.0 == 0 {
            String::new()
        } else {
            // Displaying a weight only shows a single decimal place.
            // Truncate the last number.
            let integer = self.0 / 100;
            let decimal = (self.0.abs() % 100) / 10;

            // If the decimal can be avoided, don't write it.
            if decimal != 0 {
                format!("{},{}", integer, decimal)
            } else {
                format!("{}", integer)
            }
        }
    }

    /// Used to reinterpret the weight as points, for `PointsSystem::Total`.
    pub fn as_points(self) -> Points {
        Points::from_i32(self.0)
    }
}

impl fmt::Display for WeightAny {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Don't display empty weights.
        if self.0 == 0 {
            Ok(())
        } else {
            // Displaying a weight only shows a single decimal place.
            // Truncate the last number.
            let integer = self.0 / 100;
            let decimal = (self.0.abs() % 100) / 10;

            // If the decimal can be avoided, don't write it.
            if decimal != 0 {
                write!(f, "{}.{}", integer, decimal)
            } else {
                write!(f, "{}", integer)
            }
        }
    }
}

impl FromStr for WeightKg {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Ok(WeightKg(0))
        } else {
            Ok(WeightKg::from_f32(s.parse::<f32>()?))
        }
    }
}

struct WeightKgVisitor;

impl<'de> Visitor<'de> for WeightKgVisitor {
    type Value = WeightKg;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A floating-point value or the empty string.")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<WeightKg, E> {
        WeightKg::from_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for WeightKg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<WeightKg, D::Error> {
        deserializer.deserialize_str(WeightKgVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weightkg_basic() {
        let w = "".parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = "789".parse::<WeightKg>().unwrap();
        assert!(w.0 == 78900);

        let w = "123.45".parse::<WeightKg>().unwrap();
        assert!(w.0 == 12345);

        let w = "-123.45".parse::<WeightKg>().unwrap();
        assert!(w.0 == -12345);
    }

    #[test]
    fn test_weightkg_f32_edgecases() {
        // Test some special f32 values.
        let w = "-0".parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = "NaN".parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = format!("{}", f32::INFINITY).parse::<WeightKg>().unwrap();
        assert!(w.0 == 0);

        let w = format!("{}", f32::NEG_INFINITY)
            .parse::<WeightKg>()
            .unwrap();
        assert!(w.0 == 0);
    }

    #[test]
    fn test_weightkg_rounding() {
        // If extra decimal numbers are reported, round appropriately.
        let w = "123.456".parse::<WeightKg>().unwrap();
        assert!(w.0 == 12346);
        let w = "-123.456".parse::<WeightKg>().unwrap();
        assert!(w.0 == -12346);
    }

    /// Some results that are initially reported in LBS wind
    /// up giving slightly-under Kg values.
    #[test]
    fn test_weightkg_as_lbs_rounding() {
        // 1709.99 lbs (reported by federation as 1710).
        let w = "775.64".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, 1710_00);

        // 1710.02 lbs should be unchanged, since that's over 1710.
        // If it really was 1710, it would have been "775.64".
        let w = "775.65".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, 1710_02);

        // 434.99 lbs (reported by federation as 435).
        let w = "197.31".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, 435_00);

        // 240.4 lbs should be unchanged.
        let w = "109.04".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, 240_40);

        // 317.5 should be just under 700lbs.
        let w = "317.5".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, 699_97);

        // Failed lifts should round the same as successful lifts.
        let w = "340.19".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, 750_00);
        let w = "-340.19".parse::<WeightKg>().unwrap();
        assert_eq!(w.as_lbs().0, -750_00);
    }

    #[test]
    fn test_weightkg_errors() {
        assert!("..".parse::<WeightKg>().is_err());
        assert!("123.45.6".parse::<WeightKg>().is_err());
        assert!("notafloat".parse::<WeightKg>().is_err());
        assert!("--123".parse::<WeightKg>().is_err());
    }

    #[test]
    fn test_weightkg_display() {
        let w = "123.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "123.4");

        let w = "100.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "100.4");

        let w = "100.056".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "100");

        let w = "-123.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "-123.4");

        let w = "-123.000".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "-123");

        let w = "-0.000".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{}", w), "");
    }

    /// Ensures that WeightKg serialization matches the original to 2 decimal places.
    ///
    /// Serialization is performed by the compiler.
    #[test]
    fn test_weightkg_serialize() {
        let w = "0.00".parse::<WeightKg>().unwrap();
        assert_eq!(w.0, 0_00);
        assert_eq!(json!(w), "");

        let w = "109.04".parse::<WeightKg>().unwrap(); // Issue 2941.
        assert_eq!(w.0, 109_04);
        assert_eq!(json!(w), "109.04");

        let w = "109.40".parse::<WeightKg>().unwrap();
        assert_eq!(w.0, 109_40);
        assert_eq!(json!(w), "109.4");

        let w = "200.00".parse::<WeightKg>().unwrap();
        assert_eq!(w.0, 200_00);
        assert_eq!(json!(w), "200");
    }

    #[test]
    fn test_weightkg_ordering() {
        let w1 = "100".parse::<WeightKg>().unwrap();
        let w2 = "200".parse::<WeightKg>().unwrap();
        assert!(w1 < w2);
    }
}

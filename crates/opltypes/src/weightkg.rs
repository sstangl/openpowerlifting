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

/// Represents numbers describing absolute weights in kilograms.
///
/// The database only tracks weights to two decimal places.
/// Instead of storing as `f32`, we can store as `i32 * 100`,
/// allowing the use of normal registers for what are effectively
/// floating-point operations, and removing all `dtoa()` calls.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct WeightKg(i32);

/// Represents numbers describing absolute weights in pounds.
///
/// This type exists to facilitate easy conversion to `WeightKg` or `WeightAny`.
/// In general, weight values should be held in kilograms.
///
/// Conversion between kilograms and pounds is lossy due to rounding.
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct WeightLbs(i32);

/// Represents unit-less numbers describing absolute weights (either Kg or Lbs).
///
/// Forgetting the unit is suitable for circumstances in which the value only
/// needs to be rendered to the user, for example when printing.
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

        write!(buf, "{integer}").expect("ArrayString overflow");
        if fraction != 0 {
            if fraction % 10 == 0 {
                // Serialize "50" as ".5".
                write!(buf, ".{}", fraction / 10).expect("ArrayString overflow");
            } else {
                // Serialize "5" as ".05".
                write!(buf, ".{fraction:0>2}").expect("ArrayString overflow");
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
        write!(buf, "{self}").expect("ArrayString overflow");

        serializer.serialize_str(&buf)
    }
}

// Simple format conversions.
impl From<WeightKg> for f32 {
    #[inline]
    fn from(w: WeightKg) -> f32 {
        (w.0 as f32) / 100.0
    }
}
impl From<WeightKg> for f64 {
    #[inline]
    fn from(w: WeightKg) -> f64 {
        f64::from(w.0) / 100.0
    }
}
impl From<WeightLbs> for f32 {
    #[inline]
    fn from(w: WeightLbs) -> f32 {
        (w.0 as f32) / 100.0
    }
}
impl From<WeightLbs> for f64 {
    #[inline]
    fn from(w: WeightLbs) -> f64 {
        f64::from(w.0) / 100.0
    }
}

// Conversions between different representations.
impl From<WeightKg> for WeightLbs {
    fn from(value: WeightKg) -> WeightLbs {
        let f = (value.0.abs() as f32) * 2.2046225; // Max precision for f32.

        // Round to the hundredth place.
        // Half-way cases are rounded away from zero.
        let mut rounded = f.round() as i32;

        // Pounds values tend to be reported only to the nearest tenth.
        // If the fractional part is close to another tenth, add a correction.
        if (rounded % 10) == 9 {
            rounded += 1;
        }

        if value.0.is_positive() {
            WeightLbs(rounded)
        } else {
            WeightLbs(-rounded)
        }
    }
}
impl From<WeightLbs> for WeightKg {
    fn from(value: WeightLbs) -> Self {
        let f = (value.0.abs() as f32) / 2.2046225; // Max precision for f32.
        let rounded = f.round() as i32; // TODO: Is this rounding mode OK? Test round-trip?

        if value.0.is_positive() {
            WeightKg(rounded)
        } else {
            WeightKg(-rounded)
        }
    }
}
impl From<WeightKg> for WeightAny {
    #[inline]
    fn from(value: WeightKg) -> WeightAny {
        WeightAny(value.0)
    }
}
impl From<WeightLbs> for WeightAny {
    #[inline]
    fn from(value: WeightLbs) -> WeightAny {
        WeightAny(value.0)
    }
}

impl WeightKg {
    /// The highest representable value a `WeightKg` can have.
    pub const MAX: WeightKg = WeightKg(i32::MAX);

    #[inline]
    pub const fn from_i32(i: i32) -> WeightKg {
        WeightKg(i.saturating_mul(100))
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

    #[inline]
    pub fn from_f64(f: f64) -> WeightKg {
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

    /// Returns the weight in pounds.
    #[inline]
    pub fn as_lbs(self) -> WeightLbs {
        WeightLbs::from(self)
    }

    /// Returns the weight without units.
    #[inline]
    pub fn as_any(self) -> WeightAny {
        WeightAny::from(self)
    }

    /// Returns the weight without units, after converting to the given unit.
    pub fn as_type(self, unit: WeightUnits) -> WeightAny {
        match unit {
            WeightUnits::Kg => self.as_any(),
            WeightUnits::Lbs => self.as_lbs().as_any(),
        }
    }
}

impl WeightLbs {
    #[inline]
    pub const fn from_i32(i: i32) -> WeightLbs {
        WeightLbs(i.saturating_mul(100))
    }

    /// Returns the weight in kilograms.
    #[inline]
    pub fn as_kg(self) -> WeightKg {
        WeightKg::from(self)
    }

    /// Returns the weight without units.
    #[inline]
    pub fn as_any(self) -> WeightAny {
        WeightAny::from(self)
    }

    /// Report as the "common name" of the weight class.
    pub fn as_class(self) -> WeightAny {
        let truncated: i32 = (self.0 / 100) * 100;
        match truncated {
            182_00 => WeightAny(183_00),
            _ => WeightAny(truncated),
        }
    }
}

impl fmt::Display for WeightKg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        WeightAny(self.0).fmt(f)
    }
}
impl fmt::Display for WeightLbs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        WeightAny(self.0).fmt(f)
    }
}

// Operators for WeightKg.

/// Addition between WeightKg objects.
impl ops::Add<WeightKg> for WeightKg {
    type Output = WeightKg;

    fn add(self, rhs: WeightKg) -> WeightKg {
        WeightKg(self.0 + rhs.0)
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

    fn sub(self, rhs: WeightKg) -> WeightKg {
        WeightKg(self.0 - rhs.0)
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
                format!("{integer},{decimal}")
            } else {
                format!("{integer}")
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
                write!(f, "{integer}.{decimal}")
            } else {
                write!(f, "{integer}")
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
impl FromStr for WeightLbs {
    type Err = num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        WeightKg::from_str(s).map(|kg| WeightLbs(kg.0)) // Cast to pounds without conversion.
    }
}

struct WeightKgVisitor;

impl Visitor<'_> for WeightKgVisitor {
    type Value = WeightKg;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number or numeric string")
    }
    fn visit_i64<E: de::Error>(self, i: i64) -> Result<WeightKg, E> {
        let v = i32::try_from(i).map_err(E::custom)?;
        Ok(WeightKg::from_i32(v))
    }
    fn visit_u64<E: de::Error>(self, u: u64) -> Result<WeightKg, E> {
        let v = i32::try_from(u).map_err(E::custom)?;
        Ok(WeightKg::from_i32(v))
    }
    fn visit_f64<E: de::Error>(self, v: f64) -> Result<WeightKg, E> {
        Ok(WeightKg::from_f64(v))
    }
    fn visit_borrowed_str<E: de::Error>(self, v: &str) -> Result<WeightKg, E> {
        WeightKg::from_str(v).map_err(E::custom)
    }
    fn visit_str<E: de::Error>(self, v: &str) -> Result<WeightKg, E> {
        self.visit_borrowed_str(v)
    }
}

impl<'de> Deserialize<'de> for WeightKg {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<WeightKg, D::Error> {
        deserializer.deserialize_any(WeightKgVisitor)
    }
}

impl<'de> Deserialize<'de> for WeightLbs {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<WeightLbs, D::Error> {
        deserializer
            .deserialize_any(WeightKgVisitor)
            .map(|kg| WeightLbs(kg.0)) // Reinterpret the value as pounds without conversion.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
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
    fn f32_edgecases() {
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
    fn rounding() {
        // If extra decimal numbers are reported, round appropriately.
        let w = "123.456".parse::<WeightKg>().unwrap();
        assert!(w.0 == 12346);
        let w = "-123.456".parse::<WeightKg>().unwrap();
        assert!(w.0 == -12346);
    }

    /// Some results that are initially reported in LBS wind
    /// up giving slightly-under Kg values.
    #[test]
    fn as_lbs_rounding() {
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
    fn errors() {
        assert!("..".parse::<WeightKg>().is_err());
        assert!("123.45.6".parse::<WeightKg>().is_err());
        assert!("notafloat".parse::<WeightKg>().is_err());
        assert!("--123".parse::<WeightKg>().is_err());
    }

    #[test]
    fn display() {
        let w = "123.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{w}"), "123.4");

        let w = "100.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{w}"), "100.4");

        let w = "100.056".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{w}"), "100");

        let w = "-123.456".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{w}"), "-123.4");

        let w = "-123.000".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{w}"), "-123");

        let w = "-0.000".parse::<WeightKg>().unwrap();
        assert_eq!(format!("{w}"), "");
    }

    /// Ensures that WeightKg serialization matches the original to 2 decimal places.
    #[test]
    fn serialize() {
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
    fn ordering() {
        let w1 = "100".parse::<WeightKg>().unwrap();
        let w2 = "200".parse::<WeightKg>().unwrap();
        assert!(w1 < w2);
    }
}

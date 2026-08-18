//! Implements Name to Username conversion logic.

use ascii::{AsciiStr, AsciiString, IntoAsciiString, ToAsciiChar};
use serde::de::{self, Deserialize, Visitor};
use serde::ser::Serialize;

use std::fmt;

use crate::writing_system::{WritingSystem, infer_writing_system};

/// A lifter's username.
///
/// Usernames are created from Name fields by:
///  1. Removing non-alphanumeric characters.
///  2. Lowercasing.
///  3. Replacing non-ASCII chars with ASCII lookalikes.
///
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Username(AsciiString);

impl fmt::Display for Username {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Username> for String {
    fn from(u: Username) -> String {
        u.0.into()
    }
}

impl Username {
    /// Converts &self to a &str slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the username as an [AsciiStr] slice.
    pub fn as_ascii_str(&self) -> &AsciiStr {
        &self.0
    }

    /// Returns the length of this [Username] in bytes.
    ///
    /// Since the [Username] is guaranteed to be ASCII, the number of bytes
    /// is also the number of chars.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the [Username] contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Given a UTF-8 Name, create the corresponding ASCII Username.
    ///
    /// Usernames are used throughout the project as unique identifiers
    /// for individual lifters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Username;
    /// let username = Username::from_name("Ed Coan").unwrap();
    /// assert_eq!(username.as_str(), "edcoan");
    /// ```
    pub fn from_name(name: &str) -> Result<Self, String> {
        // Empty names should be invalid, but can occur from user input.
        if name.is_empty() {
            return Ok(Username::default());
        }

        // CJK characters have no canonical ASCII representation, so we use a number.
        let writing_system = infer_writing_system(name);
        if matches!(
            writing_system,
            WritingSystem::Japanese | WritingSystem::CJK | WritingSystem::Korean
        ) {
            let ea_id: String = name
                .chars()
                .filter(|c| !c.is_whitespace())
                .map(hira_to_kata_char)
                .map(|c| (c as u32).to_string())
                .collect();
            let s = format!("ea-{ea_id}");
            return Ok(Username(s.into_ascii_string().unwrap()));
        }

        if writing_system == WritingSystem::Greek {
            return convert_greek_to_ascii(name);
        }

        // Otherwise, the name only has characters that can be converted to ASCII.
        convert_to_ascii(name)
    }

    /// Interprets a [&str] as a [Username]. Used in deserialization.
    ///
    /// This constructor does not perform any validation that the username is well-formed.
    pub fn from_trusted_str(s: &str) -> Result<Self, ascii::FromAsciiError<&str>> {
        Ok(Username(s.into_ascii_string()?))
    }

    /// Returns the base name and variant of a [Username], if applicable.
    ///
    /// # Examples
    ///
    /// ```
    /// # use opltypes::Username;
    /// let u = Username::from_name("John Doe").unwrap();
    /// let (base, variant) = u.to_parts();
    /// assert_eq!(base.as_str(), "johndoe");
    /// assert_eq!(variant, None);
    ///
    /// let u = Username::from_name("John Doe #1").unwrap();
    /// let (base, variant) = u.to_parts();
    /// assert_eq!(base.as_str(), "johndoe");
    /// assert_eq!(variant, Some(1));
    /// ```
    pub fn to_parts(&self) -> (&AsciiStr, Option<u32>) {
        // Common case first: if no digit at end, it's not a variant.
        if let Some(ascii_char) = self.0.last() {
            if !ascii_char.is_ascii_digit() {
                return (&self.0, None);
            }
        } else {
            return (&self.0, None); // Username was the empty string.
        }

        // Slow case: the username ends with a digit.
        //
        // If the username begins with "ea-", then it's an East Asian name
        // that is numerically encoded and cannot be disambiguated.
        if self.0.as_str().starts_with("ea-") {
            return (&self.0, None);
        }

        // Definitely a variant.
        //
        // Walk the string backwards looking for the first index
        // of an ASCII digit.
        let mut start: usize = self.len() - 1;
        for i in (0..start).rev() {
            if self.0[i].is_ascii_digit() {
                start = i;
            } else {
                break;
            }
        }

        let variant = self.0.as_str()[start..].parse::<u32>().unwrap_or(0);
        (&self.0[0..start], Some(variant))
    }

    /// Returns whether the username has a disambiguation variant.
    pub fn has_variant(&self) -> bool {
        let (_base, maybe_variant) = self.to_parts();
        maybe_variant.is_some()
    }

    /// Returns the username without the disambiguation variant, if present.
    ///
    /// For example, `lifter2` becomes `lifter`.
    pub fn without_variant(&self) -> Username {
        let (base, _maybe_variant) = self.to_parts();
        Username(base.to_ascii_string())
    }

    /// Returns the username with the specified disambiguation variant.
    pub fn with_variant(&self, variant: u32) -> Username {
        let (base, _maybe_old_variant) = self.to_parts();
        Username::from_trusted_str(&format!("{base}{variant}")).unwrap()
    }
}

impl Serialize for Username {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

struct UsernameVisitor;
impl Visitor<'_> for UsernameVisitor {
    type Value = Username;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an ASCII string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Username, E> {
        Username::from_trusted_str(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Username {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_str(UsernameVisitor)
    }
}

/// Converts Greek characters into Latin characters following ISO 843, but ASCII.
///
/// <https://en.wikipedia.org/wiki/ISO_843>
fn convert_greek_to_ascii(greek_name: &str) -> Result<Username, String> {
    let mut ascii_name = AsciiString::with_capacity(greek_name.len());

    for letter in greek_name.to_lowercase().chars() {
        if is_exception(letter) {
            continue;
        }

        // Push ASCII characters. This accounts for disambiguation numbers.
        if let Ok(ascii) = letter.to_ascii_char()
            && ascii.is_alphanumeric()
        {
            ascii_name.push(ascii);
            continue;
        }

        let s: &str = match letter {
            'α' | 'ά' => "a",
            'β' => "v",
            'γ' => "g",
            'δ' => "d",
            'ε' => "e",
            'ζ' => "z",
            'η' => "i",
            'θ' => "th",
            'ι' | 'ί' | 'ϊ' | 'ΐ' => "i",
            'κ' => "k",
            'λ' => "l",
            'μ' => "m",
            'ν' => "n",
            'ξ' => "x",
            'ο' | 'ό' => "o",
            'π' => "p",
            'ρ' => "r",
            'σ' | 'ς' => "s",
            'τ' => "t",
            'υ' | 'ύ' | 'ϋ' | 'ΰ' => "y",
            'φ' => "f",
            'χ' => "ch",
            'ψ' => "ps",
            'ω' | 'ώ' => "o",
            _ => {
                return Err(format!(
                    "Unknown Greek character '{letter}' ({letter:?}) in '{}'",
                    greek_name.to_lowercase()
                ));
            }
        };
        // Safe: the limited set of ASCII targets above are indeed ASCII chars.
        ascii_name.push_str(unsafe { AsciiStr::from_ascii_unchecked(s.as_bytes()) });
    }
    Ok(Username(ascii_name))
}

/// Calculates the ASCII equivalent of a Name.
fn convert_to_ascii(name: &str) -> Result<Username, String> {
    let mut ascii_name = AsciiString::with_capacity(name.len());

    // The to_lowercase call uses extra heap memory,
    // but I haven't come up with a better way of doing this right now,
    // since lowercase letters can take up more space than uppercase ones.
    for letter in name.to_lowercase().chars() {
        // Ignore punctuation.
        if is_exception(letter) {
            continue;
        }

        // Push ASCII characters. This accounts for disambiguation numbers.
        if let Ok(ascii) = letter.to_ascii_char()
            && ascii.is_alphanumeric()
        {
            ascii_name.push(ascii);
            continue;
        }

        // A single UTF-8 char can expand to multiple ASCII chars.
        let s: &str = match letter {
            'á' | 'ä' | 'å' | 'ą' | 'ã' | 'à' | 'â' | 'ā' | 'ắ' | 'ấ' | 'ầ' | 'ặ' | 'ạ' | 'ă'
            | 'ả' | 'ậ' | 'ằ' | 'ẩ' => "a",
            'æ' => "ae",
            'ć' | 'ç' | 'č' | 'ĉ' | 'ċ' => "c",
            'đ' | 'ð' | 'ď' => "d",
            'é' | 'ê' | 'ë' | 'è' | 'ě' | 'ę' | 'ē' | 'ế' | 'ễ' | 'ể' | 'ề' | 'ệ' | 'ė' | 'ə' => {
                "e"
            }
            'ğ' | 'ģ' => "g",
            'î' | 'í' | 'ï' | 'ì' | 'ї' | 'ī' | 'ĩ' | 'ị' | 'ı' | 'į' => "i",
            'ķ' => "k",
            'ľ' | 'ĺ' | 'ļ' | 'ŀ' | 'ł' => "l",
            'ñ' | 'ń' | 'ň' | 'ņ' => "n",
            'ø' | 'ô' | 'ö' | 'ó' | 'ő' | 'õ' | 'ò' | 'ỗ' | 'ọ' | 'ơ' | 'ồ' | 'ớ' | 'ố' | 'ō'
            | 'ŏ' | 'ờ' | 'ộ' | 'ợ' => "o",
            'ř' => "r",
            'ß' => "ss",
            'š' | 'ś' | 'ș' | 'ş' => "s",
            'ț' | 'ť' | 'ţ' => "t",
            'þ' => "th",
            'ü' | 'ů' | 'ú' | 'ù' | 'ū' | 'ű' | 'ư' | 'ứ' | 'ũ' | 'ữ' | 'ự' | 'ừ' | 'ử' => {
                "u"
            }
            'ý' | 'ỳ' | 'ỹ' | 'ỷ' | 'ұ' => "y",
            'ž' | 'ż' | 'ź' => "z",
            '\u{307}' => "", // A Turkish critical mark.
            _ => {
                return Err(format!(
                    "Unknown Latin character '{letter}' ({letter:?}) in '{}'",
                    name.to_lowercase()
                ));
            }
        };

        // Safe: the limited set of ASCII targets above are indeed ASCII chars.
        ascii_name.push_str(unsafe { AsciiStr::from_ascii_unchecked(s.as_bytes()) });
    }
    Ok(Username(ascii_name))
}

/// Whether the character should be silently omitted.
fn is_exception(letter: char) -> bool {
    matches!(letter, ' ' | '\\' | '#' | '.' | '-' | '\'')
}

const HIRAGANA_START: u32 = 0x3041;
const HIRAGANA_END: u32 = 0x3096;
const KATAKANA_START: u32 = 0x30A1;

/// Returns the character, converting any Hiragana to Katakana.
///
/// Hiragana characters are always a single Unicode scalar value.
/// When changing this function, also change the test update hira_to_kata_char_is_safe().
fn hira_to_kata_char(c: char) -> char {
    let scalar = c as u32;
    if (HIRAGANA_START..=HIRAGANA_END).contains(&scalar) {
        // Shift from the Hiragana list to the equivalent Katakana list.
        let kata_scalar = scalar + (KATAKANA_START - HIRAGANA_START);
        // Safe because of the bounds checking above.
        // Safety is asserted by the test "hira_to_kata_char_is_safe()" below.
        unsafe { std::char::from_u32_unchecked(kata_scalar) }
    } else {
        c
    }
}

/// Gives the equivalent Katakana for a Hiragana String.
///
/// Currently only used for testing.
#[cfg(test)]
fn hira_to_kata(name: &str) -> String {
    name.chars().map(hira_to_kata_char).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(Username::from_name("").unwrap().as_str(), "");
    }

    #[test]
    fn ascii() {
        assert_eq!(
            Username::from_name("JOHN SMITH").unwrap().as_str(),
            "johnsmith"
        );
        assert_eq!(
            Username::from_name("Petr Petráš").unwrap().as_str(),
            "petrpetras"
        );
        assert_eq!(
            Username::from_name("Auðunn Jónsson").unwrap().as_str(),
            "audunnjonsson"
        );
    }

    /// Tests the conversion of valid GreekName column values to Username.
    #[test]
    fn greek_name() {
        assert_eq!(
            Username::from_name("Αθανασιος Τριαντης").unwrap().as_str(),
            "athanasiostriantis"
        );
        assert_eq!(
            Username::from_name("Νικολαος Χριστοφορακης")
                .unwrap()
                .as_str(),
            "nikolaoschristoforakis"
        );
    }

    /// Tests the conversion of valid JapaneseName column values to Username.
    #[test]
    fn japanese_name() {
        assert_eq!(
            Username::from_name("武田 裕介").unwrap().as_str(),
            "ea-27494300003502920171"
        );
        assert_eq!(
            Username::from_name("光紀 高橋").unwrap().as_str(),
            "ea-20809320003964027211"
        );
    }

    #[test]
    fn japanese_regression() {
        assert!(Username::from_name("佐々木博之").is_ok());
        assert!(Username::from_name("石川記みよ").is_ok());
        assert!(Username::from_name("加藤 みどり").is_ok());
        assert!(Username::from_name("澤山 あおい").is_ok());
        assert!(Username::from_name("ラナ　ヘメンドラ　チャンドラ").is_ok());
        assert!(Username::from_name("宮口 ｼｮｰﾝﾏｷ").is_ok());
        assert!(Username::from_name("みぶ 真也").is_ok());
        assert!(Username::from_name("松浦すぐる").is_ok());
    }

    #[test]
    fn disambig() {
        assert_eq!(
            Username::from_name("John Smith #1").unwrap().as_str(),
            "johnsmith1"
        );
        assert_eq!(
            Username::from_name("Kevin Jäger #1").unwrap().as_str(),
            "kevinjager1"
        );
    }

    #[test]
    fn exception() {
        assert_eq!(
            Username::from_name("Brenda v.d. Meulen").unwrap().as_str(),
            "brendavdmeulen"
        );
        assert_eq!(
            Username::from_name("Aliaksandr Hrynkevich-Sudnik")
                .unwrap()
                .as_str(),
            "aliaksandrhrynkevichsudnik"
        );
    }

    #[test]
    fn invalid_utf8() {
        assert!(Username::from_name("John Smith❤ ").is_err());
    }

    #[test]
    fn invalid_ascii() {
        assert!(Username::from_name("John Smith; ").is_err());
    }

    /// Tests that Hiragana characters are converted to Katakana
    /// for purposes of username comparisons, and that non-Hiragana
    /// characters are left alone.
    #[test]
    fn valid_hira_to_kata() {
        assert!(hira_to_kata("なべ やかん") == "ナベ ヤカン");
        assert!(hira_to_kata("因幡 英昭") == "因幡 英昭");
        assert!(hira_to_kata("ASCII Chars") == "ASCII Chars");
    }

    /// Tests that the limited use of "unsafe" in hira_to_kata_char
    /// is safe for all possible inputs.
    #[test]
    fn hira_to_kata_char_is_safe() {
        for scalar in HIRAGANA_START..=HIRAGANA_END {
            let kata_scalar = scalar + (KATAKANA_START - HIRAGANA_START);
            assert!(std::char::from_u32(kata_scalar).is_some());
        }
    }

    /// Basic test for variant detection.
    #[test]
    fn has_variant() {
        let johnsmith = Username::from_name("John Smith").unwrap();
        let johnsmith2 = Username::from_name("John Smith #2").unwrap();

        assert!(!johnsmith.has_variant());
        assert!(johnsmith2.has_variant());
    }

    /// Tests that the `without_variant` function successfully removes variant info.
    #[test]
    fn without_variant() {
        let johnsmith = Username::from_name("John Smith").unwrap();
        let johnsmith2 = Username::from_name("John Smith #2").unwrap();

        assert_eq!(johnsmith.without_variant().as_str(), "johnsmith");
        assert_eq!(johnsmith2.without_variant().as_str(), "johnsmith");
    }

    /// Tests that the `with_variant` function successfully adds variant info.
    #[test]
    fn with_variant() {
        let johnsmith = Username::from_name("John Smith").unwrap();
        let johnsmith2 = Username::from_name("John Smith #2").unwrap();

        assert_eq!(johnsmith.with_variant(3).as_str(), "johnsmith3");
        assert_eq!(johnsmith2.with_variant(3).as_str(), "johnsmith3");
    }
}

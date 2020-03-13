//! Implements Name to Username conversion logic.

/// Writing systems for characters, for categorization.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum WritingSystem {
    Cyrillic,
    Greek,
    Japanese,
    Korean,
    Latin,
}

impl Default for WritingSystem {
    fn default() -> WritingSystem {
        WritingSystem::Latin
    }
}

/// Calculates the ASCII equivalent of a Name.
fn convert_to_ascii(name: &str) -> Result<String, String> {
    let mut ascii_name = String::with_capacity(name.len());

    // The to_lowercase call uses extra heap memory,
    // but I haven't come up with a better way of doing this right now,
    // since lowercase letters can take up more space than uppercase ones.
    for letter in name.to_lowercase().chars() {
        if is_exception(letter) {
            continue;
        } else if letter.is_alphanumeric() && letter.is_ascii() {
            ascii_name.push(letter);
        } else {
            ascii_name.push_str(match letter {
                'á' | 'ä' | 'å' | 'ą' | 'ã' | 'à' | 'â' | 'ā' | 'ắ' | 'ấ' | 'ầ' | 'ặ' | 'ạ'
                | 'ă' | 'ả' | 'ậ' | 'ằ' => "a",
                'æ' => "ae",
                'ć' | 'ç' | 'č' | 'ĉ' | 'ċ' => "c",
                'đ' | 'ð' | 'ď' => "d",
                'é' | 'ê' | 'ë' | 'è' | 'ě' | 'ę' | 'ē' | 'ế' | 'ễ' | 'ể' | 'ề' | 'ệ' | 'ė'
                | 'ə' => "e",
                'ğ' | 'ģ' => "g",
                'î' | 'í' | 'ï' | 'ì' | 'ї' | 'ī' | 'ĩ' | 'ị' | 'ı' => "i",
                'ķ' => "k",
                'ľ' | 'ĺ' | 'ļ' | 'ŀ' | 'ł' => "l",
                'ñ' | 'ń' | 'ň' | 'ņ' => "n",
                'ø' | 'ô' | 'ö' | 'ó' | 'ő' | 'õ' | 'ò' | 'ỗ' | 'ọ' | 'ơ' | 'ồ' | 'ớ' | 'ố'
                | 'ō' | 'ŏ' | 'ờ' | 'ộ' => "o",
                'ř' => "r",
                'ß' => "ss",
                'š' | 'ś' | 'ș' | 'ş' => "s",
                'ț' | 'ť' | 'ţ' => "t",
                'þ' => "th",
                'ü' | 'ů' | 'ú' | 'ù' | 'ū' | 'ű' | 'ư' | 'ứ' | 'ũ' | 'ữ' | 'ự' | 'ừ' | 'ử' => {
                    "u"
                }
                'ý' | 'ỳ' | 'ỹ' | 'ỷ' => "y",
                'ž' | 'ż' | 'ź' => "z",
                '\u{307}' => "", // A Turkish critical mark.
                _ => {
                    return Err(format!(
                        "Unknown character '{}' ({:?}) in '{}'",
                        letter,
                        letter,
                        name.to_lowercase()
                    ));
                }
            });
        }
    }
    Ok(ascii_name)
}

/// Whether the character should be silently omitted.
fn is_exception(letter: char) -> bool {
    match letter {
        ' ' | '\\' | '#' | '.' | '-' | '\'' => true,
        _ => false,
    }
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
    if scalar >= HIRAGANA_START && scalar <= HIRAGANA_END {
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
    name.chars().map(|c| hira_to_kata_char(c)).collect()
}

/// Get the WritingSystem for the current character.
///
/// Returns `Latin` if unknown.
pub fn get_writing_system(c: char) -> WritingSystem {
    match c as u32 {
        // ASCII. Checking the common case first improves performance.
        0x0..=0x7F => WritingSystem::Latin,
        // Greek.
        0x370..=0x3FF => WritingSystem::Greek,
        // Cyrillic.
        0x400..=0x4FF => WritingSystem::Cyrillic,

        // CJK Radicals Supplement.
        0x2E80..=0x2EFF => WritingSystem::Japanese,
        // Some valid punctuation symbols.
        0x3005..=0x3006 => WritingSystem::Japanese,
        // Hiragana.
        0x3040..=0x309F => WritingSystem::Japanese,
        // Katakana.
        0x30A0..=0x30FF => WritingSystem::Japanese,
        // CJK Unified Ideographs Extension A.
        0x3400..=0x4DBF => WritingSystem::Japanese,
        // CJK Unified Ideographs.
        0x4E00..=0x9FFF => WritingSystem::Japanese,
        // CJK Compatibility Ideographs.
        0xF900..=0xFAFF => WritingSystem::Japanese,
        // CJK Compatibility Forms.
        0xFE30..=0xFE4F => WritingSystem::Japanese,
        // CJK Unified Ideographs Extension B.
        0x20000..=0x2A6DF => WritingSystem::Japanese,
        // CJK Unified Ideographs Extensions C, D, and E.
        0x2A700..=0x2CEAF => WritingSystem::Japanese,
        // CJK Compatibility Ideographs Supplement.
        0x2F800..=0x2FA1F => WritingSystem::Japanese,

        // Hangul Syllables.
        0xAC00..=0xD7AF => WritingSystem::Korean,
        // Hangul Jamo.
        0x1100..=0x11FF => WritingSystem::Korean,
        // Hangul Compatibility Jamo.
        0x3130..=0x318F => WritingSystem::Korean,
        // Hangul Jamo Extended-A.
        0xA960..=0xA97F => WritingSystem::Korean,
        // Hangul Jamo Extended B.
        0xD7B0..=0xD7FF => WritingSystem::Korean,

        // Character is either Latin or not a letter.
        _ => WritingSystem::Latin,
    }
}

/// Returns the likely writing system of a string.
///
/// The first non-Latin character encountered is considered representative.
pub fn infer_writing_system(s: &str) -> WritingSystem {
    s.chars()
        .find_map(|c| match get_writing_system(c) {
            WritingSystem::Latin => None,
            other => Some(other),
        })
        .unwrap_or(WritingSystem::Latin)
}

/// Given a UTF-8 Name, create the corresponding ASCII Username.
///
/// Usernames are used throughout the project as unique identifiers
/// for individual lifters.
///
/// # Examples
///
/// ```
/// # use usernames::make_username;
/// let username = make_username("Ed Coan").unwrap();
/// assert_eq!(username, "edcoan");
/// ```
pub fn make_username(name: &str) -> Result<String, String> {
    if name.is_empty() {
        Ok(String::default())
    } else if infer_writing_system(name) == WritingSystem::Japanese {
        let ea_id: String = name
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| hira_to_kata_char(c))
            .map(|c| (c as u32).to_string())
            .collect();
        Ok(format!("ea-{}", ea_id))
    } else {
        convert_to_ascii(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(make_username("").unwrap(), "");
    }

    #[test]
    fn ascii() {
        assert_eq!(make_username("JOHN SMITH").unwrap(), "johnsmith");
        assert_eq!(make_username("Petr Petráš").unwrap(), "petrpetras");
        assert_eq!(make_username("Auðunn Jónsson").unwrap(), "audunnjonsson");
    }

    #[test]
    fn japanese_name() {
        assert_eq!(
            make_username("武田 裕介").unwrap(),
            "ea-27494300003502920171"
        );
        assert_eq!(
            make_username("光紀 高橋").unwrap(),
            "ea-20809320003964027211"
        );
    }

    #[test]
    fn japanese_regression() {
        assert!(make_username("佐々木博之").is_ok());
        assert!(make_username("石川記みよ").is_ok());
        assert!(make_username("加藤 みどり").is_ok());
        assert!(make_username("澤山 あおい").is_ok());
        assert!(make_username("ラナ　ヘメンドラ　チャンドラ").is_ok());
        assert!(make_username("宮口 ｼｮｰﾝﾏｷ").is_ok());
        assert!(make_username("みぶ 真也").is_ok());
        assert!(make_username("松浦すぐる").is_ok());
    }

    #[test]
    fn disambig() {
        assert_eq!(make_username("John Smith #1").unwrap(), "johnsmith1");
        assert_eq!(make_username("Kevin Jäger #1").unwrap(), "kevinjager1");
    }

    #[test]
    fn exception() {
        assert_eq!(
            make_username("Brenda v.d. Meulen").unwrap(),
            "brendavdmeulen"
        );
        assert_eq!(
            make_username("Aliaksandr Hrynkevich-Sudnik").unwrap(),
            "aliaksandrhrynkevichsudnik"
        );
    }

    #[test]
    fn invalid_utf8() {
        assert!(make_username("John Smith❤ ").is_err());
    }

    #[test]
    fn invalid_ascii() {
        assert!(make_username("John Smith; ").is_err());
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
}

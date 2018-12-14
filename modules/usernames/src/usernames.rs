//! Implements Name to Username conversion logic.

/// Calculates the ASCII equivalent of a Name.
fn convert_to_ascii(name: &str) -> Result<String, String> {
    let mut ascii_name = String::with_capacity(name.len());

    // The to_lowercase call uses extra heap memory, but I haven't come up with a better way of doing this right now,
    // since lowercase letters can take up more space than uppercase ones.
    for letter in name.to_lowercase().chars() {
        if is_exception(letter) {
            continue;
        } else if letter.is_alphanumeric() && letter.is_ascii() {
            ascii_name.push(letter);
        } else {
            ascii_name.push_str(match letter {
                'á' | 'ä' | 'å' | 'ą' | 'ã' | 'à' | 'â' | 'ā' | 'ắ' | 'ấ' | 'ầ'
                | 'ặ' | 'ạ' | 'ă' | 'ả' | 'ậ' => "a",
                'æ' => "ae",
                'ć' | 'ç' | 'č' => "c",
                'đ' | 'ð' | 'ď' => "d",
                'é' | 'ê' | 'ë' | 'è' | 'ě' | 'ę' | 'ē' | 'ế' | 'ễ' | 'ể' | 'ề'
                | 'ệ' | 'ė' | 'ə' => "e",
                'ğ' | 'ģ' => "g",
                'î' | 'í' | 'ï' | 'ì' | 'ї' | 'ī' | 'ĩ' | 'ị' | 'ı' => "i",
                'ķ' => "k",
                'ľ' | 'ĺ' | 'ļ' | 'ŀ' => "l",
                'ñ' | 'ń' | 'ň' | 'ņ' => "n",
                'ø' | 'ô' | 'ö' | 'ó' | 'ő' | 'õ' | 'ò' | 'ỗ' | 'ọ' | 'ơ' | 'ồ'
                | 'ớ' | 'ố' => "o",
                'ř' => "r",
                'ß' => "ss",
                'š' | 'ś' | 'ș' | 'ş' => "s",
                'ț' | 'ť' => "t",
                'þ' => "th",
                'ü' | 'ů' | 'ú' | 'ù' | 'ū' | 'ű' | 'ư' | 'ứ' | 'ũ' | 'ữ' | 'ự'
                | 'ừ' => "u",
                'ý' | 'ỳ' | 'ỹ' => "y",
                'ł' => "w",
                'ž' | 'ż' | 'ź' => "z",
                _ => return Err(format!("Unknown char type {:?}", letter)),
            });
        }
    }
    Ok(ascii_name)
}

/// Whether the character should be silently omitted.
fn is_exception(letter: char) -> bool {
    match letter {
        ' ' | '\\' | '#' | '.' | '-' => true,
        _ => false,
    }
}

/// Checks if the given character is Chinese/Japanese/Korean.
fn is_eastasian(letter: char) -> bool {
    let ord: u32 = letter as u32;
    match ord {
        //CJK Compatibility
        13_056...13_311 => true,
        //CJK Unified Ideographs
        19_968...40_959 => true,
        //CJK Compatibility Forms
        65_072...65_103 => true,
        //CJK Compatibility Ideographs
        63_744...64_255 => true,
        //CJK Compatibility Ideographs Supplement
        194_560...195_103 => true,
        //Katakana
        12_448...12_543 => true,
        //CJK Radicals Supplement
        11_904...12_031 => true,
        //CJK Unified Ideographs Extension A
        13_312...19_903 => true,
        //CJK Unified Ideographs Extension B
        131_072...173_791 => true,
        //CJK Unified Ideographs Extension C
        173_824...177_983 => true,
        //CJK Unified Ideographs Extension D
        177_984...178_207 => true,
        //CJK Unified Ideographs Extension E
        178_208...183_983 => true,
        //Non East-Asian
        _ => false,
    }
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
        return Ok("".to_string());
    }

    if name.chars().all(|x| is_eastasian(x) || is_exception(x)) {
        let ea_id: String = name
            .chars()
            .map(|letter| (letter as u32).to_string())
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
    fn test_empty() {
        assert_eq!(make_username("").unwrap(), "");
    }

    #[test]
    fn test_ascii() {
        assert_eq!(make_username("JOHN SMITH").unwrap(), "johnsmith");
        assert_eq!(make_username("Petr Petráš").unwrap(), "petrpetras");
        assert_eq!(make_username("Auðunn Jónsson").unwrap(), "audunnjonsson");
    }

    #[test]
    fn test_eastasian() {
        assert_eq!(
            make_username("武田 裕介").unwrap(),
            "ea-2749430000323502920171"
        );
        assert_eq!(
            make_username("光紀 高橋").unwrap(),
            "ea-2080932000323964027211"
        );
    }

    #[test]
    fn test_disambig() {
        assert_eq!(make_username("John Smith #1").unwrap(), "johnsmith1");
        assert_eq!(make_username("Kevin Jäger #1").unwrap(), "kevinjager1");
    }

    #[test]
    fn test_exception() {
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
    fn test_invalid_utf8() {
        assert!(make_username("John Smith❤ ").is_err());
    }

    #[test]
    fn test_invalid_ascii() {
        assert!(make_username("John Smith; ").is_err());
    }
}

#![allow(unused)]


//calculates the ascii equivalent of a name
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
    return Ok(ascii_name);
}

//Allowed non-latin characters
fn is_exception(letter: char) -> bool {
    match letter {
        ' ' | '\\' | '#' | '.' | '-' => true,
        _ => false,
    }
}

//Check if character is Japanese/Chinese
fn is_eastasian(letter: char) -> bool {
    let ord: u32 = letter as u32;
    match ord {
        //CJK Compatibility
        13056...13311 => true,
        //CJK Unified Ideographs
        19968...40959 => true,
        //CJK Compatibility Forms
        65072...65103 => true,
        //CJK Compatibility Ideographs
        63744...64255 => true,
        //CJK Compatibility Ideographs Supplement
        194560...195103 => true,
        //Katakana
        12448...12543 => true,
        //CJK Radicals Supplement
        11904...12031 => true,
        //CJK Unified Ideographs Extension A
        13312...19903 => true,
        //CJK Unified Ideographs Extension B
        131072...173791 => true,
        //CJK Unified Ideographs Extension C
        173824...177983 => true,
        //CJK Unified Ideographs Extension D
        177984...178207 => true,
        //CJK Unified Ideographs Extension E
        178208...183983 => true,
        //Non East-Asian
        _ => false,
    }
}

pub fn make_username(name: &str) -> Result<String, String> {
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

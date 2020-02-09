//! Simple benchmarks to assess the speed of username generation.

#![feature(test)]

extern crate usernames;

mod benches {
    use super::*;

    extern crate test;
    use self::test::Bencher;

    /// Tests the all-ASCII fast path (50 ASCII characters).
    #[bench]
    fn make_username_ascii(b: &mut Bencher) {
        let ascii_name = "1234567890".repeat(5);
        b.iter(|| {
            usernames::make_username(&ascii_name).unwrap();
        });
    }

    /// Tests the UTF-8 path (50 UTF-8 characters).
    ///
    /// The string that's chosen is one where a single UTF-8 character
    /// expands into two ASCII characters, doubling the resultant string size.
    #[bench]
    fn make_username_utf8_expansion(b: &mut Bencher) {
        let utf8_name = "þ".repeat(50);
        b.iter(|| {
            usernames::make_username(&utf8_name).unwrap();
        });
    }

    /// Tests Japanese names written in Hiragana, which get normalized into Katakana.
    #[bench]
    fn make_username_hiragana(b: &mut Bencher) {
        let hiragana_name = "なべやかん".repeat(10);
        b.iter(|| {
            usernames::make_username(&hiragana_name).unwrap();
        });
    }
}

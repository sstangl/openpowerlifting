//! Implements writing system detection.

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

/// Get the WritingSystem for the current character.
///
/// Returns `Latin` if unknown.
pub fn writing_system(c: char) -> WritingSystem {
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
        .find_map(|c| match writing_system(c) {
            WritingSystem::Latin => None,
            other => Some(other),
        })
        .unwrap_or(WritingSystem::Latin)
}

use crate::constants::*;

pub const fn is_tamil_unicode(c: char) -> bool {
    matches!(c, '\u{0B82}'..='\u{0BFA}')
}

pub const fn is_combining_mark(c: char) -> bool {
    matches!(c, '்' | 'ா'..='ூ' | 'ெ'..='ை' | 'ொ'..='ௌ')
}
pub const fn is_uyir(c: char) -> bool {
    matches!(
        c,
        VOWEL_A
            | VOWEL_AA
            | VOWEL_I
            | VOWEL_II
            | VOWEL_U
            | VOWEL_UU
            | VOWEL_E
            | VOWEL_EE
            | VOWEL_AI
            | VOWEL_O
            | VOWEL_OO
            | VOWEL_AU
    )
}

pub fn has_tamil(word: &str) -> bool {
    word.chars().any(is_tamil_char)
}
pub const fn is_tamil_char(c: char) -> bool {
    matches!(
        c,
        // Standard Tamil block (letters, signs, digits 0-9, symbols 10, 100, 1000)
        '\u{0B80}'..='\u{0BFF}'
        // Optional: Tamil Supplement block (historical fractions & symbols)
        | '\u{11FC0}'..='\u{11FFF}'
    )
}

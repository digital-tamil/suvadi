use crate::letters::tamil_letters::*;

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

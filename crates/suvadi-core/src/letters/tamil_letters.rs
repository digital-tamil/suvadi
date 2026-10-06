pub const VOWEL_A: char = 'அ';
pub const VOWEL_AA: char = 'ஆ';
pub const VOWEL_I: char = 'இ';
pub const VOWEL_II: char = 'ஈ';
pub const VOWEL_U: char = 'உ';
pub const VOWEL_UU: char = 'ஊ';
pub const VOWEL_E: char = 'எ';
pub const VOWEL_EE: char = 'ஏ';
pub const VOWEL_AI: char = 'ஐ';
pub const VOWEL_O: char = 'ஒ';
pub const VOWEL_OO: char = 'ஓ';
pub const VOWEL_AU: char = 'ஔ';

pub const AYTHAM_LETTER: char = 'ஃ';
pub const PULLI: char = '்';

pub const UYIR: [char; 12] = [
    VOWEL_A, VOWEL_AA, VOWEL_I, VOWEL_II, VOWEL_U, VOWEL_UU, VOWEL_E, VOWEL_EE, VOWEL_AI, VOWEL_O,
    VOWEL_OO, VOWEL_AU,
];

pub const UYIR_LETTERS: &[char; 12] = &UYIR;

pub const KURIL_LETTERS: [char; 5] = [VOWEL_A, VOWEL_I, VOWEL_U, VOWEL_E, VOWEL_O];
pub const NEDIL_LETTERS: [char; 7] = [
    VOWEL_AA, VOWEL_II, VOWEL_UU, VOWEL_EE, VOWEL_AI, VOWEL_OO, VOWEL_AU,
];
pub const DIPTHONG_LETTERS: [char; 2] = [VOWEL_AI, VOWEL_AU];

pub const PRONOUN_LETTERS: [char; 3] = [VOWEL_A, VOWEL_I, VOWEL_U];
pub const SUTTEZHUTHTHU: &[char; 3] = &PRONOUN_LETTERS;

pub const QUESTIONSUFFIX_LETTERS: [char; 3] = [VOWEL_AA, VOWEL_EE, VOWEL_OO];
pub const VINAAEZHUTHTHU: &[char; 3] = &QUESTIONSUFFIX_LETTERS;

pub const VALLINAM_LETTERS: [&str; 6] = ["க்", "ச்", "ட்", "த்", "ப்", "ற்"];
pub const MELLINAM_LETTERS: [&str; 6] = ["ங்", "ஞ்", "ண்", "ந்", "ம்", "ன்"];
pub const IDAYINAM_LETTERS: [&str; 6] = ["ய்", "ர்", "ல்", "வ்", "ழ்", "ள்"];

pub const MEI_LETTERS: [&str; 18] = [
    "க்", "ச்", "ட்", "த்", "ப்", "ற்", "ஞ்", "ங்", "ண்", "ந்", "ம்", "ன்", "ய்", "ர்", "ல்", "வ்", "ழ்", "ள்",
];

pub const ACCENT_SYMBOLS: [char; 13] = [
    '\u{0}', 'ா', 'ி', 'ீ', 'ு', 'ூ', 'ெ', 'ே', 'ை', 'ொ', 'ோ', 'ௌ', 'ஃ',
];

pub const PULLI_SYMBOLS: [char; 1] = [PULLI];

pub const AGARAM_LETTERS: [char; 18] = [
    'க', 'ச', 'ட', 'த', 'ப', 'ற', 'ஞ', 'ங', 'ண', 'ந', 'ம', 'ன', 'ய', 'ர', 'ல', 'வ', 'ழ', 'ள',
];

pub const MAYANGOLI_LETTERS: [char; 8] = ['ண', 'ன', 'ந', 'ல', 'ழ', 'ள', 'ர', 'ற'];

pub const SANSKRIT_LETTERS: [&str; 6] = ["ஶ", "ஜ", "ஷ", "ஸ", "ஹ", "க்ஷ"];
pub const SANSKRIT_MEI_LETTERS: [&str; 6] = ["ஶ்", "ஜ்", "ஷ்", "ஸ்", "ஹ்", "க்ஷ்"];

pub const GRANTHA_MEI_LETTERS: [&str; 24] = [
    "க்",
    "ச்",
    "ட்",
    "த்",
    "ப்",
    "ற்",
    "ஞ்",
    "ங்",
    "ண்",
    "ந்",
    "ம்",
    "ன்",
    "ய்",
    "ர்",
    "ல்",
    "வ்",
    "ழ்",
    "ள்",
    "ஶ்",
    "ஜ்",
    "ஷ்",
    "ஸ்",
    "ஹ்",
    "க்ஷ்",
];

pub const UYIRMEI_LETTERS: [&str; 216] = [
    "க", "கா", "கி", "கீ", "கு", "கூ", "கெ", "கே", "கை", "கொ", "கோ", "கௌ", "ச", "சா", "சி", "சீ", "சு",
    "சூ", "செ", "சே", "சை", "சொ", "சோ", "சௌ", "ட", "டா", "டி", "டீ", "டு", "டூ", "டெ", "டே", "டை",
    "டொ", "டோ", "டௌ", "த", "தா", "தி", "தீ", "து", "தூ", "தெ", "தே", "தை", "தொ", "தோ", "தௌ", "ப",
    "பா", "பி", "பீ", "பு", "பூ", "பெ", "பே", "பை", "பொ", "போ", "பௌ", "ற", "றா", "றி", "றீ", "று",
    "றூ", "றெ", "றே", "றை", "றொ", "றோ", "றௌ", "ஞ", "ஞா", "ஞி", "ஞீ", "ஞு", "ஞூ", "ஞெ", "ஞே", "ஞை",
    "ஞொ", "ஞோ", "ஞௌ", "ங", "ஙா", "ஙி", "ஙீ", "ஙு", "ஙூ", "ஙெ", "ஙே", "ஙை", "ஙொ", "ஙோ", "ஙௌ", "ண",
    "ணா", "ணி", "ணீ", "ணு", "ணூ", "ணெ", "ணே", "ணை", "ணொ", "ணோ", "ணௌ", "ந", "நா", "நி", "நீ", "நு",
    "நூ", "நெ", "நே", "நை", "நொ", "நோ", "நௌ", "ம", "மா", "மி", "மீ", "மு", "மூ", "மெ", "மே", "மை",
    "மொ", "மோ", "மௌ", "ன", "னா", "னி", "னீ", "னு", "னூ", "னெ", "னே", "னை", "னொ", "னோ", "னௌ", "ய",
    "யா", "யி", "யீ", "யு", "யூ", "யெ", "யே", "யை", "யொ", "யோ", "யௌ", "ர", "ரா", "ரி", "ரீ", "ரு",
    "ரூ", "ரெ", "ரே", "ரை", "ரொ", "ரோ", "ரௌ", "ல", "லா", "லி", "லீ", "லு", "லூ", "லெ", "லே", "லை",
    "லொ", "லோ", "லௌ", "வ", "வா", "வி", "வீ", "வு", "வூ", "வெ", "வே", "வை", "வொ", "வோ", "வௌ", "ழ",
    "ழா", "ழி", "ழீ", "ழு", "ழூ", "ழெ", "ழே", "ழை", "ழொ", "ழோ", "ழௌ", "ள", "ளா", "ளி", "ளீ", "ளு",
    "ளூ", "ளெ", "ளே", "ளை", "ளொ", "ளோ", "ளௌ",
];

const COMBINING_SIGNS: [char; 12] = ['்', 'ா', 'ி', 'ீ', 'ு', 'ூ', 'ெ', 'ே', 'ை', 'ொ', 'ோ', 'ௌ'];

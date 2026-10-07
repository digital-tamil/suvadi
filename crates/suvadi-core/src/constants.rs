/// Tamil Om / Pranava Symbol (ௐ - Om) - U+0BD0
pub const OHM: char = 'ௐ';

/// Tamil Letter A (அ - Short Vowel / குறில்) - U+0B85
pub const VOWEL_A: char = 'அ';
/// Tamil Letter Aa (ஆ - Long Vowel / நெடில்) - U+0B86
pub const VOWEL_AA: char = 'ஆ';
/// Tamil Letter I (இ - Short Vowel / குறில்) - U+0B87
pub const VOWEL_I: char = 'இ';
/// Tamil Letter Ii (ஈ - Long Vowel / நெடில்) - U+0B88
pub const VOWEL_II: char = 'ஈ';
/// Tamil Letter U (உ - Short Vowel / குறில்) - U+0B89
pub const VOWEL_U: char = 'உ';
/// Tamil Letter Uu (ஊ - Long Vowel / நெடில்) - U+0B8A
pub const VOWEL_UU: char = 'ஊ';
/// Tamil Letter E (எ - Short Vowel / குறில்) - U+0B8E
pub const VOWEL_E: char = 'எ';
/// Tamil Letter Ee (ஏ - Long Vowel / நெடில்) - U+0B8F
pub const VOWEL_EE: char = 'ஏ';
/// Tamil Letter Ai (ஐ - Diphthong / நெடில்) - U+0B90
pub const VOWEL_AI: char = 'ஐ';
/// Tamil Letter O (ஒ - Short Vowel / குறில்) - U+0B92
pub const VOWEL_O: char = 'ஒ';
/// Tamil Letter Oo (ஓ - Long Vowel / நெடில்) - U+0B93
pub const VOWEL_OO: char = 'ஓ';
/// Tamil Letter Au (ஔ - Diphthong / நெடில்) - U+0B94
pub const VOWEL_AU: char = 'ஔ';

/// Tamil Ayudha Letter / Visarga (ஃ - ஆய்த எழுத்து / Āyutha Ezhuthu) - U+0B83
pub const AYUDHAM_LETTER: char = 'ஃ';
/// Tamil Sign Virama / Pulli (் - புள்ளி / Puḷḷi) - U+0BCD
/// Combining virama sign placed above consonants to mute the inherent vowel, forming pure consonants (மெய் எழுத்து).
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

pub const COMBINING_SIGNS: [char; 12] = ['்', 'ா', 'ி', 'ீ', 'ு', 'ூ', 'ெ', 'ே', 'ை', 'ொ', 'ோ', 'ௌ'];

pub const UYIRMEI_OFFSET: u8 = 12 + 1 + 18 + 2;
pub const DAY: &str = "௳";
pub const MONTH: &str = "௴";
pub const YEAR: &str = "௵";
pub const DEBIT: &str = "௶";
pub const CREDIT: &str = "௷";
pub const RUPEE: &str = "௹";
pub const NUMERAL: &str = "௺";
pub const SRI: &str = "\u{0bb6}\u{0bcd}\u{0bb0}\u{0bc0}"; // #SRI -ஶ்ரீ
pub const KSHA: &str = "\u{0b95}\u{0bcd}\u{0bb7}"; // #KSHA - க்ஷ
pub const KSH: &str = "\u{0b95}\u{0bcd}\u{0bb7}\u{0bcd}"; // #KSH - க்ஷ்
pub const INDIAN_RUPEE: &str = "₹";
pub const TAMIL_SYMBOLS: [&str; 11] = [
    DAY,
    MONTH,
    YEAR,
    DEBIT,
    CREDIT,
    RUPEE,
    NUMERAL,
    SRI,
    KSHA,
    KSH,
    INDIAN_RUPEE,
];

const fn generate_tamil247() -> [&'static str; 247] {
    let mut table = [""; 247];

    table[0] = "ஃ";

    let mut i = 0;
    const UYIR_LETTERS_STR: [&str; 12] =
        ["அ", "ஆ", "இ", "ஈ", "உ", "ஊ", "எ", "ஏ", "ஐ", "ஒ", "ஓ", "ஔ"];
    while i < UYIR_LETTERS_STR.len() {
        table[1 + i] = UYIR_LETTERS_STR[i];
        i += 1;
    }

    let mut j = 0;
    while j < MEI_LETTERS.len() {
        table[1 + UYIR_LETTERS_STR.len() + j] = MEI_LETTERS[j];
        j += 1;
    }

    let mut k = 0;
    while k < UYIRMEI_LETTERS.len() {
        table[1 + UYIR_LETTERS_STR.len() + MEI_LETTERS.len() + k] = UYIRMEI_LETTERS[k];
        k += 1;
    }

    table
}

/// Pure Tamil Letters
pub const TAMIL247_LETTERS: [&str; 247] = generate_tamil247();

/// Tamil Letters with sanskrit influence
pub const TAMIL_LETTERS: [&str; 345] = [
    /* Uyir */
    "அ",
    "ஆ",
    "இ",
    "ஈ",
    "உ",
    "ஊ",
    "எ",
    "ஏ",
    "ஐ",
    "ஒ",
    "ஓ",
    "ஔ",
    /* Ayuda Ezhuthu */
    "ஃ",
    /* Mei */
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
    /* Sanskrit (Mei) */
    "ஜ்",
    "ஷ்",
    "ஸ்",
    "ஹ்",
    /* Agaram */
    "க",
    "ச",
    "ட",
    "த",
    "ப",
    "ற",
    "ஞ",
    "ங",
    "ண",
    "ந",
    "ம",
    "ன",
    "ய",
    "ர",
    "ல",
    "வ",
    "ழ",
    "ள",
    /* Sanskrit (Vada Mozhi) */
    "ஜ",
    "ஷ",
    "ஸ",
    "ஹ",
    /* Uyir Mei */
    "க",
    "கா",
    "கி",
    "கீ",
    "கு",
    "கூ",
    "கெ",
    "கே",
    "கை",
    "கொ",
    "கோ",
    "கௌ",
    "ச",
    "சா",
    "சி",
    "சீ",
    "சு",
    "சூ",
    "செ",
    "சே",
    "சை",
    "சொ",
    "சோ",
    "சௌ",
    "ட",
    "டா",
    "டி",
    "டீ",
    "டு",
    "டூ",
    "டெ",
    "டே",
    "டை",
    "டொ",
    "டோ",
    "டௌ",
    "த",
    "தா",
    "தி",
    "தீ",
    "து",
    "தூ",
    "தெ",
    "தே",
    "தை",
    "தொ",
    "தோ",
    "தௌ",
    "ப",
    "பா",
    "பி",
    "பீ",
    "பு",
    "பூ",
    "பெ",
    "பே",
    "பை",
    "பொ",
    "போ",
    "பௌ",
    "ற",
    "றா",
    "றி",
    "றீ",
    "று",
    "றூ",
    "றெ",
    "றே",
    "றை",
    "றொ",
    "றோ",
    "றௌ",
    "ஞ",
    "ஞா",
    "ஞி",
    "ஞீ",
    "ஞு",
    "ஞூ",
    "ஞெ",
    "ஞே",
    "ஞை",
    "ஞொ",
    "ஞோ",
    "ஞௌ",
    "ங",
    "ஙா",
    "ஙி",
    "ஙீ",
    "ஙு",
    "ஙூ",
    "ஙெ",
    "ஙே",
    "ஙை",
    "ஙொ",
    "ஙோ",
    "ஙௌ",
    "ண",
    "ணா",
    "ணி",
    "ணீ",
    "ணு",
    "ணூ",
    "ணெ",
    "ணே",
    "ணை",
    "ணொ",
    "ணோ",
    "ணௌ",
    "ந",
    "நா",
    "நி",
    "நீ",
    "நு",
    "நூ",
    "நெ",
    "நே",
    "நை",
    "நொ",
    "நோ",
    "நௌ",
    "ம",
    "மா",
    "மி",
    "மீ",
    "மு",
    "மூ",
    "மெ",
    "மே",
    "மை",
    "மொ",
    "மோ",
    "மௌ",
    "ன",
    "னா",
    "னி",
    "னீ",
    "னு",
    "னூ",
    "னெ",
    "னே",
    "னை",
    "னொ",
    "னோ",
    "னௌ",
    "ய",
    "யா",
    "யி",
    "யீ",
    "யு",
    "யூ",
    "யெ",
    "யே",
    "யை",
    "யொ",
    "யோ",
    "யௌ",
    "ர",
    "ரா",
    "ரி",
    "ரீ",
    "ரு",
    "ரூ",
    "ரெ",
    "ரே",
    "ரை",
    "ரொ",
    "ரோ",
    "ரௌ",
    "ல",
    "லா",
    "லி",
    "லீ",
    "லு",
    "லூ",
    "லெ",
    "லே",
    "லை",
    "லொ",
    "லோ",
    "லௌ",
    "வ",
    "வா",
    "வி",
    "வீ",
    "வு",
    "வூ",
    "வெ",
    "வே",
    "வை",
    "வொ",
    "வோ",
    "வௌ",
    "ழ",
    "ழா",
    "ழி",
    "ழீ",
    "ழு",
    "ழூ",
    "ழெ",
    "ழே",
    "ழை",
    "ழொ",
    "ழோ",
    "ழௌ",
    "ள",
    "ளா",
    "ளி",
    "ளீ",
    "ளு",
    "ளூ",
    "ளெ",
    "ளே",
    "ளை",
    "ளொ",
    "ளோ",
    "ளௌ", /* Sanskrit Uyir-Mei */
    "ஶ",
    "ஶா",
    "ஶி",
    "ஶீ",
    "ஶு",
    "ஶூ",
    "ஶெ",
    "ஶே",
    "ஶை",
    "ஶொ",
    "ஶோ",
    "ஶௌ",
    "ஜ",
    "ஜா",
    "ஜி",
    "ஜீ",
    "ஜு",
    "ஜூ",
    "ஜெ",
    "ஜே",
    "ஜை",
    "ஜொ",
    "ஜோ",
    "ஜௌ",
    "ஷ",
    "ஷா",
    "ஷி",
    "ஷீ",
    "ஷு",
    "ஷூ",
    "ஷெ",
    "ஷே",
    "ஷை",
    "ஷொ",
    "ஷோ",
    "ஷௌ",
    "ஸ",
    "ஸா",
    "ஸி",
    "ஸீ",
    "ஸு",
    "ஸூ",
    "ஸெ",
    "ஸே",
    "ஸை",
    "ஸொ",
    "ஸோ",
    "ஸௌ",
    "ஹ",
    "ஹா",
    "ஹி",
    "ஹீ",
    "ஹு",
    "ஹூ",
    "ஹெ",
    "ஹே",
    "ஹை",
    "ஹொ",
    "ஹோ",
    "ஹௌ",
    "க்ஷ",
    "க்ஷா",
    "க்ஷி",
    "க்ஷீ",
    "க்ஷு",
    "க்ஷூ",
    "க்ஷெ",
    "க்ஷே",
    "க்ஷை",
    "க்ஷொ",
    "க்ஷோ",
    "க்ஷௌ",
];

// NUMBERS
/// Tamil Digit Zero (பூஜ்யம் / சுழி) - U+0BE6
pub const TAMIL_ZERO: char = '௦'; // '\u{0BE6}'

/// Tamil Digit One (ஒன்று - Ontru) - U+0BE7
pub const TAMIL_ONE: char = '௧'; // '\u{0BE7}'

/// Tamil Digit Two (இரண்டு - Irandu) - U+0BE8
pub const TAMIL_TWO: char = '௨'; // '\u{0BE8}'

/// Tamil Digit Three (மூன்று - Moontru) - U+0BE9
pub const TAMIL_THREE: char = '௩'; // '\u{0BE9}'

/// Tamil Digit Four (நான்கு - Naanku) - U+0BEA
pub const TAMIL_FOUR: char = '௪'; // '\u{0BEA}'

/// Tamil Digit Five (ஐந்து - Ainthu) - U+0BEB
pub const TAMIL_FIVE: char = '௫'; // '\u{0BEB}'

/// Tamil Digit Six (ஆறு - Aaru) - U+0BEC
pub const TAMIL_SIX: char = '௬'; // '\u{0BEC}'

/// Tamil Digit Seven (ஏழு - Ezu) - U+0BED
pub const TAMIL_SEVEN: char = '௭'; // '\u{0BED}'

/// Tamil Digit Eight (எட்டு - Ettu) - U+0BEE
pub const TAMIL_EIGHT: char = '௮'; // '\u{0BEE}'

/// Tamil Digit Nine (ஒன்பது - Onpathu) - U+0BEF
pub const TAMIL_NINE: char = '௯'; // '\u{0BEF}'

/// Tamil Number Ten (பத்து - Pathu) - U+0BF0
/// Note: In Unicode, this is a distinct numeral glyph (U+0BF0), not the positional pair '௧௦'.
pub const TAMIL_TEN: char = '௰';

/// Tamil Fraction One Three-Hundred-and-Twentieth (முந்திரி - Muntiri)
/// Value: 1/320 (0.003125) - U+11FC0
pub const TAMIL_FRACTION_MUNTIRI: char = '𑿀'; // '\u{11FC0}'

/// Tamil Fraction One One-Hundred-and-Sixtieth (அரைக்காணி - Araikkaṇi)
/// Value: 2/320 = 1/160 (0.00625) - U+11FC1
pub const TAMIL_FRACTION_ARAIKKANI: char = '𑿁'; // '\u{11FC1}'

/// Tamil Fraction One Eightieth (காணி - Kāṇi)
/// Value: 4/320 = 1/80 (0.0125) - U+11FC2
pub const TAMIL_FRACTION_KANI: char = '𑿂'; // '\u{11FC2}'

/// Tamil Fraction One Fortieth (அரைய்மா - Araimā)
/// Value: 8/320 = 1/40 (0.025) - U+11FC4
pub const TAMIL_FRACTION_ARAIMA: char = '𑿄'; // '\u{11FC4}'

/// Tamil Fraction Three Eightieths (முக்காணி - Mukkaṇi)
/// Value: 12/320 = 3/80 (0.0375) - U+11FC6
pub const TAMIL_FRACTION_MUKKANI: char = '𑿆'; // '\u{11FC6}'

/// Tamil Fraction One Twentieth (மா - Mā)
/// Value: 16/320 = 1/20 (0.05) - U+11FC8
pub const TAMIL_FRACTION_MA: char = '𑿈'; // '\u{11FC8}'

/// Tamil Fraction One Tenth (இருமா - Irumā)
/// Value: 32/320 = 1/10 (0.1) - U+11FCB
pub const TAMIL_FRACTION_IRUMA: char = '𑿋'; // '\u{11FCB}'

/// Tamil Fraction Three Twentieths (மும்மா - Mummā)
/// Value: 48/320 = 3/20 (0.15) - U+11FCD
pub const TAMIL_FRACTION_MUMMA: char = '𑿍'; // '\u{11FCD}'

/// Tamil Fraction One Fifth (நான்குமா - Nāṅkumā)
/// Value: 64/320 = 1/5 (0.2) - U+11FCF
pub const TAMIL_FRACTION_NANKUMA: char = '𑿏'; // '\u{11FCF}'

/// Tamil Fraction One Sixty-Fourth (கால் விசம் - Kālvicam)
/// Value: 5/320 = 1/64 (0.015625) - U+11FC3
pub const TAMIL_FRACTION_KAL_VICAM: char = '𑿃'; // '\u{11FC3}'

/// Tamil Fraction One Thirty-Second (அரை விசம் - Araivicam)
/// Value: 10/320 = 1/32 (0.03125) - U+11FC5
/// Note: Historically often printed using the two-letter ligature "ழூ" ("\u{0BB4}\u{0BC2}").
pub const TAMIL_FRACTION_ARAI_VICAM: char = '𑿅'; // '\u{11FC5}'
pub const TAMIL_FRACTION_ARAI_VICAM_LIGATURE: &str = "ழூ"; // "\u{0BB4}\u{0BC2}"

/// Tamil Fraction Three Sixty-Fourths (முக்கால் விசம் - Mukkālvicam)
/// Value: 15/320 = 3/64 (0.046875) - U+11FC7
pub const TAMIL_FRACTION_MUKKAL_VICAM: char = '𑿇'; // '\u{11FC7}'

/// Tamil Fraction One Sixteenth - Form 1 (விசம் / மாகாணி - Vicam / Mākāṇi)
/// Value: 20/320 = 1/16 (0.0625) - U+11FC9
pub const TAMIL_FRACTION_VICAM_1: char = '𑿉'; // '\u{11FC9}'

/// Tamil Fraction One Sixteenth - Form 2 (விசம் / மாகாணி - Vicam / Mākāṇi)
/// Value: 20/320 = 1/16 (0.0625) - U+11FCA
pub const TAMIL_FRACTION_VICAM_2: char = '𑿊'; // '\u{11FCA}'

/// Tamil Fraction One Eighth (அரை கால் - Araikkāl)
/// Value: 40/320 = 1/8 (0.125) - U+11FCC
pub const TAMIL_FRACTION_ARAI_KAL: char = '𑿌'; // '\u{11FCC}'

/// Tamil Fraction Three Sixteenths (மூ விசம் / மும்மா முக்காணி - Mūvicam / Mummāmukkaṇi)
/// Value: 60/320 = 3/16 (0.1875) - U+11FCE
pub const TAMIL_FRACTION_MU_VICAM: char = '𑿎'; // '\u{11FCE}'

/// Tamil Fraction One Quarter (கால் - Kāl)
/// Value: 80/320 = 1/4 (0.25) - U+11FD0
pub const TAMIL_FRACTION_KAL: char = '𑿐'; // '\u{11FD0}'

/// Tamil Fraction One Half - Form 1 (அரை - Arai)
/// Value: 160/320 = 1/2 (0.5) - U+11FD1
pub const TAMIL_FRACTION_ARAI_1: char = '𑿑'; // '\u{11FD1}'

/// Tamil Fraction One Half - Form 2 (அரை - Arai)
/// Value: 160/320 = 1/2 (0.5) - U+11FD2
pub const TAMIL_FRACTION_ARAI_2: char = '𑿒'; // '\u{11FD2}'

/// Tamil Fraction Three Quarters (முக்கால் - Mukkāl)
/// Value: 240/320 = 3/4 (0.75) - U+11FD3
pub const TAMIL_FRACTION_MUKKAL: char = '𑿓'; // '\u{11FD3}'

/// Tamil Fraction Downscaling Factor Kiizh (கில் / கீழ் - Kiḷ)
/// Value: Multiplier factor ×1/320 (×0.003125) - U+11FD4
pub const TAMIL_FRACTION_KIL: char = '𑿔'; // '\u{11FD4}'

/// Tamil Number One Hundred (நூறு - Nūṟu) - 10² (100) - U+0BF1
pub const TAMIL_HUNDRED: char = '௱'; // '\u{0BF1}'
pub const TAMIL_NURU: char = TAMIL_HUNDRED;

/// Tamil Number One Thousand (ஆயிரம் - Āyiram) - 10³ (1,000) - U+0BF2
pub const TAMIL_THOUSAND: char = '௲'; // '\u{0BF2}'
pub const TAMIL_AYIRAM: char = TAMIL_THOUSAND;

/// Tamil Number Ten Thousand (பத்தாயிரம் - Pattāyiram) - 10⁴ (10,000)
/// Formed by Ten (௰) + Thousand (௲)
pub const TAMIL_TEN_THOUSAND: &str = "௰௲"; // "\u{0BF0}\u{0BF2}"
pub const TAMIL_PATTAYIRAM: &str = TAMIL_TEN_THOUSAND;

/// Tamil Number Hundred Thousand (நூறாயிரம் - Nūṟāyiram) - 10⁵ (100,000)
/// Formed by Hundred (௱) + Thousand (௲)
pub const TAMIL_HUNDRED_THOUSAND: &str = "௱௲"; // "\u{0BF1}\u{0BF2}"
pub const TAMIL_NURAYIRAM: &str = TAMIL_HUNDRED_THOUSAND;

/// Tamil Number Million (மெய்யிரம் - Meyyiram) - 10⁶ (1,000,000)
/// Formed by Thousand × Thousand (௲௲)
pub const TAMIL_MILLION: &str = "௲௲"; // "\u{0BF2}\u{0BF2}"
pub const TAMIL_MEYYIRAM: &str = TAMIL_MILLION;

/// Tamil Number Billion / Milliard (தொள்ளுண் - Toḷḷuṇ) - 10⁹ (1,000,000,000)
pub const TAMIL_BILLION: &str = "௲௲௲"; // "\u{0BF2}\u{0BF2}\u{0BF2}"
pub const TAMIL_TOLLUN: &str = TAMIL_BILLION;

/// Tamil Number Trillion / Billion (ஈகியம் - Īkiyam) - 10¹² (1,000,000,000,000)
pub const TAMIL_TRILLION: &str = "௲௲௲௲"; // "\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}"
pub const TAMIL_IKIYAM: &str = TAMIL_TRILLION;

/// Tamil Number Quadrillion / Billiard (நெளை - Neḷai) - 10¹⁵ (1,000,000,000,000,000)
pub const TAMIL_QUADRILLION: &str = "௲௲௲௲௲"; // "\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}"
pub const TAMIL_NELAI: &str = TAMIL_QUADRILLION;

/// Tamil Number Quintillion / Trillion (இளஞ்சி - Iḷañci) - 10¹⁸ (1,000,000,000,000,000,000)
pub const TAMIL_QUINTILLION: &str = "௲௲௲௲௲௲"; // "\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}"
pub const TAMIL_ILANCI: &str = TAMIL_QUINTILLION;

/// Tamil Number Hundred Quintillion (வெள்ளம் - Veḷḷam) - 10²⁰ (100,000,000,000,000,000,000)
/// Formed by Hundred (௱) followed by 6 Thousand glyphs (௲)
pub const TAMIL_HUNDRED_QUINTILLION: &str = "௱௲௲௲௲௲௲"; // "\u{0BF1}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}"
pub const TAMIL_VELLAM: &str = TAMIL_HUNDRED_QUINTILLION;

/// Tamil Number Sextillion / Trilliard (ஆம்பல் - Āmbal) - 10²¹ (1,000,000,000,000,000,000,000)
/// Formed by 7 Thousand glyphs (௲)
pub const TAMIL_SEXTILLION: &str = "௲௲௲௲௲௲௲"; // "\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}\u{0BF2}"
pub const TAMIL_AMBAL: &str = TAMIL_SEXTILLION;

pub mod tamil_rank_values {
    pub const VALUE_PATTU: u128 = 10;
    pub const VALUE_NURU: u128 = 100;
    pub const VALUE_AYIRAM: u128 = 1_000;
    pub const VALUE_PATTAYIRAM: u128 = 10_000;
    pub const VALUE_NURAYIRAM: u128 = 100_000;
    pub const VALUE_MEYYIRAM: u128 = 1_000_000;
    pub const VALUE_TOLLUN: u128 = 1_000_000_000;
    pub const VALUE_IKIYAM: u128 = 1_000_000_000_000;
    pub const VALUE_NELAI: u128 = 1_000_000_000_000_000;
    pub const VALUE_ILANCI: u128 = 1_000_000_000_000_000_000;
    pub const VALUE_VELLAM: u128 = 100_000_000_000_000_000_000;
    pub const VALUE_AMBAL: u128 = 1_000_000_000_000_000_000_000;
}

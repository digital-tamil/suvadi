use crate::constants::{
    TAMIL_EIGHT, TAMIL_FIVE, TAMIL_FOUR, TAMIL_NINE, TAMIL_ONE, TAMIL_SEVEN, TAMIL_SIX, TAMIL_TEN,
    TAMIL_THREE, TAMIL_TWO, TAMIL_ZERO,
};

pub struct Number {
    tamil_name: char,
    value: usize,
}
pub const tamil_numbers: [char; 11] = [
    TAMIL_ZERO,
    TAMIL_ONE,
    TAMIL_TWO,
    TAMIL_THREE,
    TAMIL_FOUR,
    TAMIL_FIVE,
    TAMIL_SIX,
    TAMIL_SEVEN,
    TAMIL_EIGHT,
    TAMIL_NINE,
    TAMIL_TEN,
];

const Tamil_Numbers_Lists: [Number; 1] = [Number {
    tamil_name: TAMIL_ZERO,
    value: 0,
}];
fn parse_tamil_number(tamil_num: String) -> isize {
    return 2;
}

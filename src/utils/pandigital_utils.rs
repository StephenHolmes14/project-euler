#[path = "string_utils.rs"] mod string_utils;

use string_utils::sort;

const ORDERED_PANDIGITAL: &str = "123456789";
const ORDERED_ZERO_PANDIGITAL: &str = "0123456789";

pub fn is_pandigital(number_string: &String) -> bool {
    sort(number_string).eq(&ORDERED_PANDIGITAL[0..number_string.len()])
}

pub fn is_pandigital_from_zero(number_string: &String) -> bool {
    sort(number_string).eq(&ORDERED_ZERO_PANDIGITAL[0..number_string.len()])
}

use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/string_utils.rs"] mod string_utils;

use string_utils::sort;

///
/// Take the number 192 and multiply it by each of 1, 2, and 3:
//
// 192 × 1 = 192
// 192 × 2 = 384
// 192 × 3 = 576
// By concatenating each product we get the 1 to 9 pandigital, 192384576. We will call 192384576 the concatenated product of 192 and (1,2,3)
//
// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5, giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
//
// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the concatenated product of an integer with (1,2, ... , n) where n > 1?
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 38: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    let ordered_number_string = String::from("123456789");
    let mut largest = 0;

    let mut current_value = 0;

    // Trying to find a good max value to check to
    // digits(1 * value) + digits(2 * value) = 9 digits max
    // Can't guarantee the number of digits of 2 * value, but it will be more than 1 * value
    // therefore
    // digits(1 * value) + digits(1 + value) < 9
    // 2 * digits(value) < 9
    // digits(value) < 4.5
    // digits(value) < 5
    // Therefore max value 99999
    while current_value < 100000 {
        current_value += 1;

        let mut current_multiplier = 0;
        let mut current_concat_string = String::new();
        while current_concat_string.len() < 9 {
            current_multiplier += 1;
            let nth_product = current_multiplier * current_value;
            current_concat_string += &nth_product.to_string();
        }

        if current_concat_string.len() == 9 && current_multiplier > 1 {
            let sorted_string = sort(&current_concat_string);
            if sorted_string.eq(&ordered_number_string)  {
                let current_concat_i32 = i32::from_str_radix(&current_concat_string, 10).unwrap();

                if current_concat_i32 > largest {
                    largest = current_concat_i32;
                }
            }
        }
    }

    largest
}


use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/binary_utils.rs"] mod binary_utils;
#[path = "../../utils/palindrome_utils.rs"] mod palindrome_utils;

use binary_utils::i32_to_binary_string;
use palindrome_utils::is_palindrome_i32;
use palindrome_utils::is_palindrome_string;

///
/// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
//
// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
//
// (Please note that the palindromic number, in either base, may not include leading zeros.)
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 36: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    let mut sum = 0;

    for x in 1..1000000 {

        // If it isn't a palindrome continue to next value
        if !is_palindrome_i32(x) {
            continue;
        }

        // It it reaches here it must be a palindrome in base 10
        // Convert to binary string then do another comparison
        let x_binary_string = i32_to_binary_string(x);

        if is_palindrome_string(&x_binary_string) {
            sum += x;
        }
    }

    sum
}


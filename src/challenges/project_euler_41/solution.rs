use std::ops::Add;
use std::time::Instant;

#[path = "../../utils/string_utils.rs"] mod string_utils;
#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use string_utils::sort;
use prime_utils::is_prime_i32;

///
/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
//
// What is the largest n-digit pandigital prime that exists?
///

const ORDERED_PANDIGITAL: &str = "123456789";

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 40: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    pandigital_search(String::new(), String::from("123456789"))
}

fn pandigital_search(current_value: String, remaining_digits: String) -> i32 {
    let mut best_solution = 0;

    if current_value != "" && is_prime_pandigital(&current_value) {
        best_solution = i32::from_str_radix(&current_value, 10).unwrap();
    }

    if remaining_digits.len() == 0 {
        return best_solution;
    }

    let largest_found = remaining_digits.chars()
        .into_iter()
        .map(|d| pandigital_search(String::from(d) + &current_value, remaining_digits.replace(d, "")))
        .max()
        .unwrap();

    if largest_found > best_solution {
        best_solution = largest_found;
    }

    best_solution
}

fn is_prime_pandigital(number_string: &String) -> bool {
    is_pandigital(number_string) && is_prime_i32(i32::from_str_radix(number_string, 10).unwrap())
}

fn is_pandigital(number_string: &String) -> bool {
    sort(number_string).eq(&ORDERED_PANDIGITAL[0..number_string.len()])
}


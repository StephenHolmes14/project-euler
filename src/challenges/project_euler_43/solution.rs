use std::ops::Add;
use std::time::Instant;
use std::fs;

#[path = "../../utils/pandigital_utils.rs"] mod pandigital_utils;

use pandigital_utils::is_pandigital_from_zero;

///
/// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
//
// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
//
// d2d3d4=406 is divisible by 2
// d3d4d5=063 is divisible by 3
// d4d5d6=635 is divisible by 5
// d5d6d7=357 is divisible by 7
// d6d7d8=572 is divisible by 11
// d7d8d9=728 is divisible by 13
// d8d9d10=289 is divisible by 17
// Find the sum of all 0 to 9 pandigital numbers with this property.
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 43: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i64 {
    // New plan - dynamic programming to precalculate the values
    // E.g. for the last property, find all numbers between 100 and 999 which are divisible by 17
    // Start from the largest numbers and work down to the lower ones (Should be faster this way I think)
    let divisible_numbers: Vec<Vec<String>> = DIVISORS.iter()
        .map(|x| (0..1000).into_iter()
                        .filter(|y| y % x == 0)
                        .map(|y| format!("{:0>3}", y))
                        .collect())
        .rev()
        .collect();

    let weird_numbers = get_numbers_with_weird_property(&String::from(""), &divisible_numbers);


    weird_numbers.iter()
        .flat_map(|n| (0..10).into_iter().map(|y| y.to_string() + n))
        .filter(|n| is_pandigital_from_zero(n))
        .map(|n| i64::from_str_radix(&n, 10).unwrap())
        .sum()
}

const DIVISORS: [i32; 7] = [2,3,5,7,11,13,17];


fn get_numbers_with_weird_property(current: &String, divisible_numbers: &Vec<Vec<String>>) -> Vec<String> {
    let remaining: Vec<Vec<String>> = divisible_numbers.clone().into_iter().skip(1).collect();

    // Base case
    if current.len() == 0 {
        return divisible_numbers.get(0)
            .unwrap()
            .iter()
            .flat_map(|d| get_numbers_with_weird_property(&d.to_string(), &remaining))
            .collect();
    }

    // Otherwise I need to check if the number continues correctly
    // We're starting from the back, so xxxxxxxx789
    // The next one to check is xxxxxxx678x
    // So the first 2 characters of the current value must be the last 2 characters of the next one
    let chars_to_find: String = get_leading_chars(current, 2);

    divisible_numbers.get(0)
        .unwrap()
        .iter()
        .map(|d| d.to_string())
        .filter(|d| get_trailing_chars(&d, 2).eq(&chars_to_find))
        .map(|d| get_leading_chars(&d, 1) + current)
        .flat_map(|d| if remaining.is_empty() { Vec::from([d]) } else { get_numbers_with_weird_property(&d, &remaining) })
        .collect()
}

fn get_leading_chars(value: &String, n: usize) -> String {
    value.chars().into_iter().take(n).collect()
}

fn get_trailing_chars(value: &String, n: usize) -> String {
    value.chars().into_iter().skip(value.len() - n).take(n).collect()
}

// This attempt to brute force is just way too slow
// fn has_weird_property(number_string: &String) -> bool {
//     for i in 1..9 {
//         let number_substring: String = number_string.chars()
//             .skip(i)
//             .take(3)
//             .collect();
//         let number_sub = i32::from_str_radix(&number_substring, 10).unwrap();
//
//         let divisor = DIVISORS.get(i - 1).unwrap();
//         if number_sub % divisor != 0 {
//             return false;
//         }
//     }
//
//     true
// }
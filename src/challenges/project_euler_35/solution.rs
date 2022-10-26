use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use prime_utils::is_prime_i32;


///
/// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
//
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//
// How many circular primes are there below one million?
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 35: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> u32 {
    let mut count = 0;

    // For numbers up to 1 millions
    for x in 2..1000000 {
        if !is_prime_i32(x) {
            continue;
        }

        // Get the integer as a string
        let x_string = x.to_string();
        let mut non_prime_value_found = false;

        // For each digit shift possible
        for shift in 1..x_string.chars().count() {
            // Split the string by index e.g. 12345 -> (1, 2345)
            let x_string_split = x_string.split_at(shift);

            // Join the pieces in the reverse order e.g. (1, 2345) -> 23451
            let shifted_x_string = x_string_split.1.to_owned() + x_string_split.0;
            let shifted_x = i32::from_str_radix(&shifted_x_string, 10).unwrap();

            // If the number isn't prime we need to break out of this loop and continue to the next x value
            if !is_prime_i32(shifted_x) {
                non_prime_value_found = true;
                break;
            }
        }

        // If one of the shifts isn't a prime number we go to the next x value
        if non_prime_value_found {
            continue;
        }

        // Reaching this point means the value must be a circular prime
        println!("Found Circular Prime: {}", x);
        count += 1;
    }

    count
}

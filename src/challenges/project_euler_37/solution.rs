use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/prime_utils.rs"] mod prime_utils;
#[path = "../../utils/number_char_utils.rs"] mod number_char_utils;

use prime_utils::is_prime_i32;
use number_char_utils::substring_i32;

///
/// The number 3797 has an interesting property. Being prime itself, it is possible to continuously remove digits from left to right, and remain prime at each stage: 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
//
// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
//
// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 37: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    let mut current = 10;
    let mut found = 0;
    let mut sum = 0;

    while found < 11 {
        current += 1;

        if !is_prime_i32(current) {
            continue;
        }

        let number_size = current.to_string().chars().count();

        let mut prime_failure = false;
        for i in 1..number_size {
            let front_check = substring_i32(current, 0, number_size - i);
            // println!("Number: {}, Front: {}", current, front_check);
            if !is_prime_i32(front_check) {
                prime_failure = true;
                break;
            }

            let back_check = substring_i32(current, i, number_size);
            // println!("Number: {}, Back: {}", current, back_check);
            if !is_prime_i32(back_check) {
                prime_failure = true;
                break;
            }
        }

        // If we failed one of the prime checks we should go to the next value
        if prime_failure {
            continue;
        }

        // If we reached here it's a valid number
        sum += current;
        found += 1;
        println!("Found: {}", current);

    }



    sum
}


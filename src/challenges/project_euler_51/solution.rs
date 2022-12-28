use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};

#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use prime_utils::is_prime_i32;

///
/// By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
//
// By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.
//
// Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 51: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> i32 {
    // Essentially this question a prime number
    // Then replace one number (all occurrences) with all other possible digits (0-9)
    // 8 of the 10 possibilities must be prime

    // Maybe I should keep track of primes and find the largest prime of the set?
    // Then when I've got the set I can find the smallest?

    // loop through all integers
    // if the number is prime add to the collection
    // For each number (0-9), find the indexes
    // Iterate through all primes, remove the digits at those indexes and check the value matches the current prime
    // E.g. 12345, check digit 3, 1245. If another number 1

    // There's one possible edge case which would fail when checking this, if the last prime has an additional same digit (e.g. 2999, when the others are 2**9)
    // It would fail...

    // It's not the most likely though and I can adapt if it fails due to that

    let mut i = 1000; // Going to assume it's larger than 1000
    let mut primes: HashSet<i32> = HashSet::new();

    loop {
        if is_prime_i32(i) {
            primes.insert(i);
        }

        let largest_prime_family = find_largest_prime_family(i, &primes);

        if largest_prime_family.len() == 8 {
            return *largest_prime_family.iter()
                .min()
                .unwrap();
        }

        i += 1;
    }
}

fn find_largest_prime_family(value: i32, found_primes: &HashSet<i32>) -> Vec<i32> {
    let mut current_largest = Vec::new();
    let value_string = value.to_string();
    // let current_index_map = get_index_map(&value_string);

    for digit in value_string.chars() {
        let mut current_prime_family = Vec::new();

        for i in 0..10 {
            let i_string = i.to_string();

            let new_string = value_string.replace(digit, &i_string);
            let new_number = i32::from_str_radix(&new_string, 10).unwrap();

            if found_primes.contains(&new_number) {
                current_prime_family.push(new_number);
            }
        }

        if current_prime_family.len() > current_largest.len() {
            current_largest = current_prime_family;
        }
    }

    current_largest
}


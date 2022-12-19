use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashSet};

#[path = "../../utils/prime_utils.rs"] mod prime_utils;
#[path = "../../utils/string_utils.rs"] mod string_utils;

use prime_utils::is_prime_i32;
use string_utils::sort;

///
/// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.
//
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.
//
// What 12-digit number do you form by concatenating the three terms in this sequence?
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 47: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> u64 {
    let mut current = 1000;
    let mut primes: BTreeSet<i32> = BTreeSet::new();

    loop {
        current += 1;

        if !is_prime_i32(current) {
            continue;
        }

        if current == 8147 {
            continue;
        }

        for prime in primes.iter().rev() {
            let diff = current - prime;
            let smallest_prime = prime - diff;

            if primes.contains(&smallest_prime) {
                let smallest_string = smallest_prime.to_string();
                let middle_string = prime.to_string();
                let biggest_string = current.to_string();

                let smallest_sorted = sort(&smallest_string);
                let middle_sorted = sort(&middle_string);
                let biggest_sorted = sort(&biggest_string);

                if smallest_sorted.eq(&middle_sorted) && middle_sorted.eq(&biggest_sorted) {
                    let number_string = smallest_string + &middle_string + &biggest_string;
                    return u64::from_str_radix(&number_string, 10).unwrap();
                }
            }
        }

        primes.insert(current);
    }
}

use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashSet};

#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use prime_utils::is_prime_i32;

///
/// The prime 41, can be written as the sum of six consecutive primes:
//
// 41 = 2 + 3 + 5 + 7 + 11 + 13
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
//
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.
//
// Which prime, below one-million, can be written as the sum of the most consecutive primes?
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 50: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> i32 {
    let mut current_largest = 0;
    let mut current_largest_prime = 0;
    let mut primes: Vec<i32> = Vec::new();

    for i in 0..1000000 {
        if !is_prime_i32(i) {
            continue;
        }

        primes.push(i);

        let sum_count = consecutive_prime_sum(&i, &primes);

        if sum_count > current_largest {
            current_largest = sum_count;
            current_largest_prime = i;
        }
    }

    current_largest_prime
}

fn consecutive_prime_sum(value: &i32, primes: &Vec<i32>) -> i32 {
    let mut prime_search_index: i32 = -1;
    let mut offset_beginning: i32 = -1;
    let mut sum = 0;

    loop {
        if prime_search_index == primes.len() as i32 {
            return 0;
        }

        if &sum == value { // We find the chain which begins earliest, primes increase so it will be the largest chain
            return prime_search_index - offset_beginning;
        }

        if &sum < value {
            prime_search_index += 1;
            sum += primes.get(prime_search_index as usize).unwrap()
        } else { // Sum is too large, move the beginning offset forward
            offset_beginning += 1;
            sum -= primes.get(offset_beginning as usize).unwrap()
        }
    }
}

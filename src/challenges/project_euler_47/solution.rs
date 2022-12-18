use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashSet};

#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use prime_utils::is_prime_i32;

///
/// The first two consecutive numbers to have two distinct prime factors are:
//
// 14 = 2 × 7
// 15 = 3 × 5
//
// The first three consecutive numbers to have three distinct prime factors are:
//
// 644 = 2² × 7 × 23
// 645 = 3 × 5 × 43
// 646 = 2 × 17 × 19.
//
// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 47: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> i32 {
    // Start with 4 primes
    let mut primes: Vec<i32> = Vec::from([2,3,5,7]);
    let mut n = 7; // First iteration will be 8
    let mut chain = 0; // How many in a row have the feature

    loop {
        n += 1;

        // Keep track of the smaller prime numbers we find
        if is_prime_i32(n) {
            primes.push(n);
            chain = 0;
            continue;
        }

        let prime_factors = find_prime_factors(n, &primes);

        if prime_factors.len() == 4 {
            chain += 1;

            if chain == 4 {
                return n - 3;
            }
        } else {
            chain = 0;
        }
    }
}

fn find_prime_factors(n: i32, other_primes: &Vec<i32>) -> HashSet<i32>{
    let mut current_n = n;
    let mut current_factors: HashSet<i32> = HashSet::new();

    for prime in other_primes.iter() {
        if current_n % prime == 0 {
            current_n = calculate_new_n(&current_n, prime);
            current_factors.insert(*prime);

            if current_n == 1 {
                return current_factors;
            }
        }
    }

    return HashSet::new();
}

fn calculate_new_n(n: &i32, prime: &i32) -> i32 {
    if n % prime == 0 {
        return calculate_new_n(&(n/prime), prime);
    }

    *n
}
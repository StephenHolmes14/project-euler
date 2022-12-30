use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};

#[path = "../../utils/vector_maths_utils.rs"] mod vector_utils;

use vector_utils::factorial;

/// https://projecteuler.net/problem=53
///
/// There are exactly ten ways of selecting three from five, 12345:
//
// 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//
// In combinatorics, we use the notation,
// .
//
// In general,
//
// , where , , and .
//
// It is not until , that a value exceeds one-million:
// .
//
// How many, not necessarily distinct, values of
//  for , are greater than one-million?
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 53: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> u64 {
    // n!/(r!(n-r)!)
    // Combinations always have a peak in the middle of the distribution
    // E.g. If I have 5 items and pick 3 of them
    // There are more possible arrangements compared to picking all 5 items (1 combination), or picking none of the items (1 combination)

    // So we should be able to find the values of n and r which have more than 1000000, without calculating the full factorial for all of them
    // We just need to find the first instance and last, then count the number between too

    // n!/(r!(n-r)!) > 1000000
    // n! > 1000000 * (r!(n-r)!)
    // There's some interesting optimisations we can do here
    // n! = n * n-1 * n-2 * ... 2 * 1
    // r is less than n, so n! > r!
    // so we can always say x * r! = n!
    // where x is n * n-1 * ... * r+1
    // in human terms, we can say there's some x which is the product of all numbers between r and n
    // Exclusive of r, and inclusive of n
    let mut n = 23;
    let mut count = 0;

    while n <= 100 {
        for r in 1..n/2 {
            let combinations = combinations(n, r);

            if combinations >= 1000000 {
                // In the example of 23 choose 10
                // We know that 23 choose 13 is the same value
                // So essentially 23 choose 10, 11, 12, 13 will be above 1000000
                // The first r - 1 we ignore (1-9 in the example)
                // We also ignore the last r (14-23 in the example)
                // So once we find some r which is above 100000
                // We know the number of r values which will also work
                // n - (r - 1) - r
                count += n - (r - 1) - r;
                break;
            }
        }

        n += 1;
    }

    count
}

fn combinations(n: u64, r: u64) -> u64 {
    // Maybe I can stop the overflow by swapping between multiplication and division

    // If we break apart the factorials we can say the equation equals:
    let mut top_values: Vec<u64> = (r+1..n+1).collect();
    let mut bottom_values: Vec<u64> = (1..n-r+1).collect();

    let mut current_value: f64 = 1.0;

    loop {
        if !top_values.is_empty() {
            current_value *= top_values.pop().unwrap() as f64;
        }

        if !bottom_values.is_empty() {
            current_value /= bottom_values.pop().unwrap() as f64;
        }

        if top_values.is_empty() && bottom_values.is_empty() {
            return current_value as u64;
        }
    }
}
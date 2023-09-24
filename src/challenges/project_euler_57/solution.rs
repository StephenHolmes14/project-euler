use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

#[path = "../../utils/vector_maths_utils.rs"] mod vector_maths_utils;

use vector_maths_utils::add;

/// It is possible to show that the square root of two can be expressed as an infinite continued fraction.
//
//
//
//
//
// By expanding this for the first four iterations, we get:
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
//
// The next three expansions are
//
// ,
//
// , and
//
// , but the eighth expansion,
//
// , is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.
//
// In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?

// Basically each iteration takes the previous value and does 1 + 1/(1 + N) where N is the last value
// First iteration N = 0, 1 + 1/2
// Second iteration 1 + 1/(2 + 1/2)
// Third iteration 1 + 1(2 + 1/(2 + 1/2))

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 56: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

fn find_answer() -> u128 {
    let mut count = 0;
    let mut prev_num = vec![3];
    let mut prev_denom = vec![2];

    for i in 1..1000 {
        let new_denom =  add(&prev_denom, &prev_num);
        let new_num = add(&prev_denom, &new_denom);

        let new_total_num_count = new_num.len();
        let new_denom_count = new_denom.len();
        if new_total_num_count > new_denom_count {
            count = count + 1;
        }

        prev_num = new_num;
        prev_denom = new_denom;
    }

    count
}
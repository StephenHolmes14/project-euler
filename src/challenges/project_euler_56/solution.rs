use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

#[path = "../../utils/vector_maths_utils.rs"] mod vector_maths_utils;

use vector_maths_utils::power_i32;
use vector_maths_utils::digit_sum;

/// A googol (
// ) is a massive number: one followed by one-hundred zeros;
//  is almost unimaginably large: one followed by two-hundred zeros. Despite their size, the sum of the digits in each number is only
// .
//
// Considering natural numbers of the form,
// , where
// , what is the maximum digital sum?

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 56: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

fn find_answer() -> u128 {
    let mut max = 0;

    for a in 1..100 {
        for b in 1..100 {
            let digits_vec = power_i32(a, b);
            let digits_sum = digit_sum(&digits_vec);

            if digits_sum > max {
                max = digits_sum;
            }
        }
    }

    max
}
use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashSet};

#[path = "../../utils/vector_maths_utils.rs"] mod vector_maths_utils;

use vector_maths_utils::add;
use vector_maths_utils::power_i32;
use vector_maths_utils::convert_vec_to_u64;

///
/// The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.
//
// Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 48: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> u64 {
    // Use vectors to track the digits
    let mut sum_vec = Vec::from([0]);

    for i in 1..1001 {
        let i_sq_vec = power_i32(i, i);
        sum_vec = add(&sum_vec, &i_sq_vec);

        // println!("i: {}, i_sq_vec: {:?}, sum_vec: {:?}", i, i_sq_vec, sum_vec);

    }

    let last_10_numbers: Vec<i32> = sum_vec.into_iter().take(10).collect();

    convert_vec_to_u64(&last_10_numbers)
}

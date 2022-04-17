use std::borrow::Borrow;
use std::collections::HashSet;
use std::time::Instant;

#[path = "../../utils/vector_maths_utils.rs"] mod vector_maths_utils;

use vector_maths_utils::power_i32;

/**
Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

1634 = 14 + 64 + 34 + 44
8208 = 84 + 24 + 04 + 84
9474 = 94 + 44 + 74 + 44
As 1 = 14 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;


    for i in 2..1000000 {
        let sum: i32 = i.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|d| d.pow(5) as i32)
            .sum();

        if i == sum {
            answer = answer + i;
        }

    }

    println!("Project Euler 30: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

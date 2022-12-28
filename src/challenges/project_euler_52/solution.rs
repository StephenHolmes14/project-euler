use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};

#[path = "../../utils/string_utils.rs"] mod string_utils;

use string_utils::sort;

///
/// It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits, but in a different order.
//
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same digits.
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 52: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> u64 {
    let mut i = 1;

    loop {
        let i_string = i.to_string();
        let sorted_string = sort(&i_string);

        for x in 2..7 {
            let new_x = i * x;
            let sorted_x_string = sort(&new_x.to_string());

            if !sorted_string.eq(&sorted_x_string) {
                break;
            }

            if x == 6 {
                return i;
            }
        }

        i += 1;
    }
}
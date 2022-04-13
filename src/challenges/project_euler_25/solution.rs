use std::time::Instant;
use crate::solution::vector_math_utils::add;

#[path = "../../utils/vector_maths_utils.rs"] mod vector_math_utils;

/**
The Fibonacci sequence is defined by the recurrence relation:

Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
Hence the first 12 terms will be:

F1 = 1
F2 = 1
F3 = 2
F4 = 3
F5 = 5
F6 = 8
F7 = 13
F8 = 21
F9 = 34
F10 = 55
F11 = 89
F12 = 144
The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence to contain 1000 digits?
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    let mut previous_fib: Vec<i32> = Vec::new();
    previous_fib.push(1);
    let mut current_fib: Vec<i32> = Vec::new();
    current_fib.push(1);
    let mut current_fib_index = 2; // This starts from 2 as both are 1

    while current_fib.len() < 1000 {
        current_fib_index = current_fib_index + 1;

        let new_fib = add(&previous_fib, &current_fib);

        println!("{}: {:?}", current_fib_index, new_fib);

        previous_fib = current_fib.clone();
        current_fib = new_fib.clone();
    }

    answer = current_fib_index;

    println!("Project Euler 24: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

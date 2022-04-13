use std::time::Instant;
use crate::solution::vector_math_utils::divide;

#[path = "../../utils/vector_maths_utils.rs"] mod vector_math_utils;

/**
A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

1/2	= 	0.5
1/3	= 	0.(3)
1/4	= 	0.25
1/5	= 	0.2
1/6	= 	0.1(6)
1/7	= 	0.(142857)
1/8	= 	0.125
1/9	= 	0.(1)
1/10	= 	0.1
Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
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

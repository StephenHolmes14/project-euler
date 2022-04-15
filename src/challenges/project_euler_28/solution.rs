use std::borrow::Borrow;
use std::time::Instant;


/**
Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is formed as follows:

21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13

It can be verified that the sum of the numbers on the diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    // I think I can loop through the numbers
    // And increment the gap every 4 times?

    let mut current_value = 1;
    answer = 1;

    let mut current_increment = 2;
    let mut increment_count = 0;

    let max = 1001*1001;

    while current_value <= max {
        current_value = current_value + current_increment;

        if current_value > max {
            break;
        }

        println!("{}", current_value);

        answer = answer + current_value;
        increment_count = increment_count + 1;

        if increment_count == 4 {
            increment_count = 0;
            current_increment = current_increment + 2;
        }
    }

    println!("Project Euler 26: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

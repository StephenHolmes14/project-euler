use std::time::Instant;

/**
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
 */
pub fn project_euler_20() {
    let start = Instant::now();
    let mut answer = 0;

    let one_hundred_factorial: u64 = (1..101).reduce(|x,y| x * y).unwrap();

    println!("Project Euler 20: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}
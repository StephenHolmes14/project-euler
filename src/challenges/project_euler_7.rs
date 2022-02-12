use std::time::Instant;

/**
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
 */
pub fn project_euler_7() {
    let start = Instant::now();
    let answer;

    // Start from 13 since the question tells us it's the 6th prime
    let mut current_number = 13;
    let mut prime_count = 6;

    loop {
        current_number += 1;

        if is_prime(current_number) {
            prime_count += 1;

            if prime_count == 10001 {
                answer = current_number;
                break;
            }
        }
    }

    println!("Project Euler 7: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn is_prime(value: i64) -> bool {
    let sqrt = (value as f64).sqrt() as i64;

    for possible_factor in 2..(sqrt+1) {
        if value % possible_factor == 0 {
            return false;
        }
    }

    true
}
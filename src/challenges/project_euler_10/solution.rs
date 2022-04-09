use std::time::Instant;

/**
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
 */
pub fn project_euler_10() {
    let start = Instant::now();
    let mut sum = 2;

    for i in 3..2000000 {
        if is_prime(i) {
            sum += i;
        }
    }

    println!("Project Euler 10: {}, Time Taken: {}", sum, start.elapsed().as_secs());
}

fn is_prime(value: u64) -> bool {
    let sqrt = (value as f64).sqrt() as u64;

    for possible_factor in 2..(sqrt+1) {
        if value % possible_factor == 0 {
            return false;
        }
    }

    true
}

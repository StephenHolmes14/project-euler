use std::time::Instant;

/**
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
 */
pub fn project_euler_3() {
    let start = Instant::now();
    let number:i64 = 600851475143;
    let sqrt_number = (number as f64).sqrt() as i64;

    let mut highest_prime_factor = 0;

    for possible_number_factor in 2..sqrt_number {
        // If this number isn't a factor continue
        if number % possible_number_factor != 0 {
            continue;
        }

        if is_prime(possible_number_factor) && possible_number_factor > highest_prime_factor {
            highest_prime_factor = possible_number_factor;
        }

        // The number was a factor
        let other_value = number / possible_number_factor;

        if is_prime(other_value) && other_value > highest_prime_factor {
            highest_prime_factor = other_value;
        }
    }

    println!("Project Euler 3: {}, Time Taken: {}", highest_prime_factor, start.elapsed().as_secs());
}

fn is_prime(value: i64) -> bool {
    let sqrt = (value as f64).sqrt() as i64;

    for possible_factor in 2..sqrt {
        if value % possible_factor == 0 {
            return false;
        }
    }

    true
}
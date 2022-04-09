use std::time::Instant;

/**
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
 */
pub fn project_euler_4() {
    let start = Instant::now();

    let smallest_factor = 100;
    let largest_factor = 1000;

    let mut highest_found = 0;

    for i in (smallest_factor..largest_factor).rev() {
        let mut found = 0;

        for j in (smallest_factor..largest_factor).rev() {
            let value = i * j;

            if is_palindrome(value) && value > found {
                found = value;
                break;
            }
        }

        if found > highest_found {
            highest_found = found;
            continue;
        }
    }

    println!("Project Euler 4: {}, Time Taken: {}", highest_found, start.elapsed().as_secs());
}

fn is_palindrome(value: i32) -> bool {
    let value_string = value.to_string();

    for i in 0..value_string.len()/2 {
        if !&value_string.chars().nth(i).unwrap().eq(&value_string.chars().nth_back(i).unwrap()) {
            return false;
        }
    }

    true
}

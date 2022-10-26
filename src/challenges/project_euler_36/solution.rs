use std::borrow::Borrow;
use std::time::Instant;


///
/// The number, 197, is called a circular prime because all rotations of the digits: 197, 971, and 719, are themselves prime.
//
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//
// How many circular primes are there below one million?
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 36: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> u32 {
    let mut count = 0;

    // 9! = 362880
    // That is 6 digits. If we check the 7 digit number 9999999, 362880 * 7 = 2540160 - which is much less, therefore it cannot be higher than 9999999

    for x in 10..1000000 {
        let x_string = x.to_string();

        for shift in 1..x_string.chars().count() {
            let new_x_string = x_string.
        }

        let sum: u32 = x_string.chars()
            .map(|i| i.to_digit(10).unwrap())
            .map(|i| factorial(i))
            .sum();

        if sum == x {
            println!("Found: {}", x);
            total_sum += sum;
        }
    }

    total_sum
}


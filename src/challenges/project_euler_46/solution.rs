use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::BTreeSet;

#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use prime_utils::is_prime_i32;

///
/// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.
//
// 9 = 7 + 2×12
// 15 = 7 + 2×22
// 21 = 3 + 2×32
// 25 = 7 + 2×32
// 27 = 19 + 2×22
// 33 = 31 + 2×12
//
// It turns out that the conjecture was false.
//
// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 46: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

pub fn find_answer() -> i32 {
    // 2 is the only even prime, if we start after 2 we can only check odd numbers
    let mut primes: BTreeSet<i32> = BTreeSet::from([2]);
    let mut n = 1; // First iteration will be 3


    loop {
        n += 2; // We only care about odd numbers

        // Keep track of the smaller prime numbers we find
        if is_prime_i32(n) {
            primes.insert(n);
            continue;
        }

        // Numbers which reach here are composite

        let mut found = false;
        for prime in primes.iter() {
            let two_squared_number = n - prime;
            if two_squared_number % 2 != 0 { // Needs to be even
                continue;
            }

            let sq = two_squared_number.div(2) as f64;
            let sq_rt = f64::sqrt(sq);

            // Easy way to check if an f64 is an integer
            // Basically "is this an integer, therefore sq is a square number"
            if sq_rt == (sq_rt as u32) as f64 {
                found = true;
                break; // If we find one which works there's no need to continue
            }
        }

        if !found {
            return n;
        }
    }
}

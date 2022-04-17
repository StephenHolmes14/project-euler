use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use prime_utils::is_prime_i64;

/**
Euler discovered the remarkable quadratic formula:


It turns out that the formula will produce 40 primes for the consecutive integer values . However, when  is divisible by 41, and certainly when  is clearly divisible by 41.

The incredible formula  was discovered, which produces 80 primes for the consecutive values . The product of the coefficients, −79 and 1601, is −126479.

Considering quadratics of the form:

, where  and

where  is the modulus/absolute value of
e.g.  and
Find the product of the coefficients,  and , for the quadratic expression that produces the maximum number of primes for consecutive values of , starting with .
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;
    let mut answer_count = 0;

    // brute force

    for a in i64::from(-999)..1000 {
        if !is_prime_i64(a.abs()) {
            continue;
        }


        for b in i64::from(-1000)..1001 {
            if !is_prime_i64(b.abs()) {
                continue;
            }

            println!("Found Prime b: {}", b);

            let count = count_primes(a, b);

            if count > answer_count {
                answer = a * b;
                answer_count = count;
            }
        }
    }

    println!("Project Euler 27: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn count_primes(a: i64, b: i64) -> i64 {
    let mut only_primes_found = true;
    let mut n = 1;

    while only_primes_found {
        let value = i64::pow(n,2) + a*n + b;

        if !is_prime_i64(value.abs()) {
            break;
        }

        n = n + 1;
    }

    return n;
}

use std::time::Instant;

/**
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
 */
pub fn project_euler_14() {
    let start = Instant::now();
    let mut answer = 0;
    let mut answer_count = 0;

    for n in (1..1000000).rev() {
        let count = calculate_chain_count(n);

        if count > answer_count {
            answer = n;
            answer_count = count;
        }
    }

    println!("Project Euler 14: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn calculate_chain_count(mut n: u64) -> u64 {
    let mut count = 1;
    loop {
        if n == 1 {
            return count;
        }

        n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
        count += 1;
    }
}
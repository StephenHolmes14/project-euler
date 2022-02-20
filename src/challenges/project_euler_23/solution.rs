use std::collections::HashSet;
use std::time::Instant;

/**
A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.

A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer: u64 = 0;

    let mut abundant_number: Vec<u64> = Vec::new();

    for i in 1..28124 {
        let divisor_sum = find_divisor_sum(i);

        if divisor_sum > i {
            abundant_number.push(i as u64);
        }
    }

    let abundunt_set: HashSet<u64> = HashSet::from_iter(abundant_number.iter()
        .flat_map(|x| abundant_number.iter()
            .map(|y| x.clone() + y)
            .filter(|x| x < &28124)));

    answer = (1..28124).sum::<u64>() - abundunt_set.iter().sum::<u64>();

    println!("Project Euler 23: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn find_divisor_sum(value: i32) -> i32 {
    let mut divisors: Vec<i32> = Vec::new();

    let sqrt = (value as f64).sqrt();

    for possible_factor in 1..(sqrt as i32 +1) {
        if value % possible_factor == 0 {
            if possible_factor == 1 || possible_factor as f64 == sqrt {
                divisors.push(possible_factor);
            } else {
                divisors.push(value/possible_factor);
                divisors.push(possible_factor);
            }
        }
    }

    divisors.into_iter()
        .reduce(|x,y| x + y)
        .unwrap()
}

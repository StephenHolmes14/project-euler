use std::time::Instant;

/**
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.
 */
pub fn project_euler_21() {
    let start = Instant::now();
    let mut answer = 0;

    let mut unmatched_numbers: Vec<(i32, i32)> = Vec::new();

    for i in 1..10000 {
        let divisor_sum = find_divisor_sum(i);

        let possible_match = unmatched_numbers.iter()
            .find(|(x,y)| divisor_sum.eq(x) && i.eq(y));

        // This pair exists
        if possible_match.is_some() {
            let result = possible_match.unwrap().to_owned();
            answer += result.0 + result.1;

            continue;
        }

        unmatched_numbers.push((i, divisor_sum));
    }

    println!("Project Euler 21: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn find_divisor_sum(value: i32) -> i32 {
    let mut divisors: Vec<i32> = Vec::new();

    let sqrt = (value as f64).sqrt() as i32;

    for possible_factor in 1..(sqrt+1) {
        if value % possible_factor == 0 {
            if possible_factor == 1 || possible_factor == sqrt {
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
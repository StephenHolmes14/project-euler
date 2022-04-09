use std::time::Instant;

/**
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
 */
pub fn project_euler_20() {
    let start = Instant::now();
    let mut answer = 0;

    // Adjusted from 16 algo
    let mut digits: Vec<i32> = Vec::new();

    digits.push(1);

    // This can go way above 10 now
    let mut current_carry_over = 0;

    for n in 1..101 {
        current_carry_over = 0;
        for digit_index in 0..digits.len() {
            let mut new_digit = digits.get(digit_index).unwrap() * n;

            let new_digit_adjusted = (current_carry_over + new_digit) % 10;

            digits[digit_index] = new_digit_adjusted;

            current_carry_over = (current_carry_over + new_digit - new_digit_adjusted)/10;

            if digit_index == digits.len()-1 && current_carry_over > 0 {
                // Current carry over digits in reverse order
                current_carry_over.to_string()
                    .chars()
                    .rev()
                    .map(|c| i32::from_str_radix(&c.to_string(), 10).unwrap())
                    .for_each(|x| digits.push(x));
            }
        }
    }

    answer = digits.into_iter().reduce(|x,y| x + y).unwrap();

    println!("Project Euler 20: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}
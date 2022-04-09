use std::time::Instant;

/**
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
 */
pub fn project_euler_16() {
    let start = Instant::now();

    let mut digits: Vec<u64> = Vec::new();
    digits.push(1);
    let mut current_carry_over = 0;

    for i in 1..1001 {
        current_carry_over = 0;
        for digit_index in 0..digits.len() {
            let mut new_digit = digits.get(digit_index).unwrap() * 2;

            let new_digit_adjusted = new_digit % 10;
            digits[digit_index] = new_digit_adjusted + current_carry_over;

            current_carry_over = (new_digit - new_digit_adjusted)/10;

            if digit_index == digits.len()-1 && current_carry_over > 0 {
                digits.push(current_carry_over);
            }
        }
    }

    let answer = digits.into_iter().reduce(|x,y| x + y).unwrap();

    println!("Project Euler 16: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

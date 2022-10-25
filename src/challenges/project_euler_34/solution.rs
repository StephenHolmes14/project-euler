use std::borrow::Borrow;
use std::time::Instant;


///
/// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//
// Note: As 1! = 1 and 2! = 2 are not sums they are not included.
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 30: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> u32 {
    let mut total_sum = 0;

    // 9! = 362880
    // That is 6 digits. If we check the 7 digit number 9999999, 362880 * 7 = 2540160 - which is much less, therefore it cannot be higher than 9999999
    //

    for x in 3..9999999 {
        let x_string = x.to_string();
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

fn factorial(n: u32) -> u32 {
    (1..(n + 1)).product()
}
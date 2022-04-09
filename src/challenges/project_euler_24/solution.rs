use std::collections::HashSet;
use std::time::Instant;

/**
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
 */
pub fn calculate() {
    let start = Instant::now();

    // 012 - 0 - 0 0 0
    // 021 - 1 - 0
    // 102 - 2
    // 120 - 3
    // 201 - 4
    // 210 - 5

    // OK So the total count will be n! because we always pick everything

    // So for each (n-1)! the first number will increment

    // 1 000 000 / 362880 > 2.75...

    // So the first number MUST be 2

    // Then for the next digit 1000000 - 2 * 362880 =  274240

    // Each (n-2)! the second number will increment

    // Ok lets try program this

    let mut current_number = 999999; // because we don't care about the 0th case

    let mut answer = String::new();

    let number = 10;

    for i in 0..10 {

        let factorial: i32 = (1..(number-i)).product();

        let current_digit_index = current_number / factorial;

        println!("Current Number: {}", current_number);
        println!("Factorial: {}", factorial);
        println!("Current Digit Index: {}", current_digit_index);


        let mut current_digit = 0;

        let mut available_count = 0;

        for x in 0..10 {
            if answer.contains(&x.to_string()) {
                continue;
            }
            available_count = available_count + 1;

            if available_count == (current_digit_index + 1) {
                current_digit = x;
                break;
            }
        }

        answer.push_str(&current_digit.to_string());

        println!("Current Answer: {}", answer);

        current_number = current_number - factorial * current_digit_index;
    }


    println!("Project Euler 24: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

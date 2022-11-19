use std::time::Instant;

///
/// An irrational decimal fraction is created by concatenating the positive integers:
//
// 0.123456789101112131415161718192021...
//
// It can be seen that the 12th digit of the fractional part is 1.
//
// If dn represents the nth digit of the fractional part, find the value of the following expression.
//
// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 40: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    const INDEX_TO_FIND_MULTIPLIER: i32 = 10;
    const MAX_INDEX: i32 = 1000000;

    let mut solution = 1;
    let mut next_index_to_find: i32 = 1;
    let mut current_index: i32 = 0;
    let mut current_integer = 0;


    // I think I can just keep track of the n
    // It can't be more than this as this is the last digit we need and each iteration will definitely add at least 1
    while current_index < MAX_INDEX {
        current_integer += 1;
        let current_integer_string = current_integer.to_string();
        let current_integer_string_length = current_integer_string.len() as i32;

        // If one of these is a digit we need to find
        if current_index + current_integer_string_length >= next_index_to_find {
            let index_of_current_integer = next_index_to_find - current_index - 1; // -1 as index will start from 0
            let nth_value_char = current_integer_string.chars().into_iter().nth(index_of_current_integer as usize).unwrap();
            solution *= i32::from_str_radix(&nth_value_char.to_string(), 10).unwrap();
            next_index_to_find *= INDEX_TO_FIND_MULTIPLIER;
        }

        current_index += current_integer_string_length;
    }

    solution
}


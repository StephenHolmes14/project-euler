use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/vector_maths_utils.rs"] mod vector_math_utils;

/**
A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

1/2	= 	0.5
1/3	= 	0.(3)
1/4	= 	0.25
1/5	= 	0.2
1/6	= 	0.1(6)
1/7	= 	0.(142857)
1/8	= 	0.125
1/9	= 	0.(1)
1/10	= 	0.1
Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;
    let mut answer_length = 0;

    for i in 1..1000 {
        let mut found_recursion = false;

        let mut current_value = String::new();
        let mut current_carryover = 10;
        let mut carryover_vector: Vec<i32> = Vec::new();

        while !found_recursion {
            let new_digit = current_carryover / i;
            current_value += &new_digit.to_string();

            current_carryover = (current_carryover % i) * 10;
            carryover_vector.push(current_carryover);

            if current_carryover == 0 {
                break;
            }

            let recursion_length = find_recursion_length(&current_value, &carryover_vector);

            if recursion_length > 0 {
                found_recursion = true;

                if recursion_length > answer_length {
                    answer = i;
                    answer_length = recursion_length;
                }
            }
        }

        println!("i: {}, value: {}, current longest: {}", i, current_value, answer_length);
    }

    println!("Project Euler 26: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}


fn find_recursion_length(value: &String, carryover_vector: &Vec<i32>) -> i32 {
    let mut current_possible_recursion = String::new();
    let current_carryover = carryover_vector.get(carryover_vector.len() - 1).unwrap();

    let mut reversed_value_chars = value.chars().clone().rev();

    for i in 0..value.len() {
        let c = reversed_value_chars.clone().nth(i).unwrap(); // this isn't right lol

        if current_possible_recursion.len() > 0
            && current_possible_recursion.chars().nth(0).unwrap().eq(&c)
            && carryover_vector.get(carryover_vector.len() - 1 - i).unwrap().eq(current_carryover){
            return current_possible_recursion.len() as i32;
        }

        current_possible_recursion.push(c);
    }

    return 0;
}
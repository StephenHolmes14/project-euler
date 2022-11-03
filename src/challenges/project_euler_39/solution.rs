use std::borrow::Borrow;
use std::time::Instant;

#[path = "../../utils/string_utils.rs"] mod string_utils;

use string_utils::sort;

///
/// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c}, there are exactly three solutions for p = 120.
//
// {20,48,52}, {24,45,51}, {30,40,50}
//
// For which value of p ≤ 1000, is the number of solutions maximised?
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 39: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    let mut solution = 0;
    let mut solution_count = 0;

    // a^2 + b^2 = c^2
    // we want a + b + c = i
    // we also know a + b >= c
    // so we can narrow down the search a bit
    // if we search for c first, we only have to check up to i/2 as a + b is more than c
    // therefore the remaining value for a + b must be at least i/2
    for i in 2i32..1001 {
        let mut current_count = 0;

        for c in 1..(i/2) {
            let c_2 = c.pow(2);
            for a in 1..((i-c)/2) {
                let b = i - a - c;

                if b > 0 && c_2 == a.pow(2) + b.pow(2) {
                    current_count += 1;
                    println!("Solution Found for {}: {}, {}, {}", i, a, b, c);
                }
            }
        }

        if current_count > solution_count {
            solution_count = current_count;
            solution = i;
        }
    }

    solution
}


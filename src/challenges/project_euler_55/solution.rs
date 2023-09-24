use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

#[path = "../../utils/number_char_utils.rs"] mod number_char_utils;
#[path = "../../utils/palindrome_utils.rs"] mod palindrome_utils;

use number_char_utils::reverse_u128;
use palindrome_utils::is_palindrome_u128;

/// If we take
// , reverse and add,
// , which is palindromic.
//
// Not all numbers produce palindromes so quickly. For example,
//
//
// That is,
//  took three iterations to arrive at a palindrome.
//
// Although no one has proved it yet, it is thought that some numbers, like
// , never produce a palindrome. A number that never forms a palindrome through the reverse and add process is called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome. In fact,
//  is the first number to be shown to require over fifty iterations before producing a palindrome:
//  (
//  iterations,
// -digits).
//
// Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is
// .
//
// How many Lychrel numbers are there below ten-thousand?
//
// NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 55: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

fn find_answer() -> u64 {
    let mut count = 0;
    // Max Iterations is 50
    for n in 0..10000 {
        if is_lychrel(n) {
            count = count + 1;
        }
    }

    10000 - count
}

fn is_lychrel(n: u128) -> bool {
    let mut current= n;
    for i in 0..50 {
        current = current + reverse_u128(current);

        if is_palindrome_u128(current) {
            return true;
        }
    }

    false
}
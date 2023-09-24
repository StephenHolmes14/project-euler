use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

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

pub fn find_answer() -> u64 {
    let all_hands_string = fs::read_to_string("src/challenges/project_euler_54/poker.txt").unwrap();
    let games_won = all_hands_string.split("\n")
        .map(|h| make_game_from_string(String::from(h)))
        .filter(|g| does_player1_win(&g))
        .count();

    games_won as u64
}
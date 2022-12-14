use std::ops::Add;
use std::time::Instant;
use std::fs;

#[path = "../../utils/string_utils.rs"] mod string_utils;
#[path = "../../utils/prime_utils.rs"] mod prime_utils;

use string_utils::sort;
use prime_utils::is_prime_i32;

///
/// The nth term of the sequence of triangle numbers is given by, tn = ½n(n+1); so the first ten triangle numbers are:
//
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
//
// By converting each letter in a word to a number corresponding to its alphabetical position and adding these values we form a word value. For example, the word value for SKY is 19 + 11 + 25 = 55 = t10. If the word value is a triangle number then we shall call the word a triangle word.
//
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly two-thousand common English words, how many are triangle words?
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 42: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> usize {
    let first_twenty_numbers: Vec<i32> = (1..21)
        .map(|n| (n*(n+1))/2)
        .collect();
    let words_string = fs::read_to_string("src/challenges/project_euler_42/words.txt").unwrap();
    words_string.replace('"', "")
        .split(",")
        .map(|x| x.to_string())
        .filter(|w| is_triangle_word(&w, &first_twenty_numbers))
        .count()
}

fn is_triangle_word(word: &String, triangle_numbers: &Vec<i32>) -> bool {
    let word_number = get_word_number(&word);
    triangle_numbers.contains(&word_number)
}

fn get_word_number(word: &String) -> i32 {
    word.chars()
        .map(|c| get_character_number(c))
        .sum()
}

fn get_character_number(character: char) -> i32 {
    i32::from(character as u8 - 64) // Document is all caps, unicode int for A is 65 so take away 64 for quick conversion
}
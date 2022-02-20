use std::fs;
use std::time::Instant;

/**
Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.

For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.

What is the total of all the name scores in the file?
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;


    // First read the file
    let names_string = fs::read_to_string("src/challenges/project_euler_22/names.txt").unwrap();
    let mut names: Vec<String> = names_string.replace('"', "")
        .split(",")
        .map(|x| x.to_string())
        .collect();

    names.sort();

    let mut index = 0;

    for name in names {
        index += 1;
        answer += find_sum(name, index);
    }

    println!("Project Euler 22: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn find_sum(string: String, index: u32) -> u32 {
    let numbers_iter = string.chars()
        .map(|x| find_char_number(x));

    let mut sum = 0;

    for num in numbers_iter {
        sum += num * index;
    }

    sum
}

fn find_char_number(char: char) -> u32 {
    char as u32 - '@' as u32
}

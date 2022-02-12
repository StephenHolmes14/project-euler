use std::time::Instant;

/**
If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?


NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.
 */
pub fn project_euler_17() {
    let start = Instant::now();
    let mut answer = 0;

    for i in 1..1001 {
        answer += convert_number_to_letter_count(i);
    }

    println!("Project Euler 17: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

fn convert_number_to_letter_count(number: i32) -> i32 {
    if number == 1000 {
        return "onethousand".len() as i32;
    }

    if number > 99 {
        // HUNDRED
        let hundred_word_count = 7;
        let mut count = 0;
        let hundreds = number / 100;
        count += convert_number_to_letter_count(hundreds) + hundred_word_count;

        let remainder = number - (hundreds * 100);

        if remainder > 0 {
            // X Hundred AND Y
            return count + 3 + convert_number_to_letter_count(remainder);
        }
        return count;
    }

    if number == 90 { return "ninety".len() as i32; }
    if number == 80 { return "eighty".len() as i32; }
    if number == 70 { return "seventy".len() as i32; }
    if number == 60 { return "sixty".len() as i32; }
    if number == 50 { return "fifty".len() as i32; }
    if number == 40 { return "forty".len() as i32; }
    if number == 30 { return "thirty".len() as i32; }

    if number > 20 {
        let tens = number / 10;
        let ones = number - (tens * 10);

        return convert_number_to_letter_count(tens*10) + convert_number_to_letter_count(ones);
    }

    if number == 20 { return "twenty".len() as i32; }
    if number == 19 { return "nineteen".len() as i32; }
    if number == 18 { return "eighteen".len() as i32; }
    if number == 17 { return "seventeen".len() as i32; }
    if number == 16 { return "sixteen".len() as i32; }
    if number == 15 { return "fifteen".len() as i32; }
    if number == 14 { return "fourteen".len() as i32; }
    if number == 13 { return "thirteen".len() as i32; }
    if number == 12 { return "twelve".len() as i32; }
    if number == 11 { return "eleven".len() as i32; }
    if number == 10 { return "ten".len() as i32; }
    if number == 9 { return "nine".len() as i32; }
    if number == 8 { return "eight".len() as i32; }
    if number == 7 { return "seven".len() as i32; }
    if number == 6 { return "six".len() as i32; }
    if number == 5 { return "five".len() as i32; }
    if number == 4 { return "four".len() as i32; }
    if number == 3 { return "three".len() as i32; }
    if number == 2 { return "two".len() as i32; }
    if number == 1 { return "one".len() as i32; }
    return 0;
}


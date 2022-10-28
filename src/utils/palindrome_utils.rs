pub fn is_palindrome_i32(value: i32) -> bool {
    let value_string = value.to_string();

    is_palindrome_string(&value_string)
}

pub fn is_palindrome_string(value: &str) -> bool {
    // For each value up until the middle
    for i in 0..value.len()/2 {
        // Get the ith character from the left
        let left_char = &value.chars().nth(i).unwrap();

        // Get the ith character from the right
        let right_char = &value.chars().nth_back(i).unwrap();

        // If they both aren't equal return false
        if !left_char.eq(right_char) {
            return false;
        }
    }

    // All left + right characters were equal, therefore it is a palindrome
    true
}
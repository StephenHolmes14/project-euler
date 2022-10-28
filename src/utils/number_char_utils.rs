
// The start index is inclusive, the end index is exclusive
pub fn substring_i32(n:i32, start_index: usize, end_index: usize) -> i32 {
    let substring: String = n.to_string() // Turn it into a String
        .chars() // Iterate the chars
        .skip(start_index) // Skip to the start index
        .take(end_index - start_index) // Take the difference between start and end index
        .collect(); // Collect to String

    i32::from_str_radix(&substring, 10).unwrap()
}
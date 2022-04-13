pub fn add(first_number: &Vec<i32>, second_number: &Vec<i32>) -> Vec<i32> {
    let largest_vector_size = first_number.len().max(second_number.len());

    let mut carry_over = 0;
    let mut return_vector: Vec<i32> = Vec::new();
    for i in 0..largest_vector_size {
        let first_digit = first_number.get(i).or(Option::from(&0));
        let second_digit = second_number.get(i).or(Option::from(&0));

        let new_digit_full = first_digit.unwrap() + second_digit.unwrap() + carry_over;
        carry_over = new_digit_full / 10;
        let new_digit = new_digit_full % 10;

        return_vector.push(new_digit);
    }
    if carry_over != 0 {
        return_vector.push(carry_over);
    }

    return_vector
}
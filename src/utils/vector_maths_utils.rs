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
        carry_over.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .rev()
            .for_each(|d| return_vector.push(d));
    }

    return_vector
}

pub fn multiply_by_i32(number_vector: &Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut return_vector: Vec<i32> = Vec::new();

    let mut carry_over = 0;
    for i in 0..number_vector.len() {
        let current = number_vector.get(i).unwrap();

        let new_digit_full = current * multiplier + carry_over;
        carry_over = new_digit_full / 10;
        let new_digit = new_digit_full % 10;

        return_vector.push(new_digit);
    }

    if carry_over != 0 {
        carry_over.to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .rev()
            .for_each(|d| return_vector.push(d));
    }

    return return_vector;
}

pub fn power_i32(number: i32, power: i32) -> Vec<i32> {
    let number_vector: Vec<i32> = number.to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .rev() // First element should always be the smallest for these utils
        .collect();

    let mut return_vector: Vec<i32> = number_vector.clone();

    for i in 1..power {
        return_vector = multiply_by_i32(&return_vector, number);
    }

    return return_vector;
}

pub fn convert_i32_to_vec(i: i32) -> Vec<i32> {
    multiply_by_i32(&Vec::from([1]), i)
}

pub fn convert_vec_to_i32(vec: &Vec<i32>) -> i32 {
    // This vector maths module treats the smaller numbers as lower index
    // Therefore to convert we need to reverse the order
    let string_i32: String = vec.iter()
        .rev()
        .map(|x| x.to_string())
        .collect();

    i32::from_str_radix(&string_i32, 10).unwrap()
}

pub fn convert_vec_to_u64(vec: &Vec<i32>) -> u64 {
    // This vector maths module treats the smaller numbers as lower index
    // Therefore to convert we need to reverse the order
    let string_u64: String = vec.iter()
        .rev()
        .map(|x| x.to_string())
        .collect();

    u64::from_str_radix(&string_u64, 10).unwrap()
}

pub fn factorial(factorial: i32) -> Vec<i32> {
    let mut factorial_vec = convert_i32_to_vec(factorial);

    for i in 2..factorial {
        factorial_vec = multiply_by_i32(&factorial_vec, i);
    }

    factorial_vec
}
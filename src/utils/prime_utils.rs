pub fn is_prime_i64(value: i64) -> bool {
    if value == 1 {
        return false;
    }

    if value == 0 {
        return false;
    }

    let sqrt = (value as f64).sqrt() as i64;

    for possible_factor in 2..(sqrt+1) {
        if value % possible_factor == 0 {
            return false;
        }
    }

    true
}

pub fn is_prime_i32(value: i32) -> bool {
    if value == 1 {
        return false;
    }

    let sqrt = (value as f64).sqrt() as i32;

    for possible_factor in 2..(sqrt+1) {
        if value % possible_factor == 0 {
            return false;
        }
    }

    true
}
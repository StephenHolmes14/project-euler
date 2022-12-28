pub fn is_prime_i64(value: i64) -> bool {
    if value == 2 || value == 3 {
        return true;
    }

    if value <= 1 || value % 2 == 0 || value % 3 == 0 {
        return false;
    }

    let mut i = 5;

    loop {
        // Primes come in pairs, one less than the sqrt and one larger (or both equal)
        // If we reach a value of i larger than sqrt of the number, we must have a prime number
        if i * i > value {
            return true;
        }

        // If the current i is a factor of the value, it's not prime
        // We also check the current i + 2 in this algorithm
        if value % i == 0 || value % (i + 2) == 0 {
            return false;
        }

        // We only need to check every 6 numbers
        i += 6;
    }
}

pub fn is_prime_i32(value: i32) -> bool {
    if value == 2 || value == 3 {
        return true;
    }
    
    if value <= 1 || value % 2 == 0 || value % 3 == 0 {
        return false;
    }
    
    let mut i = 5;

    loop {
        // Primes come in pairs, one less than the sqrt and one larger (or both equal)
        // If we reach a value of i larger than sqrt of the number, we must have a prime number
        if i * i > value {
            return true;
        }

        // If the current i is a factor of the value, it's not prime
        // We also check the current i + 2 in this algorithm
        if value % i == 0 || value % (i + 2) == 0 {
            return false;
        }

        // We only need to check every 6 numbers
        i += 6;
    }
}
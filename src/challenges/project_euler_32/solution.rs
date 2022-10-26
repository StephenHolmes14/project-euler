use std::borrow::Borrow;
use std::time::Instant;


///
/// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
/// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier, and product is 1 through 9 pandigital.
/// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a 1 through 9 pandigital.
/// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    // Initial Thoughts:
    //     987654321 is the largest pan digital
    //     The values have to be a * b = c, where a, b, c containing every digit from 1 - 9 only once overall
    //     So xa * yb = zc and x + y + z = 9 where x, y, z are the number of digits
    //     Quick maths
    //     x = 9 - y - z
    //     y = 9 - x - z
    //     z = 9 - x - y Is this helpful? Probably not.
    //
    //     Maybe look at max values
    //     None can be 8 or 9 as all must have some value
    //     If x is 7, y is 1 and z is 1 - impossible in any combination.
    //     Ah but more generically I can z = x + (y - 1) or z = x + y
    //     E.g. 1 * 1 = 1 (z = x + (y - 1))
    //     2 * 5 = 10 (z = x + y)
    //     9 * 9 = 81 (z = x + y)
    //
    //     We could prove but induction but nah I'm pretty sure
    //
    //     one more check
    //
    //     10 * 10 = 100 ( z = x + (y - 1)) )
    //     99 * 99 = 9801 ( z = x + y )
    //     Looks good
    //
    //     So the minimum z can be is x + (y - 1), the maximum can be z = x + y
    //
    //     x + y + z = 9
    //     x + y = 9 - z
    //
    //     z = x + y
    //     z = 9 - z // from above
    //     2z = 9
    //     z = 9/2 (Not possible as must be integer)
    //
    //     Lets try z = x + y - 1
    //     z = 9 - z + 1
    //     2z = 10
    //     z = 5 // OK
    //
    //     x + y = 5
    //     possible options, 1, 4, and 2, 3
    //
    //     OK so I need to search
    //     x = 2 - 9
    //     y = 1000 - 9999
    //
    //     x = 10 - 99
    //     y = 100 - 999

    answer = find_sum();

    println!("Project Euler 32: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_sum() -> i32 {
    let all_digits_string = "123456789";
    let mut found = Vec::new();
    let mut sum = 0;

    for x in 1..100 {
        let x_string = x.to_string();
        for y in 100..10000 {
            let z = x * y;

            // if z < 10000 {
            //     continue;
            // }

            let y_string = y.to_string();
            let z_string = z.to_string();

            let full_string: String = y_string + &z_string + &x_string;
            let mut full_string_chars: Vec<char> = full_string.chars().collect();
            full_string_chars.sort();
            let full_string_sorted: String = full_string_chars.into_iter().collect();

            if &full_string_sorted == all_digits_string {
                println!("Found x: {}, y: {}, z: {}, String: {}", x, y, z, full_string_sorted);

                if !found.contains(&z) {
                    sum += z;
                    found.push(z);
                }

            }

        }
    }

    sum
}

use std::borrow::Borrow;
use std::time::Instant;


///
/// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
//
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
//
// There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.
//
// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    // Initial Thoughts:
    // I need to check all digits 10 - 99, in two loops
    // If the digits have a shared divisor then it should be checked
    // If the new values after the division have resulted in the same digit being remove from both numerator and denominator then multiple by denom

    answer = find_answer();

    println!("Project Euler 33: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

pub fn find_answer() -> i32 {
    let mut total_denominator = 1;
    let mut total_numerator = 1;

    for x in 10..100 {
        let x_string = x.to_string();
        for y in 10..x {
            let y_string = y.to_string();

            for divisor in 2..x {
                if x % divisor != 0 || y % divisor != 0 {
                    continue;
                }

                let x_after = x / divisor;
                let y_after = y / divisor;

                let x_after_string = x_after.to_string();
                let y_after_string = y_after.to_string();

                let x_removed_digit = x_string.replace(&x_after_string, "");
                let y_removed_digit = y_string.replace(&y_after_string, "");

                if &x_removed_digit == "" || &x_removed_digit == "0" {
                    continue;
                }

                if y_removed_digit != x_removed_digit  {
                    continue;
                }

                println!("Found: {} / {}", y, x);

                // x is the denominator
                total_denominator = total_denominator * x_after;
                total_numerator = total_numerator * y_after;
            }

        }
    }

    println!("Result: {} / {}", total_numerator, total_denominator);

    total_denominator // This worked but I only found 3 out of the 4 apparent values based on the question - very odd
}

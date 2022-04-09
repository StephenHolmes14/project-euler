use std::time::Instant;

/**
The sum of the squares of the first ten natural numbers is,

The square of the sum of the first ten natural numbers is,

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is .

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 */
pub fn project_euler_6() {
    // There might be an equation I could use here

    let start = Instant::now();

    let mut i = 1;
    let mut sum = 0;
    let mut sum_squares = 0;

    loop {
        sum += i;
        sum_squares += i*i;

        i += 1;

        if i > 100 {
            break;
        }
    }

    let difference = (sum*sum) - sum_squares;

    println!("Project Euler 6: {}, Time Taken: {}", difference, start.elapsed().as_secs());
}

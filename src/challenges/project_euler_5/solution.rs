use std::time::Instant;

/**
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */
pub fn project_euler_5() {
    // Probably a faster way to do this one

    let start = Instant::now();

    let mut value: i32 = 0;

    loop {
        value += 1;

        let mut divisable = true;

        for i in 2..20 {
            if value % i != 0 {
                divisable = false;
                break;
            }
        }

        if divisable {
            println!("Project Euler 5: {}, Time Taken: {}", value, start.elapsed().as_secs());
            break;
        }
    }
}

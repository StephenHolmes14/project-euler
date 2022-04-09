use std::time::Instant;

/**
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a2 + b2 = c2
For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
 */
pub fn project_euler_9() {
    let start = Instant::now();
    let mut answer = 0;


    for c in 2..998 {
        let c_squared = c*c;

        for b in 2..c {
            let a = 1000 - c - b;

            if a*a + b*b == c_squared {
                answer = a * b * c;
                break
            }
        }

        if answer != 0 {
            break;
        }
    }

    println!("Project Euler 9: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}

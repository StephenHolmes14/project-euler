use std::time::Instant;

pub fn project_euler_1() {
    let start = Instant::now();
    let mut sum = 0;
    for i in 3..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    println!("Project Euler 1: {}, Time Taken: {}", sum, start.elapsed().as_secs());
}
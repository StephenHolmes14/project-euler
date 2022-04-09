use std::time::Instant;

/**
Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.


How many such routes are there through a 20×20 grid?
 */
pub fn project_euler_15() {
    let start = Instant::now();

    let mut state = [[0u64; 21]; 21];
    let mut answer = recursive_check(20, 20, &mut state);

    // Maybe think of it like a tree? Count the branches? Might be horrible recursion here though

    println!("Project Euler 15: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}


fn recursive_check(position_x: usize, position_y: usize, current_state: &mut [[u64; 21]; 21]) -> u64 {
    if (position_x == 1 && position_y == 0) || (position_x == 0 && position_y == 1) {
        return 1;
    }

    if current_state[position_x][position_y] != 0 {
        return current_state[position_x][position_y];
    }

    let can_go_down = position_y > 0;
    let can_go_right = position_x > 0;
    let mut total = 0;

    if can_go_down {
        total += recursive_check(position_x, position_y-1, current_state);
    }

    if can_go_right {
        total += recursive_check(position_x-1, position_y, current_state);
    }

    current_state[position_x][position_y] = total;

    return total;
}

use std::time::Instant;

/**
By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.

3
7 4
2 4 6
8 5 9 3

That is, 3 + 7 + 4 + 9 = 23.

Find the maximum total from top to bottom of the triangle below:

75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

NOTE: As there are only 16384 routes, it is possible to solve this problem by trying every route. However, Problem 67, is the same challenge with a triangle containing one-hundred rows; it cannot be solved by brute force, and requires a clever method! ;o)
 */
pub fn project_euler_18() {
    let start = Instant::now();
    let mut answer = 0;

    let grid_string = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

    let grid: Vec<Vec<i32>> = grid_string.split("\n")
        .map(|r| r.split(" ").map(|x| i32::from_str_radix(x, 10).unwrap()).collect())
        .collect();

    // This grid is inverted, starting from the second last row
    let mut largest_path_grid: Vec<Vec<i32>> = vec![vec![0; grid.last().unwrap().len()]; grid.len()];

    largest_path_grid[grid.len()-1] = grid.last().unwrap().clone();

    // From second last row going up the tree
    for i in (0..(grid.len() - 1)).rev() {
        for j in 0..grid.get(i).unwrap().len() {
            let left_path = grid.get(i).unwrap().get(j).unwrap() + largest_path_grid.get(i + 1).unwrap().get(j).unwrap();
            let right_path = grid.get(i).unwrap().get(j).unwrap() + largest_path_grid.get(i + 1).unwrap().get(j + 1).unwrap();

            let largest_path = if left_path > right_path { left_path } else { right_path };
            largest_path_grid[i][j] = largest_path;
        }
    }

    answer = largest_path_grid[0][0];

    println!("Project Euler 18: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}
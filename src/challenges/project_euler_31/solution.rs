use std::borrow::Borrow;
use std::time::Instant;


/**
In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins in general circulation:

1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
It is possible to make £2 in the following way:

1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
How many different ways can £2 be made using any number of coins?
 */
pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    let coins = vec![1, 2, 5, 10, 20, 50, 100, 200];

    answer = find_number_of_combinations(200, 200, coins);

    println!("Project Euler 30: {}, Time Taken: {}", answer, start.elapsed().as_secs());
}


// Basically this algo is:
// For a given amount and a set of coins
// If the amount is 0 it's found a combination, 1 is returned to signify "1 found"
// If the amount isn't 0, the coin array is filtered to find the coins which are both larger than the previous coin and larger than the current amount
// The coin needs to be the same size or larger to prevent counting all permutations, we only want combinations
// The coin needs to be larger or equal to ensure we can deduct the amount
// This method is then called recursively for each coin, with the amount decremented by the coin
// The total of all the coins methods calls is the result
pub fn find_number_of_combinations(previous_coin: i32, amount: i32, coins: Vec<i32>) -> i32 {
    if amount == 0 {
        return 1;
    }

    let possible_coins: Vec<i32> = coins.into_iter()
        .filter(|c| amount >= *c && previous_coin >= *c)
        .collect();

    possible_coins.iter()
        .map(|c| find_number_of_combinations(*c, amount - *c, possible_coins.clone()))
        .sum()
}

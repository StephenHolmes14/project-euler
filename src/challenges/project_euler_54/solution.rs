use std::ops::{Add, Div, Sub};
use std::time::Instant;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;

/// https://projecteuler.net/problem=54
///
/// There are exactly ten ways of selecting three from five, 12345:
//
// In the Card game poker, a hand consists of five cards and are ranked, from lowest to highest, in the following way:
//
// High Card: Highest value Card.
// One Pair: Two cards of the same value.
// Two Pairs: Two different pairs.
// Three of a Kind: Three cards of the same value.
// Straight: All cards are consecutive values.
// Flush: All cards of the same suit.
// Full House: Three of a kind and a pair.
// Four of a Kind: Four cards of the same value.
// Straight Flush: All cards are consecutive values of same suit.
// Royal Flush: Ten, Jack, Queen, King, Ace, in same suit.
// The cards are valued in the order:
// 2, 3, 4, 5, 6, 7, 8, 9, 10, Jack, Queen, King, Ace.
//
// If two players have the same ranked hands then the rank made up of the highest value wins; for example, a pair of eights beats a pair of fives (see example 1 below). But if two ranks tie, for example, both players have a pair of queens, then highest cards in each hand are compared (see example 4 below); if the highest cards tie then the next highest cards are compared, and so on.
//
// Consider the following five hands dealt to two players:
//
// Hand	 	Player 1	 	Player 2	 	Winner
// 1	 	5H 5C 6S 7S KD
// Pair of Fives
//  	2C 3S 8S 8D TD
// Pair of Eights
//  	Player 2
// 2	 	5D 8C 9S JS AC
// Highest Card Ace
//  	2C 5C 7D 8S QH
// Highest Card Queen
//  	Player 1
// 3	 	2D 9C AS AH AC
// Three Aces
//  	3D 6D 7D TD QD
// Flush with Diamonds
//  	Player 2
// 4	 	4D 6S 9H QH QC
// Pair of Queens
// Highest Card Nine
//  	3D 6D 7H QD QS
// Pair of Queens
// Highest Card Seven
//  	Player 1
// 5	 	2H 2D 4C 4D 4S
// Full House
// With Three Fours
//  	3C 3D 3S 9S 9D
// Full House
// with Three Threes
//  	Player 1
// The file, poker.txt, contains one-thousand random hands dealt to two players. Each line of the file contains ten cards (separated by a single space): the first five are Player 1's cards and the last five are Player 2's cards. You can assume that all hands are valid (no invalid characters or repeated cards), each player's hand is in no specific order, and in each hand there is a clear winner.
//
// How many hands does Player 1 win?
//
///

pub fn calculate() {
    let start = Instant::now();
    let mut answer = 0;

    answer = find_answer();

    println!("Project Euler 54: {}, Time Taken: {}ms", answer, start.elapsed().as_millis());
}

struct PokerGame {
    player1: PokerHand,
    player2: PokerHand,
    cards: String
}

struct PokerHand {
    best_hand_value: i32, // Lower is better
    high_card_value: i32, // Higher is better, this is the high card for
    second_high_card_value: i32,
    cards: Vec<PokerCard>
}

struct PokerCard {
    suit: i32,
    value: u32
}

struct MatchedCards {
    count: i32,
    value: u32
}

pub fn find_answer() -> u64 {
    let all_hands_string = fs::read_to_string("src/challenges/project_euler_54/poker.txt").unwrap();
    let games_won = all_hands_string.split("\n")
        .map(|h| make_game_from_string(String::from(h)))
        .filter(|g| does_player1_win(&g))
        .count();

    games_won as u64
}

fn does_player1_win(game: &PokerGame) -> bool {
    if game.player1.best_hand_value < game.player2.best_hand_value {
        println!("PLAYER 2 WINS");
        return false;
    }

    if game.player1.best_hand_value > game.player2.best_hand_value {
        println!("PLAYER 1 WINS");
        return true;
    }

    if game.player1.high_card_value < game.player2.high_card_value {
        println!("PLAYER 2 WINS");
        return false;
    }

    if game.player1.high_card_value > game.player2.high_card_value {
        println!("PLAYER 1 WINS");
        return true;
    }

    if game.player1.second_high_card_value < game.player2.second_high_card_value {
        println!("PLAYER 2 WINS");
        return false;
    }

    if game.player1.second_high_card_value > game.player2.second_high_card_value {
        println!("PLAYER 1 WINS");
        return true;
    }

    // If we reach here the hands are equal and we need to check the high cards

    println!("SAME SCORE!! - {}", game.cards);

    for i in 0..5 {
        let player1_card = game.player1.cards.get(i).unwrap().value;
        let player2_card = game.player1.cards.get(i).unwrap().value;

        if player1_card > player2_card {
            println!("PLAYER 1 WINS");
            return true;
        } else if player2_card > player1_card {
            println!("PLAYER 2 WINS");
            return false;
        }

        // otherwise next card
    }
    // Shouldn't happen but if he reaches here he loses lol
    false
}

fn make_game_from_string(game_string: String) -> PokerGame {
    println!("---");
    println!("{}", game_string);

    let mut iter = game_string.split(' ');

    let mut player1_cards: Vec<PokerCard> = (0..5)
        .map(|_| make_card_from_string(String::from(iter.next().unwrap())))
        .collect();

    player1_cards.sort_by(|a, b| b.value.cmp(&a.value));

    let mut player2_cards: Vec<PokerCard> = (0..5)
        .map(|_| make_card_from_string(String::from(iter.next().unwrap())))
        .collect();

    // Revered order sort
    player2_cards.sort_by(|a, b| b.value.cmp(&a.value));

    let (player1_best_hand, player1_high_card, player1_second_high_card) = check_players_best_hand(&player1_cards);
    let (player2_best_hand, player2_high_card, player2_second_high_card) = check_players_best_hand(&player2_cards);

    println!("---");
    PokerGame {
        player1: PokerHand {
            cards: player1_cards,
            best_hand_value: player1_best_hand,
            high_card_value: player1_high_card,
            second_high_card_value: player1_second_high_card
        },
        player2: PokerHand {
            cards: player2_cards,
            best_hand_value: player2_best_hand,
            high_card_value: player2_high_card,
            second_high_card_value: player2_second_high_card
        },
        cards: game_string,
    }
}

fn make_card_from_string(card_string: String) -> PokerCard {
    let chars_vec: Vec<char> = card_string.chars().collect();
    let suit_char = chars_vec.get(1).unwrap();
    let value_char = chars_vec.get(0).unwrap();

    // Suits in alphabetical - C D H S
    let suit = match suit_char {
        'C' => 0,
        'D' => 1,
        'H' => 2,
        _ => 3 // Spades, we don't expect any errors in data
    };

    // Just to make future maths easier, I'm sticking with 0 - 12 as the numbers
    // To make it complicated lol, I'm treating A as high. So 0 is actually 2, 1 is 3, 2 is 4 etc etc
    // Where 0 is A (or 1)
    let value = match value_char {
        'A' => 12, // (1 - 2) % 13
        'K' => 11, // 13 - 2
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        number => number.clone() as u32 - 50 // ASCII hack but also - 2 like the others
    };

    PokerCard {
        suit,
        value
    }
}

// Returns (Score, HighCard, SecondHighCard)
fn check_players_best_hand(cards: &Vec<PokerCard>) -> (i32, i32, i32) {
    // Somewhere between 0 and 12
    let high_card_value = cards.get(0).unwrap().value;
    let second_high_card_value = cards.get(1).unwrap().value;

    // Royal Flush
    if check_straight_flush(cards, 12) { // Flush starting from A
        println!("FOUND ROYAL FLUSH");
        return (21, 12, 0);
    }

    // Straight Flush
    if check_straight_flush(cards, high_card_value) {
        println!("FOUND STRAIGHT FLUSH");
        return (20, high_card_value as i32, 0);
    }

    // Four of a Kind 19 - Will be checked as part of end checks
    // Full House 18 - Will be checked as part of end checks

    // Flush
    if check_flush(cards) {
        println!("FOUND FLUSH");
        return (17, high_card_value as i32, 0);
    }

    // Straight
    if check_straight(cards, high_card_value) {
        println!("FOUND STRAIGHT");
        return (16, high_card_value as i32, 0);
    }

    // Three of a Kind 15
    // Two Pair 14
    // One Pair 13
    let (matches, matched_high_card, matched_second_high_card) = count_matches(cards);
    if matches != 0 {
        println!("MATCHED {} {} {}", matches, matched_high_card, matched_second_high_card);
        return (matches, matched_high_card, matched_second_high_card);
    }

    // Nothing found, returning high card value
    println!("FOUND HIGH CARD: {} {}", high_card_value, second_high_card_value);
    (high_card_value as i32, second_high_card_value as i32, 0)
}

// This assumes card vec is reverse ordered
fn check_straight_flush(cards: &Vec<PokerCard>, start_digit: u32) -> bool {
    if !check_flush(cards) {
        return false;
    }

    if !check_straight(cards, start_digit) {
        return false
    }

    true
}

fn check_flush(cards: &Vec<PokerCard>) -> bool {
    let suit = cards.get(0).unwrap().suit;
    for card in cards {
        if card.suit != suit {
            return false;
        }
    }

    true
}

fn check_straight(cards: &Vec<PokerCard>, start_digit: u32) -> bool {
    let last_card = cards.last().unwrap();
    if last_card.value > 8 { // If the last number is 10 (8 because of -2), we can't have a flush
        return false
    }

    let mut current_value = start_digit;
    for card in cards {
        if card.value != current_value {
            return false;
        }

        // If we reached 0 the only valid straight is if the next (and last value) is A
        if current_value == 0 {
            current_value = 13
        }

        current_value = current_value - 1;
    }

    true
}

fn count_matches(cards: &Vec<PokerCard>) -> (i32, i32, i32){
    let mut previous = 13; // Not a real number used
    let mut current_count = 1;

    let mut matched_cards = Vec::new();

    for card in cards {
        if previous == card.value {
            current_count = current_count + 1;
            continue;
        }

        if current_count > 1 {
            matched_cards.push(MatchedCards{
                value: previous,
                count: current_count
            });

            current_count = 1
        }

        previous = card.value;
    }

    if current_count > 1 {
        matched_cards.push(MatchedCards{
            value: previous,
            count: current_count
        });
    }

    // Highest Number Matched
    // If it's a two pair sort by value desc
    if matched_cards.iter().all(|x| x.count == 2) {
        matched_cards.sort_by(|a, b| b.value.cmp(&a.value) );
    } else {
        // Otherwise sort by highest count desc
        matched_cards.sort_by(|a, b| b.count.cmp(&a.count) );
    }

    let highest_card = if matched_cards.len() < 1 {0} else {matched_cards.get(0).unwrap().value as i32};
    let second_highest_card = if matched_cards.len() <= 1 {0} else {matched_cards.get(1).unwrap().value as i32};

    // Four of a Kind
    if matched_cards.iter().any(|x| x.count == 4) {
        println!("FOUND FOUR OF A KIND");
        return (19, highest_card, second_highest_card);
    }

    // Full House
    if matched_cards.iter().map(|x| x.count).sum::<i32>() == 5 {
        println!("FOUND FULL HOUSE");
        return (18, highest_card, second_highest_card);
    }

    // Three of a Kind
    if matched_cards.iter().any(|x| x.count == 3) {
        println!("FOUND THREE OF A KIND");
        return (15, highest_card, second_highest_card);
    }

    // Two Pair
    if matched_cards.len() == 2 && matched_cards.iter().all(|x| x.count == 2) {
        let values: Vec<u32> = matched_cards.iter().map(|x| x.value).collect();
        println!("FOUND TWO PAIR");
        return (14, highest_card, second_highest_card);
    }

    if matched_cards.iter().any(|x| x.count == 2) {
        println!("FOUND PAIR");
        return (13, highest_card, second_highest_card)
    }

    // No matches
    (0, 0, 0)
}
use std::collections::HashMap;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day7/input.txt").unwrap();

    utilities::print_results(7, || part_1(&contents), || part_2(&contents));
}

// Sort the hands by type, breaking ties by card ranks.
fn part_1(data: &str) -> usize {
    let mut hands = parse_hands(data);

    hands.sort_by(|a, b| a.partial_cmp(&b).unwrap());

    let total_score = hands
        .iter()
        .enumerate()
        .fold(0, |count, (i, hand)| count + (i + 1) * hand.bid);

    total_score
}

#[derive(Debug)]
struct Hand {
    hand: String,
    bid: usize,
    hand_type: usize,
}

impl Hand {
    fn new(hand: &str, bid: usize) -> Self {
        let hand_type = Hand::get_hand_type(hand);

        Self {
            hand: hand.to_string(),
            bid,
            hand_type,
        }
    }

    fn get_hand_type(hand: &str) -> usize {
        let mut cards: HashMap<char, usize> = HashMap::new();

        for c in hand.chars() {
            cards.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        // Types of hands:
        // Five of a kind: 7
        // Four of a kind: 6
        // Full house: 5
        // Three of a kind: 4
        // Two pair: 3
        // One pair: 2
        // High card: 1

        let max_count = cards.values().max().unwrap();

        let hand_type = match cards.keys().len() {
            1 => {
                // Five of a kind
                7
            }
            2 => {
                // Four of a kind or a Full house
                if max_count == &4 {
                    // Four of a kind
                    6
                } else {
                    // Full house
                    5
                }
            }
            3 => {
                // Three of a kind or Two pair
                if max_count == &3 {
                    // Three of a kind
                    4
                } else {
                    // Two pair
                    3
                }
            }
            4 => {
                // One pair
                2
            }
            5 => {
                // High card
                1
            }
            _ => panic!("Illegal hand size"),
        };

        hand_type
    }

    fn get_card_rank(card: char) -> usize {
        match card {
            '2'..='9' => card.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Illegal card!"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        // Compare cards individually.
        for (a, b) in self.hand.chars().zip(other.hand.chars()) {
            match Hand::get_card_rank(a).partial_cmp(&Hand::get_card_rank(b)) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }

        Some(core::cmp::Ordering::Equal)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand && self.hand_type == other.hand_type
    }
}

fn parse_hands(data: &str) -> Vec<Hand> {
    let mut hands = vec![];

    for line in data.lines() {
        let mut pieces_iter = line.split_ascii_whitespace();

        let hand = pieces_iter.next().unwrap();
        let bid = pieces_iter.next().unwrap().parse::<usize>().unwrap();

        hands.push(Hand::new(hand, bid));
    }

    hands
}

fn part_2(data: &str) -> usize {
    0
}

use std::collections::HashMap;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day7/input.txt").unwrap();

    utilities::print_results(7, || part_1(&contents), || part_2(&contents));
}

// Sort the hands by type, breaking ties by card ranks.
fn part_1(data: &str) -> usize {
    let mut hands = parse_hands(data, false);

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
    fn new(hand: &str, bid: usize, j_as_joker: bool) -> Self {
        let hand_type = if j_as_joker {
            Hand::get_hand_type_v2(hand)
        } else {
            Hand::get_hand_type(hand)
        };

        Self {
            hand: hand.to_string(),
            bid,
            hand_type,
        }
    }

    fn get_hand_type(hand: &str) -> usize {
        let cards = Hand::get_card_counts(hand);

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

    fn get_card_counts(hand: &str) -> HashMap<char, usize> {
        let mut cards: HashMap<char, usize> = HashMap::new();

        for c in hand.chars() {
            cards.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        cards
    }

    fn get_hand_type_v2(hand: &str) -> usize {
        let cards = Hand::get_card_counts(hand);

        // The presence of Jokers can upgrade the hand type by acting as
        // other cards.

        let joker_count = cards.get(&'J').unwrap_or(&0);

        let mut other_cards: HashMap<char, usize> = HashMap::new();

        for (card, count) in &cards {
            if card != &'J' {
                other_cards.insert(*card, *count);
            }
        }

        let other_max = other_cards.values().max().unwrap_or(&0);
        let other_min = other_cards.values().min().unwrap_or(&0);

        let hand_type = match joker_count {
            5 => {
                // Upgrade to five of a kind
                7
            }
            4 => {
                // Upgrade to five of a kind
                7
            }
            3 => {
                // The other cards are either a one pair or singles
                if other_max == &2 {
                    // Upgrade to five of a kind
                    7
                } else {
                    // Upgrade to four of a kind
                    6
                }
            }
            2 => {
                // The other cards are either three of a kind, one pair, or singles
                if other_max == &3 {
                    // Upgrade to five of a kind
                    7
                } else if other_max == &2 {
                    // Upgrade to four of a kind
                    6
                } else {
                    // Upgrade to three of a kind
                    4
                }
            }
            1 => {
                // The other cards can be a four of a kind, a three of a kind, two pair,
                // one pair, or singles.
                if other_max == &4 {
                    // Upgrade to five of a kind
                    7
                } else if other_max == &3 {
                    // Upgrade to four of a kind
                    6
                } else if other_max == &2 {
                    // Two pair or one pair
                    if other_min == &2 {
                        // Two pair, upgrade to full house
                        5
                    } else {
                        // One pair, upgrade to three of a kind
                        4
                    }
                } else {
                    // Singles, upgrade to one pair
                    2
                }
            }
            0 => {
                Hand::get_hand_type(hand)
            }
            _ => panic!("Illegal Joker count")
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

    fn get_card_rank_v2(card: char) -> usize {
        // In this case, J = Joker and is the lowest individual card.
        match card {
            '2'..='9' => card.to_digit(10).unwrap() as usize,
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Illegal card!"),
        }
    }

    fn partial_cmp_v2(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Additional compare method that takes into account Jokers.
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }

        // Compare cards individually.
        for (a, b) in self.hand.chars().zip(other.hand.chars()) {
            match Hand::get_card_rank_v2(a).partial_cmp(&Hand::get_card_rank_v2(b)) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
        }

        Some(core::cmp::Ordering::Equal)
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

fn parse_hands(data: &str, j_as_joker: bool) -> Vec<Hand> {
    let mut hands = vec![];

    for line in data.lines() {
        let mut pieces_iter = line.split_ascii_whitespace();

        let hand = pieces_iter.next().unwrap();
        let bid = pieces_iter.next().unwrap().parse::<usize>().unwrap();

        hands.push(Hand::new(hand, bid, j_as_joker));
    }

    hands
}

// J is now a Joker, which can act as any other card to make the hand have a stronger type. For
// matching hand types, it acts as the weakest card when comparing.
fn part_2(data: &str) -> usize {
    let mut hands = parse_hands(data, true);

    hands.sort_by(|a, b| a.partial_cmp_v2(&b).unwrap());

    let total_score = hands
        .iter()
        .enumerate()
        .fold(0, |count, (i, hand)| count + (i + 1) * hand.bid);

    total_score
}

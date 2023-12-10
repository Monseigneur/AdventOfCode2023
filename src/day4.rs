use std::{collections::HashMap, collections::HashSet, fs};

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day4/input.txt").unwrap();

    utilities::print_results(4, || part_1(&contents), || part_2(&contents));
}

fn part_1(data: &str) -> u32 {
    // line: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    let mut points: u32 = 0;

    let matching_numbers = find_matching_numbers(data);

    for (_, num_matching) in matching_numbers {
        if num_matching != 0 {
            points += 2_u32.pow(num_matching - 1) as u32;
        }
    }

    points
}

fn parse_numbers(data: &str) -> HashSet<u32> {
    let numbers: Vec<&str> = data.trim().split(" ").collect();

    let mut results: HashSet<u32> = HashSet::new();

    numbers.iter().for_each(|n| {
        if let Ok(i) = n.parse::<u32>() {
            results.insert(i);
        }
    });

    results
}

fn find_matching_numbers(data: &str) -> HashMap<u32, u32> {
    let mut matching_numbers: HashMap<u32, u32> = HashMap::new();

    for line in data.lines() {
        let card_info: Vec<&str> = line.split("|").collect();

        let first_piece: Vec<&str> = card_info[0].split(":").collect();

        // println!("first_piece {:?}", first_piece);

        let card_title: Vec<&str> = first_piece[0]
            .split(" ")
            .filter(|s| !s.is_empty())
            .collect();
        // println!("  card_title {:?}", card_title);
        let card_number: u32 = card_title[1].trim().parse().unwrap();

        let winning_numbers = parse_numbers(first_piece[1]);
        let card_numbers = parse_numbers(card_info[1]);

        let intersection_size = winning_numbers
            .intersection(&card_numbers)
            .collect::<Vec<&u32>>()
            .len() as u32;

        matching_numbers.insert(card_number, intersection_size);
    }

    matching_numbers
}

fn part_2(data: &str) -> u32 {
    let matching_numbers = find_matching_numbers(data);

    // 1->4, 2->2, 3->2, 4->1, 5->0, 6->0
    // println!("matching_numbers {:?}", matching_numbers);

    // println!("num cards {}", matching_numbers.len());

    let mut card_counts = CardCounts::new(matching_numbers.len());

    for i in (1..=matching_numbers.len()).rev() {
        let val = matching_numbers.get(&(i as u32));

        assert!(val.is_some());

        let num_matches = val.unwrap();

        card_counts.calc_cards(i, *num_matches);
    }

    // println!("card_counts {:?}", card_counts);

    let mut total = 0;
    for i in 1..=matching_numbers.len() {
        total += card_counts.get_count(i);
    }

    // 9903606 is wrong for part 2
    // 9924412
    total
}

struct CardCounts {
    counts: Vec<u32>,
}

impl CardCounts {
    fn new(num_cards: usize) -> Self {
        Self {
            counts: vec![0; num_cards],
        }
    }

    fn get_count(&self, card_id: usize) -> u32 {
        assert!(card_id <= self.counts.len());

        self.counts[card_id - 1]
    }

    fn calc_cards(&mut self, card_id: usize, num_matches: u32) {
        assert!(card_id <= self.counts.len());

        // If num_matches is 0, only win this card.
        // Else, win the next num_matches cards.

        let mut count = 1;

        if num_matches != 0 {
            for i in 0..num_matches {
                count += self.counts[card_id + i as usize];
            }
        }

        self.counts[card_id - 1] = count;
    }
}

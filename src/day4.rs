use std::{fs, collections::HashSet};

pub fn run() {
    let contents = fs::read_to_string("test_files/day4/input.txt").unwrap();

    let part_1_result = part_1(&contents);
    let part_2_result = part_2(&contents);

    println!("[Day 4]: part 1: {part_1_result}, part 2: {part_2_result}");
}

fn part_1(data: &str) -> u32 {
    // line: Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

    let mut points: u32 = 0;

    for line in data.lines() {
        let card_info: Vec<&str> = line.split("|").collect();

        let winning_numbers = card_info[0].split(":").collect::<Vec<&str>>()[1];

        let winning_numbers = parse_numbers(winning_numbers);
        let card_numbers = parse_numbers(card_info[1]);

        let intersection_size = winning_numbers.intersection(&card_numbers).collect::<Vec<&u32>>().len() as u32;

        if intersection_size != 0 {
            points += 2_u32.pow(intersection_size - 1) as u32;
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

fn part_2(data: &str) -> u32 {
    1
}
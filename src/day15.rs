use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day15/input.txt").unwrap();

    utilities::print_results(15, || part_1(&contents), || part_2(&contents));
}

// Hash each string in the comma-separated list of tokens.
fn part_1(data: &str) -> usize {
    let mut current = 0;

    for s in data.split(",") {
        current += hash_str(s);
    }

    current
}

fn hash_str(s: &str) -> usize {
    let mut current = 0;

    for c in s.chars() {
        let code = c as usize;

        current = (current + code) * 17 % 256;
    }

    current
}

fn part_2(data: &str) -> usize {
    0
}

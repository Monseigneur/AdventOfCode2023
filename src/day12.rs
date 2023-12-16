use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day12/example.txt").unwrap();

    utilities::print_results(12, || part_1(&contents), || part_2(&contents));
}

fn part_1(data: &str) -> usize {
    0
}

fn part_2(data: &str) -> usize {
    0
}

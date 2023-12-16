use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day13/input.txt").unwrap();

    utilities::print_results(13, || part_1(&contents), || part_2(&contents));
}

// For each pattern, find the column or row of reflection, and then sum the number of columns
// before and 100 * the number of rows before.
fn part_1(data: &str) -> usize {
    let mut sum = 0;

    let mut pattern_lines = vec![];

    for line in data.lines() {
        if line.is_empty() {
            sum += calculate_pattern_value(&pattern_lines);

            pattern_lines.clear();

            continue;
        }

        pattern_lines.push(line);
    }

    if !pattern_lines.is_empty() {
        sum += calculate_pattern_value(&pattern_lines);
    }

    sum
}

fn calculate_pattern_value(data: &Vec<&str>) -> usize {
    let data = data
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    for col in 0..(data[0].len() - 1) {
        if !check_columns(&data, col) {
            continue;
        }

        return col + 1;
    }

    for row in 0..(data.len() - 1) {
        if !check_rows(&data, row) {
            continue;
        }

        return 100 * (row + 1);
    }

    panic!("No line of symmetry found!");
}

fn check_columns(data: &Vec<Vec<char>>, col: usize) -> bool {
    let before_dist = col + 1;
    let after_dist = data[0].len() - col - 1;
    let elements_to_match = before_dist.min(after_dist);

    for i in 0..elements_to_match {
        for row in data.iter() {
            if row[col - i] != row[col + 1 + i] {
                return false;
            }
        }
    }

    true
}

fn check_rows(data: &Vec<Vec<char>>, row: usize) -> bool {
    let before_dist = row + 1;
    let after_dist = data.len() - row - 1;
    let elements_to_match = before_dist.min(after_dist);

    for i in 0..elements_to_match {
        for col in 0..data[0].len() {
            if data[row - i][col] != data[row + 1 + i][col] {
                return false;
            }
        }
    }

    true
}

fn part_2(data: &str) -> usize {
    0
}

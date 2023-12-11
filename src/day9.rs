use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day9/input.txt").unwrap();

    utilities::print_results(8, || part_1(&contents), || part_2(&contents));
}

// The pattern seems like Pascal's triangle. It also seems like simple derivatives.
fn part_1(data: &str) -> isize {
    data.lines().map(|line| process_line(line)).sum()
}

fn process_line(line: &str) -> isize {
    let mut line_numbers = convert_line(line);

    assert!(!line_numbers.is_empty());

    let mut next_val = *line_numbers.iter().last().unwrap();

    while !is_all_zero(&line_numbers) {
        let new_line_numbers = calc_line_delta(&line_numbers);

        next_val += new_line_numbers.iter().last().unwrap_or(&0);

        line_numbers = new_line_numbers;
    }

    next_val
}

type LineData = Vec<isize>;

fn convert_line(line: &str) -> LineData {
    line.split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect()
}

fn is_all_zero(line: &LineData) -> bool {
    line.iter().find(|val| **val != 0 as isize).is_none()
}

fn calc_line_delta(line: &LineData) -> LineData {
    line.iter()
        .zip(line.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<LineData>()
}

// This is the same, but in reverse, extrapolating the value before the first value.
fn part_2(data: &str) -> isize {
    data.lines().map(|line| process_line_v2(line)).sum()
}

fn process_line_v2(line: &str) -> isize {
    let mut line_numbers = convert_line(line);

    assert!(!line_numbers.is_empty());

    // Need to walk backwards in the list of first elements, so have to keep
    // track of them all at first.
    let mut first_vals = vec![];

    first_vals.push(*line_numbers.iter().next().unwrap());

    while !is_all_zero(&line_numbers) {
        let new_line_numbers = calc_line_delta(&line_numbers);

        first_vals.push(*new_line_numbers.iter().next().unwrap_or(&0));

        line_numbers = new_line_numbers;
    }

    first_vals.iter().rfold(0, |acc, val| val - acc)
}

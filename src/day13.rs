use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day13/input.txt").unwrap();

    utilities::print_results(13, || part_1(&contents), || part_2(&contents));
}

// For each pattern, find the column or row of reflection, and then sum the number of columns
// before and 100 * the number of rows before.
fn part_1(data: &str) -> usize {
    process_data(data, false)
}

fn process_data(data: &str, find_smudge: bool) -> usize {
    let mut sum = 0;

    let mut pattern_lines = vec![];

    for line in data.lines() {
        if line.is_empty() {
            sum += calculate_pattern_value(&pattern_lines, find_smudge);

            pattern_lines.clear();

            continue;
        }

        pattern_lines.push(line);
    }

    if !pattern_lines.is_empty() {
        sum += calculate_pattern_value(&pattern_lines, find_smudge);
    }

    sum
}

fn calculate_pattern_value(data: &Vec<&str>, find_smudge: bool) -> usize {
    let data = data
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut score = None;

    for col in 0..(data[0].len() - 1) {
        let (found, delta) = check_columns(&data, col, find_smudge);

        if !find_smudge {
            if !found {
                continue;
            }
        } else if delta != 1 {
            continue;
        }

        score = Some(col + 1);

        break;
    }

    for row in 0..(data.len() - 1) {
        let (found, delta) = check_rows(&data, row, find_smudge);

        if !find_smudge {
            if !found {
                continue;
            }
        } else if delta != 1 {
            continue;
        }

        score = Some(100 * (row + 1));

        break;
    }

    assert!(score.is_some());

    score.unwrap()
}

fn check_columns(data: &Vec<Vec<char>>, col: usize, exhaustive: bool) -> (bool, usize) {
    let before_dist = col + 1;
    let after_dist = data[0].len() - col - 1;
    let elements_to_match = before_dist.min(after_dist);

    let mut result = true;
    let mut delta = 0;

    'outer: for i in 0..elements_to_match {
        for row in 0..data.len() {
            if data[row][col - i] != data[row][col + 1 + i] {
                result = false;
                delta += 1;

                if !exhaustive {
                    break 'outer;
                }
            }
        }
    }

    (result, delta)
}

fn check_rows(data: &Vec<Vec<char>>, row: usize, exhaustive: bool) -> (bool, usize) {
    let before_dist = row + 1;
    let after_dist = data.len() - row - 1;
    let elements_to_match = before_dist.min(after_dist);

    let mut result = true;
    let mut delta = 0;

    'outer: for i in 0..elements_to_match {
        for col in 0..data[0].len() {
            if data[row - i][col] != data[row + 1 + i][col] {
                result = false;
                delta += 1;

                if !exhaustive {
                    break 'outer;
                }
            }
        }
    }

    (result, delta)
}

// For each pattern, one of the marks is incorrect and swapping it leads to the line of symmetry
// changing. Calculate the new score based on changing the one mark.
fn part_2(data: &str) -> usize {
    process_data(data, true)
}

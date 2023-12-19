use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day12/input.txt").unwrap();

    utilities::print_results(12, || part_1(&contents), || part_2(&contents));
}

// Given a bunch of lines of data about springs, where each line contains the spring arrangement with
// each spring either operation (.), damaged (#), or unknown (?), and information about the contiguous
// groups of damaged springs, calculate the number of possible arrangements.
fn part_1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

            process_spring_data(pieces[0], pieces[1])
        })
        .sum()
}

// Notes:
// - Perhaps counting springs is useful, at least for an upper bound?
//      - damaged_info.sum() - damaged_count = max damaged springs in Y '?' tiles
//

fn process_spring_data(spring_info: &str, damaged_info: &str) -> usize {
    let (broken_springs, total_broken) = get_broken_springs(damaged_info);

    let unknown_count = spring_info.chars().filter(|&c| c == '?').count();
    let known_broken = spring_info.chars().filter(|&c| c == '#').count();

    let broken_in_unknown = total_broken - known_broken;

    let mut unknown_indexes = vec![];

    for (i, c) in spring_info.char_indices() {
        if c == '?' {
            unknown_indexes.push(i);
        }
    }

    let mut total = 0;
    let mut spring = spring_info.chars().collect::<Vec<char>>();

    // There are unknown_count choose broken_in_unknown ways to fill in the broken springs. Try them all.
    let max = (1 << unknown_count) - 1;

    'outer: for i in 0..=max {
        let mut count = 0;
        let mut index = 0;

        let mut val = i;

        for _ in 0..unknown_count {
            spring[unknown_indexes[index]] = if val % 2 != 0 {
                count += 1;

                if count > broken_in_unknown {
                    continue 'outer;
                }

                '#'
            } else {
                '.'
            };

            index += 1;

            val = val >> 1;
        }

        let s = spring.iter().collect::<String>();

        if check_pattern(&s, &broken_springs) {
            total += 1;
        }
    }

    total
}

fn get_broken_springs(damaged_info: &str) -> (Vec<usize>, usize) {
    let broken_springs = damaged_info
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let sum = broken_springs.iter().sum();

    (broken_springs, sum)
}

fn check_pattern(pattern: &str, broken_springs: &Vec<usize>) -> bool {
    let pattern_vec = pattern
        .split(".")
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect::<Vec<usize>>();

    &pattern_vec == broken_springs
}

fn part_2(data: &str) -> usize {
    0
}

use utilities;

pub fn run() {
    utilities::run_puzzle(12, true, part_1, part_2);
}

// Given a bunch of lines of data about springs, where each line contains the spring arrangement with
// each spring either operation (.), damaged (#), or unknown (?), and information about the contiguous
// groups of damaged springs, calculate the number of possible arrangements.
fn part_1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

            process_spring_data_v2(pieces[0], pieces[1])
        })
        .sum()
}

// The patterns are repeated 5 times each, with a '?' separating the spring parts, and a ',' separating the broken
// spring runs.
fn part_2(data: &str) -> usize {
    let mut count = 0;

    for line in data.lines() {
        let pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

        let mut spring_info = pieces[0].to_owned();
        let mut damaged_info = pieces[1].to_owned();

        for _ in 0..4 {
            spring_info.push('?');
            spring_info.push_str(pieces[0]);
            damaged_info.push(',');
            damaged_info.push_str(pieces[1]);
        }

        count += process_spring_data_v2(&spring_info, &damaged_info);
    }

    count
}

fn process_spring_data_v2(spring_info: &str, damaged_info: &str) -> usize {
    let mut springs = ".".to_owned();
    springs.push_str(spring_info);
    springs.push_str(".");

    let mut damaged = vec![false];

    damaged_info.split(",").for_each(|s| {
        let val = s.parse::<usize>().unwrap();

        for _ in 0..val {
            damaged.push(true);
        }

        damaged.push(false);
    });

    let springs_len = springs.len();
    let damaged_len = damaged.len();

    let mut table: Vec<Vec<usize>> = vec![vec![0; damaged_len + 1]; springs_len + 1];
    table[springs_len][damaged_len] = 1;

    for i in (0..springs_len).rev() {
        for j in (0..damaged_len).rev() {
            let mut is_damaged = false;
            let mut is_operational = false;

            match springs.as_bytes()[i] {
                b'#' => is_damaged = true,
                b'.' => is_operational = true,
                b'?' => {
                    is_damaged = true;
                    is_operational = true;
                }
                _ => panic!("Illegal character"),
            }

            let mut sum = 0;
            if is_damaged && damaged[j] {
                sum += table[i + 1][j + 1];
            } else if is_operational && !damaged[j] {
                sum += table[i + 1][j + 1] + table[i + 1][j];
            }

            table[i][j] = sum;
        }
    }

    table[0][0]
}

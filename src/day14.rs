use std::collections::HashMap;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day14/input.txt").unwrap();

    utilities::print_results(14, || part_1(&contents), || part_2(&contents));
}

type Data = Vec<Vec<char>>;

// Given a map of round rocks (O), square rocks (#), and empty spaces (.), figure out the round rock
// distribution if the entire plane is tilted such that the round rocks roll north. Square rocks do not
// move, and will block round rocks. Calculate the weight on the northern supports by
// sum(num_rocks_in_row * row_dist_from_south).
fn part_1(data: &str) -> usize {
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Data>();

    let mut sum = 0;
    for col in 0..data[0].len() {
        sum += process_column(&data, col);
    }

    sum
}

fn process_column(data: &Data, col: usize) -> usize {
    let mut score = 0;

    // Track the updated position for each item in the column, and track the latest "wall" (where the
    // round rocks would stop).

    let length = data.len();
    let mut wall_index = length;

    for (row, row_data) in data.iter().enumerate() {
        let position_index = length - row - 1;

        // Depending on the type of tile (round rock, square rock, open ground), update the wall position
        // accordingly. The open ground doesn't take any space, so it will not change the wall position
        // when tilted. The rocks will adjust the wall position, either by moving it one position back for
        // the round rock, or fixing it to the current position for the square rock.
        match row_data[col] {
            'O' => {
                score += wall_index;
                wall_index -= 1;
            }
            '#' => wall_index = position_index,
            _ => (),
        };
    }

    score
}

// Instead of just tilting to the north, tilt in a cycle of north, west, south, east. After 1000000000 cycles
// calculate the weight on the north supports.
fn part_2(data: &str) -> usize {
    const ITERATIONS: usize = 1_000_000_000;

    let mut data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Data>();

    // It's likely that the rock arrangements stabilize after a while. Track the arrangements to find when
    // a pattern is repeated.
    let mut patterns: HashMap<String, usize> = HashMap::new();

    patterns.insert(build_string(&data), 0);

    let mut result_cycle_count = None;

    for i in 0..ITERATIONS {
        let cycle_count = i + 1;

        apply_cycle(&mut data);

        let new = build_string(&data);

        if patterns.contains_key(&new) {
            // The pattern is cycling, determine the index that would extrapolate to 1,000,000,000.
            let original_cycle_count = patterns.get(&new).unwrap();

            let delta = i - original_cycle_count;
            let offset = (ITERATIONS - original_cycle_count) % delta;

            result_cycle_count = Some(original_cycle_count + offset);

            break;
        }

        patterns.insert(new, cycle_count);
    }

    // From the resulting extrapolated arrangement, calculate the weight score for that pattern.
    assert!(result_cycle_count.is_some());

    for (str_data, index) in patterns {
        if index == result_cycle_count.unwrap() {
            return calculate_string_weight(&str_data, data.len(), data[0].len());
        }
    }

    panic!("No score found!");
}

fn build_string(data: &Data) -> String {
    data.iter()
        .map(|row| row.iter().collect::<String>())
        .fold(String::new(), |s, row| s + &row)
}

fn apply_cycle(data: &mut Data) {
    for col in 0..data[0].len() {
        tilt_column_north(data, col);
    }

    for row in 0..data.len() {
        tilt_row_west(data, row);
    }

    for col in 0..data[0].len() {
        tilt_column_south(data, col);
    }

    for row in 0..data.len() {
        tilt_row_east(data, row);
    }
}

fn tilt_column_north(data: &mut Data, col: usize) {
    let mut open_index = 0;

    for row in 0..data.len() {
        let position = row;

        match data[row][col] {
            'O' => {
                data[position][col] = '.';
                data[open_index][col] = 'O';

                open_index += 1;
            }
            '#' => open_index = position + 1,
            _ => (),
        };
    }
}

fn tilt_row_west(data: &mut Data, row: usize) {
    let mut open_index = 0;

    for col in 0..data[0].len() {
        match data[row][col] {
            'O' => {
                data[row][col] = '.';
                data[row][open_index] = 'O';

                open_index += 1;
            }
            '#' => open_index = col + 1,
            _ => (),
        };
    }
}

fn tilt_column_south(data: &mut Data, col: usize) {
    let length = data.len();
    let mut open_index = length - 1;

    for row in 0..data.len() {
        let position = length - row - 1;

        match data[position][col] {
            'O' => {
                data[position][col] = '.';
                data[open_index][col] = 'O';

                if open_index != 0 {
                    open_index -= 1;
                }
            }
            '#' => {
                if position != 0 {
                    open_index = position - 1;
                }
            }
            _ => (),
        };
    }
}

fn tilt_row_east(data: &mut Data, row: usize) {
    let length = data[0].len();
    let mut open_index = length - 1;

    for col in 0..data[0].len() {
        let position = length - col - 1;

        match data[row][position] {
            'O' => {
                data[row][position] = '.';
                data[row][open_index] = 'O';

                if open_index != 0 {
                    open_index -= 1;
                }
            }
            '#' => {
                if position != 0 {
                    open_index = position - 1;
                }
            }
            _ => (),
        };
    }
}

fn calculate_string_weight(data: &str, num_rows: usize, num_cols: usize) -> usize {
    let mut score = 0;

    assert!(num_rows * num_cols == data.len());

    for row in 0..num_rows {
        let row_data = &data[(row * num_cols)..((row + 1) * num_cols)];

        let round_rock_count = row_data.chars().filter(|&tile| tile == 'O').count();

        score += round_rock_count * (num_rows - row);
    }

    score
}

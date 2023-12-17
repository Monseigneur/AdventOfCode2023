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

    let length = data.len();

    // Track the updated position for each item in the column, and track the latest  "wall" (where the
    // round rocks would stop).

    let mut wall_index = length;
    let mut updated_positions = vec![0; length];

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

        updated_positions[position_index] = wall_index;
    }

    score
}

fn part_2(data: &str) -> usize {
    0
}

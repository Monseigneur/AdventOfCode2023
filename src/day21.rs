use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day21/input.txt").unwrap();

    utilities::print_results(21, || part_1(&contents), || part_2(&contents));
}

type CharGrid = Vec<Vec<char>>;

// Given a grid representing a garden with starting position 'S', gardens '.', and rocks '#', calculate
// the number of positions that can be reached in 64 steps.
fn part_1(data: &str) -> usize {
    let grid = parse_input(data);

    let starting_point = find_start(&grid).unwrap();

    find_reachable_plots(&starting_point, 64, &grid)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn parse_input(data: &str) -> CharGrid {
    data.lines()
        .map(|line| line.chars().collect())
        .collect::<CharGrid>()
}

fn find_start(grid: &CharGrid) -> Option<Point> {
    for (row, row_data) in grid.iter().enumerate() {
        for (col, &c) in row_data.iter().enumerate() {
            if c == 'S' {
                return Some(Point::new(row, col));
            }
        }
    }

    None
}

fn find_reachable_plots(starting_point: &Point, max_steps: usize, grid: &CharGrid) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // Sounds like it can be a level-aware BFS?
    let mut queue: VecDeque<Point> = VecDeque::new();
    let mut visited: HashSet<Point> = HashSet::new();

    queue.push_back(starting_point.clone());

    let mut prev_level_size = 0;
    let mut current_level_count = queue.len();
    let mut current_level = 0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        current_level_count -= 1;

        if !visited.contains(&current) {
            // Handle neighbors
            let neighbors = get_neighbors(&current, num_rows, num_cols);

            for neighbor in neighbors {
                if grid[neighbor.row][neighbor.col] != '#' {
                    queue.push_back(neighbor);
                }
            }

            visited.insert(current);
        }

        if current_level_count == 0 {
            // Processed all of the nodes for the current level. The nodes in the queue are now
            // the potential nodes of the next level, though may be already visited nodes.
            current_level += 1;
            current_level_count = queue.len();

            prev_level_size = visited.len() - prev_level_size;

            if current_level > max_steps {
                break;
            }
        }
    }

    if current_level <= max_steps {
        println!("Exited early, ran out of locations to reach at level {current_level}");
    }

    prev_level_size
}

fn get_neighbors(current: &Point, num_rows: usize, num_cols: usize) -> Vec<Point> {
    let mut neighbors = vec![];

    if current.row > 0 {
        neighbors.push(Point::new(current.row - 1, current.col));
    }

    if current.col < num_cols - 1 {
        neighbors.push(Point::new(current.row, current.col + 1));
    }

    if current.row < num_rows - 1 {
        neighbors.push(Point::new(current.row + 1, current.col));
    }

    if current.col > 0 {
        neighbors.push(Point::new(current.row, current.col - 1));
    }

    neighbors
}

fn part_2(data: &str) -> usize {
    0
}

use std::collections::HashMap;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day10/input.txt").unwrap();

    utilities::print_results(10, || part_1(&contents), || part_2(&contents));
}

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn move_direction(&mut self, direction: &Direction) {
        match direction {
            Direction::North => self.row -= 1,
            Direction::East => self.col += 1,
            Direction::South => self.row += 1,
            Direction::West => self.col -= 1,
            Direction::Start => (),
        };
    }
}

// Given the input, find the point along the loop farthest from where S is.
fn part_1(data: &str) -> usize {
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let animal = find_animal(&data);
    let tile_lookup = TileLookup::new();

    // Search for the start of the loop
    let (mut current, mut direction) = find_start(&data, &animal, &tile_lookup);

    let mut count = 0;

    while direction != Direction::Start {
        count += 1;
        direction = make_move(&data, &current, direction, &tile_lookup);

        current.move_direction(&direction);
    }

    count / 2
}

#[derive(Debug, PartialEq)]
enum Direction {
    Start,
    North,
    East,
    South,
    West,
}

fn find_animal(data: &Vec<Vec<char>>) -> Point {
    let mut start_point: Option<Point> = None;

    for (row, row_data) in data.iter().enumerate() {
        for (col, tile) in row_data.iter().enumerate() {
            if tile == &'S' {
                start_point = Some(Point::new(row, col));
            }
        }
    }

    start_point.unwrap()
}

struct TileLookup {
    map: HashMap<char, (Direction, Direction)>,
}

impl TileLookup {
    fn new() -> Self {
        let mut map: HashMap<char, (Direction, Direction)> = HashMap::new();

        // Store the directions allowed to move into this tile.
        // When returning, need to flip direction to the outbound one.
        map.insert('|', (Direction::North, Direction::South));
        map.insert('-', (Direction::East, Direction::West));
        map.insert('L', (Direction::South, Direction::West));
        map.insert('J', (Direction::South, Direction::East));
        map.insert('7', (Direction::North, Direction::East));
        map.insert('F', (Direction::North, Direction::West));

        Self { map }
    }

    fn allow_entry(&self, tile: char, prev_direction: &Direction) -> bool {
        self.map
            .get(&tile)
            .map(|(a, b)| prev_direction == a || prev_direction == b)
            .unwrap_or(false)
    }

    fn lookup_next_direction(&self, tile: char, prev_direction: &Direction) -> Direction {
        if tile == 'S' {
            return Direction::Start;
        }

        let opposite_entry = self
            .map
            .get(&tile)
            .map(|(a, b)| if prev_direction == a { b } else { a })
            .unwrap();

        TileLookup::flip_direction(opposite_entry)
    }

    fn flip_direction(direction: &Direction) -> Direction {
        match direction {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::Start => Direction::Start,
        }
    }
}

fn find_start(data: &Vec<Vec<char>>, animal: &Point, lookup: &TileLookup) -> (Point, Direction) {
    // 4 points to check: (r-1, c), (r, c+1), (r+1, c), (r, c-1)

    let mut start_point = Point::new(animal.row, animal.col);

    if start_point.row > 0 {
        let direction = Direction::North;
        if lookup.allow_entry(data[start_point.row - 1][start_point.col], &direction) {
            start_point.move_direction(&direction);

            return (start_point, direction);
        }
    }

    if start_point.col < data[0].len() - 1 {
        let direction = Direction::East;
        if lookup.allow_entry(data[start_point.row][start_point.col + 1], &direction) {
            start_point.move_direction(&direction);

            return (start_point, direction);
        }
    }

    if start_point.row < data.len() - 1 {
        let direction = Direction::South;
        if lookup.allow_entry(data[start_point.row + 1][start_point.col], &direction) {
            start_point.move_direction(&direction);

            return (start_point, direction);
        }
    }

    if start_point.col > 0 {
        let direction = Direction::West;
        if lookup.allow_entry(data[start_point.row][start_point.col - 1], &direction) {
            start_point.move_direction(&direction);

            return (start_point, direction);
        }
    }

    panic!("No move found!");
}

fn make_move(
    data: &Vec<Vec<char>>,
    current: &Point,
    prev_direction: Direction,
    lookup: &TileLookup,
) -> Direction {
    let current_tile = data[current.row][current.col];

    if current_tile == 'S' {
        Direction::Start
    } else {
        lookup.lookup_next_direction(current_tile, &prev_direction)
    }
}

fn part_2(data: &str) -> usize {
    0
}

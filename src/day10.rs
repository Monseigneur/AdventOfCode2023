use std::collections::HashMap;

use utilities;

pub fn run() {
    utilities::run_puzzle(10, true, part_1, part_2);
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
        direction = make_move(&data, &current, &direction, &tile_lookup);

        current.move_direction(&direction);
    }

    count / 2
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
    prev_direction: &Direction,
    lookup: &TileLookup,
) -> Direction {
    let current_tile = data[current.row][current.col];

    if current_tile == 'S' {
        Direction::Start
    } else {
        lookup.lookup_next_direction(current_tile, &prev_direction)
    }
}

// Figure out how many tiles are within the bounds of the loop.
fn part_2(data: &str) -> usize {
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // First find the loop and remove other tiles.
    let mut tile_data: Vec<Vec<char>> = vec![vec!['.'; data[0].len()]; data.len()];

    let animal = find_animal(&data);

    tile_data[animal.row][animal.col] = data[animal.row][animal.col];

    let tile_lookup = TileLookup::new();

    // Search for the start of the loop
    let (mut current, mut direction) = find_start(&data, &animal, &tile_lookup);

    while direction != Direction::Start {
        tile_data[current.row][current.col] = data[current.row][current.col];

        direction = make_move(&data, &current, &direction, &tile_lookup);

        current.move_direction(&direction);
    }

    // New idea, with the path, start from the edge and find the wall. At that point, start following the path in
    // the up/right direction. Now, everything on the left is outside, and the right is inside.

    replace_animal(&mut tile_data, &animal, &tile_lookup);

    // First find first wall
    let mut starting_wall = None;

    'outer: for col in 0..tile_data[0].len() {
        for row in 0..tile_data.len() {
            match tile_data[row][col] {
                '.' => {
                    tile_data[row][col] = 'O';
                    continue;
                }
                _ => {
                    starting_wall = Some(Point::new(row, col));

                    break 'outer;
                }
            }
        }
    }

    assert!(starting_wall.is_some());

    let starting_wall = starting_wall.unwrap();

    let (mut current, mut direction) = find_start(&tile_data, &starting_wall, &tile_lookup);

    let num_rows = tile_data.len();
    let num_cols = tile_data[0].len();

    while tile_data[current.row][current.col] != 'W' {
        let prev_direction = direction;
        direction = make_move(&tile_data, &current, &prev_direction, &tile_lookup);

        // Set current tile to W for wall, and left to O and right to I.
        tile_data[current.row][current.col] = 'W';

        if let Some(outside_point) =
            get_adjacent_point(&current, &prev_direction, num_rows, num_cols, true)
        {
            if tile_data[outside_point.row][outside_point.col] == '.' {
                tile_data[outside_point.row][outside_point.col] = 'O';
            }
        }

        if let Some(inside_point) =
            get_adjacent_point(&current, &prev_direction, num_rows, num_cols, false)
        {
            if tile_data[inside_point.row][inside_point.col] == '.' {
                tile_data[inside_point.row][inside_point.col] = 'I';
            }
        }

        if let Some(outside_point) =
            get_adjacent_point(&current, &direction, num_rows, num_cols, true)
        {
            if tile_data[outside_point.row][outside_point.col] == '.' {
                tile_data[outside_point.row][outside_point.col] = 'O';
            }
        }

        if let Some(inside_point) =
            get_adjacent_point(&current, &direction, num_rows, num_cols, false)
        {
            if tile_data[inside_point.row][inside_point.col] == '.' {
                tile_data[inside_point.row][inside_point.col] = 'I';
            }
        }

        current.move_direction(&direction);
    }

    flood_fill(&mut tile_data);

    tile_data.iter().fold(0, |acc, row_data| {
        acc + row_data.iter().filter(|&&tile| tile == 'I').count()
    })
}

fn replace_animal(tile_data: &mut Vec<Vec<char>>, animal: &Point, lookup: &TileLookup) {
    let mut north_wall = false;

    if animal.row > 0 {
        if lookup.allow_entry(tile_data[animal.row - 1][animal.col], &Direction::North) {
            north_wall = true;
        }
    }

    let mut east_wall = false;

    if animal.col < tile_data[0].len() - 1 {
        if lookup.allow_entry(tile_data[animal.row][animal.col + 1], &Direction::East) {
            east_wall = true;
        }
    }

    let mut south_wall = false;

    if animal.row < tile_data.len() - 1 {
        if lookup.allow_entry(tile_data[animal.row + 1][animal.col], &Direction::South) {
            south_wall = true;
        }
    }

    let mut west_wall = false;

    if animal.col > 0 {
        if lookup.allow_entry(tile_data[animal.row][animal.col - 1], &Direction::West) {
            west_wall = true;
        }
    }

    // Wall types:
    //  | NS
    //  - EW
    //  L NE
    //  J NW
    //  7 SW
    //  F SE

    let new_animal = if north_wall && south_wall {
        '|'
    } else if north_wall && east_wall {
        'L'
    } else if north_wall && west_wall {
        'J'
    } else if east_wall && west_wall {
        '-'
    } else if east_wall && south_wall {
        'F'
    } else {
        '7'
    };

    tile_data[animal.row][animal.col] = new_animal;
}

fn get_adjacent_point(
    current: &Point,
    direction: &Direction,
    num_rows: usize,
    num_cols: usize,
    left: bool,
) -> Option<Point> {
    if left {
        match direction {
            Direction::North => {
                if current.col > 0 {
                    return Some(Point::new(current.row, current.col - 1));
                }
            }
            Direction::East => {
                if current.row > 0 {
                    return Some(Point::new(current.row - 1, current.col));
                }
            }
            Direction::South => {
                if current.col < num_cols - 1 {
                    return Some(Point::new(current.row, current.col + 1));
                }
            }
            Direction::West => {
                if current.row < num_rows - 1 {
                    return Some(Point::new(current.row + 1, current.col));
                }
            }
            _ => return None,
        }
    } else {
        match direction {
            Direction::North => {
                if current.col < num_cols - 1 {
                    return Some(Point::new(current.row, current.col + 1));
                }
            }
            Direction::East => {
                if current.row < num_rows - 1 {
                    return Some(Point::new(current.row + 1, current.col));
                }
            }
            Direction::South => {
                if current.col > 0 {
                    return Some(Point::new(current.row, current.col - 1));
                }
            }
            Direction::West => {
                if current.row > 0 {
                    return Some(Point::new(current.row - 1, current.col));
                }
            }
            _ => return None,
        }
    }

    None
}

fn flood_fill(tile_data: &mut Vec<Vec<char>>) {
    let num_rows = tile_data.len();
    let num_cols = tile_data[0].len();

    loop {
        let mut open_ground_count = 0;

        for row in 0..num_rows {
            for col in 0..num_cols {
                fill_around(tile_data, row, col);
            }

            for col in 0..num_cols {
                if tile_data[row][col] == '.' {
                    open_ground_count += 1;
                }
            }
        }

        if open_ground_count == 0 {
            break;
        }
    }
}

fn fill_around(tile_data: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let current_tile = tile_data[row][col];

    if current_tile == 'W' {
        return;
    }

    if row > 0 && tile_data[row - 1][col] == '.' {
        tile_data[row - 1][col] = current_tile;
    }

    if col < tile_data[0].len() - 1 && tile_data[row][col + 1] == '.' {
        tile_data[row][col + 1] = current_tile;
    }

    if row < tile_data.len() - 1 && tile_data[row + 1][col] == '.' {
        tile_data[row + 1][col] = current_tile;
    }

    if col > 0 && tile_data[row][col - 1] == '.' {
        tile_data[row][col - 1] = current_tile;
    }
}

use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day18/input.txt").unwrap();

    utilities::print_results(18, || part_1(&contents), || part_2(&contents));
}

type CharGrid = Vec<Vec<char>>;

// Following the dig path, figure out the enclosed area. This seems like day 10 again.
fn part_1(data: &str) -> usize {
    let dig_plan = parse_input(data);

    let (num_rows, num_cols, starting_point) = calculate_bounds(&dig_plan);

    // Fill in the initial path, leaving a border around to flood fill from.
    let mut grid: CharGrid = vec![vec!['.'; num_cols + 2]; num_rows + 2];

    prefill_grid(&mut grid);
    fill_path(&mut grid, &starting_point, &dig_plan);
    flood_fill(&mut grid);

    let grid_area = grid.len() * grid[0].len();

    let outside_count = grid.iter().fold(0, |acc, row_data| {
        acc + row_data.iter().filter(|&&tile| tile == 'O').count()
    });

    grid_area - outside_count
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

    fn new_from_other(other: &Point) -> Self {
        Self {
            row: other.row,
            col: other.col,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_str(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "R" => Self::Right,
            "D" => Self::Down,
            "L" => Self::Left,
            _ => panic!("Illegal string!"),
        }
    }
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    distance: usize,
    color: String,
}

impl Dig {
    fn new(line: &str) -> Self {
        let plan_pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

        let direction = Direction::from_str(plan_pieces[0]);
        let distance = plan_pieces[1].parse::<usize>().unwrap();
        let color = plan_pieces[2].to_owned();

        Self {
            direction,
            distance,
            color,
        }
    }

    fn new2(line: &str) -> Self {
        let plan_pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

        let color_piece = plan_pieces[2]
            .strip_prefix("(#")
            .unwrap()
            .strip_suffix(")")
            .unwrap();

        let distance = usize::from_str_radix(&color_piece[0..(color_piece.len() - 1)], 16).unwrap();

        let direction = match &color_piece[(color_piece.len() - 1)..] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => panic!("Illegal direction"),
        };

        let color = plan_pieces[2].to_owned();

        Self {
            direction,
            distance,
            color,
        }
    }
}

fn parse_input(data: &str) -> Vec<Dig> {
    data.lines().map(|line| Dig::new(line)).collect()
}

fn calculate_bounds(dig_plan: &Vec<Dig>) -> (usize, usize, Point) {
    // Figure out the bounds
    let mut upmost = 0;
    let mut rightmost = 0;
    let mut downmost = 0;
    let mut leftmost = 0;

    let mut x = 0;
    let mut y = 0;

    for dig in dig_plan {
        // Using increasing x to the right, increasing y going down.
        match dig.direction {
            Direction::Up => {
                y -= dig.distance as isize;
                upmost = upmost.min(y);
            }
            Direction::Right => {
                x += dig.distance as isize;
                rightmost = rightmost.max(x);
            }
            Direction::Down => {
                y += dig.distance as isize;
                downmost = downmost.max(y);
            }
            Direction::Left => {
                x -= dig.distance as isize;
                leftmost = leftmost.min(x);
            }
        }
    }

    // Want (leftmost, upmost) to be (0, 0)
    let num_rows = (downmost - upmost + 1) as usize;
    let num_cols = (rightmost - leftmost + 1) as usize;

    let adjusted_x = (x - leftmost) as usize;
    let adjusted_y = (y - upmost) as usize;

    (num_rows, num_cols, Point::new(adjusted_y, adjusted_x))
}

fn prefill_grid(grid: &mut CharGrid) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for col in 0..num_cols {
        grid[0][col] = 'O';
        grid[num_rows - 1][col] = 'O';
    }

    for row in 0..num_rows {
        grid[row][0] = 'O';
        grid[row][num_cols - 1] = 'O';
    }
}

fn fill_path(grid: &mut CharGrid, starting_point: &Point, dig_plan: &Vec<Dig>) {
    let mut current = Point::new_from_other(starting_point);
    current.row += 1;
    current.col += 1;

    for dig in dig_plan {
        for i in 0..dig.distance {
            match dig.direction {
                Direction::Up => grid[current.row - i - 1][current.col] = '#',
                Direction::Right => grid[current.row][current.col + i + 1] = '#',
                Direction::Down => grid[current.row + i + 1][current.col] = '#',
                Direction::Left => grid[current.row][current.col - i - 1] = '#',
            }
        }

        match dig.direction {
            Direction::Up => current.row -= dig.distance,
            Direction::Right => current.col += dig.distance,
            Direction::Down => current.row += dig.distance,
            Direction::Left => current.col -= dig.distance,
        }
    }
}

fn flood_fill(grid: &mut CharGrid) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    loop {
        let mut updated = 0;

        for row in 0..num_rows {
            for col in 0..num_cols {
                updated += fill_around(grid, row, col);
            }
        }

        if updated == 0 {
            break;
        }
    }
}

fn fill_around(grid: &mut CharGrid, row: usize, col: usize) -> usize {
    let current_tile = grid[row][col];

    if current_tile != 'O' {
        return 0;
    }

    let mut count = 0;

    if row > 0 && grid[row - 1][col] == '.' {
        grid[row - 1][col] = current_tile;
        count += 1;
    }

    if col < grid[0].len() - 1 && grid[row][col + 1] == '.' {
        grid[row][col + 1] = current_tile;
        count += 1;
    }

    if row < grid.len() - 1 && grid[row + 1][col] == '.' {
        grid[row + 1][col] = current_tile;
        count += 1;
    }

    if col > 0 && grid[row][col - 1] == '.' {
        grid[row][col - 1] = current_tile;
        count += 1;
    }

    count
}

// The elves misinterpreted the input data, and instead the color field is the important information, where
// the first 5 hex digits give the distance, and the last hex digit gives the direction.
fn part_2(data: &str) -> usize {
    let dig_plan = parse_input2(data);

    calculate_area(&dig_plan)
}

fn parse_input2(data: &str) -> Vec<Dig> {
    data.lines().map(|line| Dig::new2(line)).collect()
}

#[derive(Debug)]
struct IPoint {
    x: isize,
    y: isize,
}

impl IPoint {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn apply(&self, direction: &Direction, distance: usize) -> Self {
        let distance = distance as isize;

        match direction {
            Direction::Up => Self {
                y: self.y + distance,
                ..*self
            },
            Direction::Right => Self {
                x: self.x + distance,
                ..*self
            },
            Direction::Down => Self {
                y: self.y - distance,
                ..*self
            },
            Direction::Left => Self {
                x: self.x - distance,
                ..*self
            },
        }
    }
}

fn calculate_area(dig_plan: &Vec<Dig>) -> usize {
    let mut right_count: usize = 0;
    let mut left_count: usize = 0;

    let mut current = IPoint::new(0, 0);

    let mut inner_area = 0;
    let mut edge_area = 0;

    for i in 0..dig_plan.len() {
        let dig = &dig_plan[i];

        let next = current.apply(&dig.direction, dig.distance);

        inner_area += current.x * next.y - current.y * next.x;

        // Calculate the edge area
        edge_area += dig.distance;

        let j = (i + 1) % dig_plan.len();

        // Count the number of each turn, to determine which side is "outside".
        if is_right_turn(dig, &dig_plan[j]) {
            right_count += 1;
        } else {
            left_count += 1;
        }

        current = next;
    }

    let total_area = inner_area.abs() as usize + edge_area + right_count.abs_diff(left_count) / 2;

    total_area / 2
}

fn is_right_turn(dig: &Dig, next_dig: &Dig) -> bool {
    let first_dir = &dig.direction;
    let second_dir = &next_dig.direction;

    match (first_dir, second_dir) {
        (Direction::Up, Direction::Right) => true,
        (Direction::Up, Direction::Left) => false,
        (Direction::Right, Direction::Down) => true,
        (Direction::Right, Direction::Up) => false,
        (Direction::Down, Direction::Left) => true,
        (Direction::Down, Direction::Right) => false,
        (Direction::Left, Direction::Up) => true,
        (Direction::Left, Direction::Down) => false,
        _ => panic!("Illegal direction combo"),
    }
}

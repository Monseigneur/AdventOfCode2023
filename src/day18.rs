use utilities;

pub fn run() {
    utilities::run_puzzle(18, true, part_1, part_2);
}

// Following the dig path, figure out the enclosed area. This seems like day 10 again.
fn part_1(data: &str) -> usize {
    let dig_plan = data.lines().map(|line| Dig::new(line, false)).collect();

    calculate_area(&dig_plan)
}

#[derive(Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }

    fn apply(&self, direction: &Direction, distance: usize) -> Self {
        let distance = distance as isize;

        let (new_x, new_y) = match direction {
            Direction::Up => (self.x, self.y + distance),
            Direction::Right => (self.x + distance, self.y),
            Direction::Down => (self.x, self.y - distance),
            Direction::Left => (self.x - distance, self.y),
        };

        Self { x: new_x, y: new_y }
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
            "R" | "0" => Self::Right,
            "D" | "1" => Self::Down,
            "L" | "2" => Self::Left,
            "U" | "3" => Self::Up,
            _ => panic!("Illegal string!"),
        }
    }
}

#[derive(Debug)]
struct Dig {
    direction: Direction,
    distance: usize,
}

impl Dig {
    fn new(line: &str, use_color: bool) -> Self {
        let plan_pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

        let direction;
        let distance;

        if use_color {
            let color_piece = plan_pieces[2]
                .strip_prefix("(#")
                .unwrap()
                .strip_suffix(")")
                .unwrap();

            let cut_index = color_piece.len() - 1;

            distance = usize::from_str_radix(&color_piece[0..cut_index], 16).unwrap();
            direction = Direction::from_str(&color_piece[cut_index..]);
        } else {
            direction = Direction::from_str(plan_pieces[0]);
            distance = plan_pieces[1].parse::<usize>().unwrap();
        }

        Self {
            direction,
            distance,
        }
    }
}

// The elves misinterpreted the input data, and instead the color field is the important information, where
// the first 5 hex digits give the distance, and the last hex digit gives the direction.
fn part_2(data: &str) -> usize {
    let dig_plan = data.lines().map(|line| Dig::new(line, true)).collect();

    calculate_area(&dig_plan)
}

fn calculate_area(dig_plan: &Vec<Dig>) -> usize {
    let mut right_count: usize = 0;
    let mut left_count: usize = 0;

    let mut current = Point::default();

    let mut inner_area = 0;
    let mut edge_area = 0;

    for i in 0..dig_plan.len() {
        let dig = &dig_plan[i];

        let next = current.apply(&dig.direction, dig.distance);

        inner_area += current.x * next.y - current.y * next.x;

        // Calculate the edge area.
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

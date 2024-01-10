use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day17/input.txt").unwrap();

    utilities::print_results(17, || part_1(&contents), || part_2(&contents));
}

// A crucible must move from the top left corner to the bottom right, with a max of 3 blocks in any given
// direction at once. Each block has a value that is the amount of heat loss from that block. Find the path
// that gives the minimum heat loss.

type Grid = Vec<Vec<usize>>;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Copy, Hash)]
enum Direction {
    Start,
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct BlockDist {
    direction: Direction,
    steps: usize,
}

impl BlockDist {
    fn start() -> Self {
        Self {
            steps: 0,
            direction: Direction::Start,
        }
    }

    fn new(direction: Direction, steps: usize) -> Self {
        Self { direction, steps }
    }

    fn advance(&self) -> Self {
        Self {
            steps: self.steps + 1,
            ..*self
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, Copy)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn default() -> Self {
        Self { row: 0, col: 0 }
    }

    fn new_from_direction(&self, direction: Direction, dist: usize) -> Self {
        let (row, col) = match direction {
            Direction::Up => (self.row - dist, self.col),
            Direction::Right => (self.row, self.col + dist),
            Direction::Down => (self.row + dist, self.col),
            Direction::Left => (self.row, self.col - dist),
            _ => (self.row, self.col),
        };

        Self { row, col }
    }
}

fn part_1(data: &str) -> usize {
    let grid = parse_input(data);

    minimum_heat(&grid, 1, 3)
}

fn parse_input(data: &str) -> Grid {
    data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Grid>()
}

fn minimum_heat(grid: &Grid, min_steps: usize, max_steps: usize) -> usize {
    let start_point = Point::default();
    let end_point = Point::new(grid.len() - 1, grid[0].len() - 1);

    let mut visited: HashSet<(Point, BlockDist)> = HashSet::new();
    let mut heat_map: HashMap<(Point, BlockDist), usize> = HashMap::new();

    let mut queue = BinaryHeap::new();

    queue.push((Reverse(0), start_point, BlockDist::start()));

    let mut best_heat = None;
    while let Some((Reverse(heat), position, block_dist)) = queue.pop() {
        if position == end_point {
            best_heat = Some(heat);
            break;
        }

        if heat_map
            .get(&(position, block_dist))
            .is_some_and(|&best_heat| best_heat < heat)
        {
            continue;
        }

        let key = (position, block_dist);

        if visited.contains(&key) {
            continue;
        }

        for (neighbor, new_block_dist) in
            get_neighbors(&grid, &position, &block_dist, min_steps, max_steps)
        {
            let neighbor_heat = heat + get_heat_loss(grid, &position, &neighbor);

            if heat_map
                .get(&(neighbor, new_block_dist))
                .is_some_and(|&best_heat| best_heat < neighbor_heat)
            {
                continue;
            }

            queue.push((Reverse(neighbor_heat), neighbor, new_block_dist));

            heat_map.insert((neighbor, new_block_dist), neighbor_heat);
        }

        visited.insert(key);
    }

    best_heat.unwrap()
}

fn get_neighbors(
    grid: &Grid,
    point: &Point,
    block_dist: &BlockDist,
    min_steps: usize,
    max_steps: usize,
) -> Vec<(Point, BlockDist)> {
    let mut neighbors = vec![];

    let is_start = block_dist.direction == Direction::Start;

    // Up
    if block_dist.direction != Direction::Down {
        let new_direction = Direction::Up;
        let far_from_edge = point.row >= min_steps;

        if is_start || block_dist.direction != new_direction {
            // This case shouldn't happen, but let's write it for consistency.
            if far_from_edge {
                let new_block_dist = BlockDist::new(new_direction, min_steps);
                let new_point = point.new_from_direction(new_direction, min_steps);

                neighbors.push((new_point, new_block_dist));
            }
        } else {
            if point.row > 0 && block_dist.steps < max_steps {
                let new_block_dist = block_dist.advance();
                let new_point = point.new_from_direction(new_direction, 1);

                neighbors.push((new_point, new_block_dist));
            }
        }
    }

    // Right
    if block_dist.direction != Direction::Left {
        let new_direction = Direction::Right;
        let far_from_edge = point.col < grid[0].len() - min_steps;

        if is_start || block_dist.direction != new_direction {
            if far_from_edge {
                let new_block_dist = BlockDist::new(new_direction, min_steps);
                let new_point = point.new_from_direction(new_direction, min_steps);

                neighbors.push((new_point, new_block_dist));
            }
        } else {
            if point.col < grid[0].len() - 1 && block_dist.steps < max_steps {
                let new_block_dist = block_dist.advance();
                let new_point = point.new_from_direction(new_direction, 1);

                neighbors.push((new_point, new_block_dist));
            }
        }
    }

    // Down
    if block_dist.direction != Direction::Up {
        let new_direction = Direction::Down;
        let far_from_edge = point.row < grid.len() - min_steps;

        if is_start || block_dist.direction != new_direction {
            if far_from_edge {
                let new_block_dist = BlockDist::new(new_direction, min_steps);
                let new_point = point.new_from_direction(new_direction, min_steps);

                neighbors.push((new_point, new_block_dist));
            }
        } else {
            if point.row < grid.len() - 1 && block_dist.steps < max_steps {
                let new_block_dist = block_dist.advance();
                let new_point = point.new_from_direction(new_direction, 1);

                neighbors.push((new_point, new_block_dist));
            }
        }
    }

    // Left
    if block_dist.direction != Direction::Right {
        let new_direction = Direction::Left;
        let far_from_edge = point.col >= min_steps;

        if is_start || block_dist.direction != new_direction {
            // This case shouldn't happen, but let's write it for consistency.
            if far_from_edge {
                let new_block_dist = BlockDist::new(new_direction, min_steps);
                let new_point = point.new_from_direction(new_direction, min_steps);

                neighbors.push((new_point, new_block_dist));
            }
        } else {
            if point.col > 0 && block_dist.steps < max_steps {
                let new_block_dist = block_dist.advance();
                let new_point = point.new_from_direction(new_direction, 1);

                neighbors.push((new_point, new_block_dist));
            }
        }
    }

    neighbors
}

fn get_heat_loss(grid: &Grid, start: &Point, end: &Point) -> usize {
    assert!(start.row < grid.len());
    assert!(start.col < grid[0].len());

    assert!(end.row < grid.len());
    assert!(end.col < grid[0].len());

    let min_row = start.row.min(end.row);
    let max_row = start.row.max(end.row);

    let min_col = start.col.min(end.col);
    let max_col = start.col.max(end.col);

    let mut heat = 0;

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            heat += grid[r][c];
        }
    }

    heat - grid[start.row][start.col]
}

// The crucibles have been upgraded, but now they can only move a minimum of 4 blocks in a direction, and a max
// of 10.
fn part_2(data: &str) -> usize {
    let grid = parse_input(data);

    minimum_heat(&grid, 4, 10)
}

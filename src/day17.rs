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

    fn new_from_direction(
        &self,
        direction: Direction,
        min_steps: usize,
        max_steps: usize,
    ) -> Option<Self> {
        if self.direction != direction {
            if self.steps < min_steps && self.steps != 0 {
                None
            } else {
                Some(Self {
                    direction,
                    steps: 1,
                })
            }
        } else if self.steps < max_steps {
            Some(Self {
                direction,
                steps: self.steps + 1,
            })
        } else {
            None
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

    fn new_from_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                row: self.row - 1,
                ..*self
            },
            Direction::Right => Self {
                col: self.col + 1,
                ..*self
            },
            Direction::Down => Self {
                row: self.row + 1,
                ..*self
            },
            Direction::Left => Self {
                col: self.col - 1,
                ..*self
            },
            _ => Self { ..*self },
        }
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

    while let Some((Reverse(heat), position, block_dist)) = queue.pop() {
        if position == end_point {
            return heat;
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
            let neighbor_heat = heat + get_heat_loss(&grid, &neighbor);

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

    panic!("Didn't find a path to the end!");
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
    let can_turn = is_start || block_dist.steps >= min_steps;

    // Up
    if block_dist.direction != Direction::Down {
        let new_direction = Direction::Up;
        let far_from_edge = point.row >= min_steps;

        if point.row > 0 && (block_dist.direction == new_direction || (can_turn && far_from_edge)) {
            if let Some(new_block_dist) =
                block_dist.new_from_direction(new_direction, min_steps, max_steps)
            {
                neighbors.push((point.new_from_direction(new_direction), new_block_dist));
            }
        }
    }

    // Right
    if block_dist.direction != Direction::Left {
        let new_direction = Direction::Right;
        let far_from_edge = point.col <= grid[0].len() - min_steps;

        if point.col < grid[0].len() - 1
            && (block_dist.direction == new_direction || (can_turn && far_from_edge))
        {
            if let Some(new_block_dist) =
                block_dist.new_from_direction(Direction::Right, min_steps, max_steps)
            {
                neighbors.push((point.new_from_direction(Direction::Right), new_block_dist));
            }
        }
    }

    // Down
    if block_dist.direction != Direction::Up {
        let new_direction = Direction::Down;
        let far_from_edge = point.row <= grid.len() - min_steps;

        if point.row < grid.len() - 1
            && (block_dist.direction == new_direction || (can_turn && far_from_edge))
        {
            if let Some(new_block_dist) =
                block_dist.new_from_direction(Direction::Down, min_steps, max_steps)
            {
                neighbors.push((point.new_from_direction(Direction::Down), new_block_dist));
            }
        }
    }

    // Left
    if block_dist.direction != Direction::Right {
        let new_direction = Direction::Left;
        let far_from_edge = point.col >= min_steps;

        if point.col > 0 && (block_dist.direction == new_direction || (can_turn && far_from_edge)) {
            if let Some(new_block_dist) =
                block_dist.new_from_direction(Direction::Left, min_steps, max_steps)
            {
                neighbors.push((point.new_from_direction(Direction::Left), new_block_dist));
            }
        }
    }

    neighbors
}

fn get_heat_loss(grid: &Grid, point: &Point) -> usize {
    assert!(point.row < grid.len());
    assert!(point.col < grid[0].len());

    grid[point.row][point.col]
}

// The crucibles have been upgraded, but now they can only move a minimum of 4 blocks in a direction, and a max
// of 10.
fn part_2(data: &str) -> usize {
    let grid = parse_input(data);

    minimum_heat(&grid, 4, 10)
}

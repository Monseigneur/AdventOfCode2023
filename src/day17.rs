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

    fn new_from_direction(&self, direction: Direction) -> Option<Self> {
        if self.direction != direction {
            Some(Self {
                direction,
                steps: 1,
            })
        } else if self.steps < 3 {
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
}

fn part_1(data: &str) -> usize {
    let grid = parse_input(data);

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

        for (neighbor, new_block_dist) in get_neighbors(&grid, &position, &block_dist) {
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

    unreachable!()
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

fn get_neighbors(grid: &Grid, point: &Point, block_dist: &BlockDist) -> Vec<(Point, BlockDist)> {
    let mut neighbors = vec![];

    // Up
    if point.row > 0 && block_dist.direction != Direction::Down {
        if let Some(new_block_dist) = block_dist.new_from_direction(Direction::Up) {
            neighbors.push((Point::new(point.row - 1, point.col), new_block_dist));
        }
    }

    // Right
    if point.col < grid[0].len() - 1 && block_dist.direction != Direction::Left {
        if let Some(new_block_dist) = block_dist.new_from_direction(Direction::Right) {
            neighbors.push((Point::new(point.row, point.col + 1), new_block_dist));
        }
    }

    // Down
    if point.row < grid.len() - 1 && block_dist.direction != Direction::Up {
        if let Some(new_block_dist) = block_dist.new_from_direction(Direction::Down) {
            neighbors.push((Point::new(point.row + 1, point.col), new_block_dist));
        }
    }

    // Left
    if point.col > 0 && block_dist.direction != Direction::Right {
        if let Some(new_block_dist) = block_dist.new_from_direction(Direction::Left) {
            neighbors.push((Point::new(point.row, point.col - 1), new_block_dist));
        }
    }

    neighbors
}

fn get_heat_loss(grid: &Grid, point: &Point) -> usize {
    assert!(point.row < grid.len());
    assert!(point.col < grid[0].len());

    grid[point.row][point.col]
}

fn part_2(_data: &str) -> usize {
    0
}

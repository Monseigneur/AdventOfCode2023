use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::ops::Range;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day22/input.txt").unwrap();

    utilities::print_results(22, || part_1(&contents), || part_2(&contents));
}

// Given a list of bricks specified as pairs of (x, y, z) coordinates representing the ends, determine
// how many bricks can be individually disintegrated once they reach the ground (z == 0) and cause no
// other bricks to shift positions. So, jenga.
fn part_1(data: &str) -> usize {
    let falling_bricks = parse_input(data);
    let bricks = get_final_bricks(&falling_bricks);

    count_disintegrated_bricks(&bricks)
}

#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

type BrickId = usize;

#[derive(Debug, Clone)]
struct Brick {
    id: BrickId,
    start: Point,
    end: Point,
}

impl Brick {
    fn new(line: &str, id: BrickId) -> Self {
        let coordinates = line
            .split(|c: char| c == ',' || c == '~')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self {
            id,
            start: Point::new(coordinates[0], coordinates[1], coordinates[2]),
            end: Point::new(coordinates[3], coordinates[4], coordinates[5]),
        }
    }

    fn get_x_range(&self) -> Range<usize> {
        Brick::get_range(self.start.x, self.end.x)
    }

    fn get_y_range(&self) -> Range<usize> {
        Brick::get_range(self.start.y, self.end.y)
    }

    fn get_z_range(&self) -> Range<usize> {
        Brick::get_range(self.start.z, self.end.z)
    }

    fn get_range(a: usize, b: usize) -> Range<usize> {
        if a < b {
            a..(b + 1)
        } else {
            b..(a + 1)
        }
    }

    fn move_to_z(&mut self, new_z: usize) {
        let z_min = self.get_z_range().start;
        let delta_z = z_min - new_z;

        self.start.z -= delta_z;
        self.end.z -= delta_z;
    }
}

fn parse_input(data: &str) -> Vec<Brick> {
    let mut bricks = data
        .lines()
        .enumerate()
        .map(|(id, line)| Brick::new(line, id))
        .collect::<Vec<Brick>>();

    bricks.sort_by(|a, b| a.get_z_range().start.cmp(&b.get_z_range().start));

    bricks
}

fn get_final_bricks(falling_bricks: &Vec<Brick>) -> Vec<Brick> {
    // For each point along a brick's (x, y) footprint, check the "max" height of that location.
    let mut bricks = vec![];

    let mut heights: HashMap<(usize, usize), usize> = HashMap::new();

    for brick in falling_bricks {
        let mut min_z = 0;

        // find the minimum z that this brick could rest at.
        for x in brick.get_x_range() {
            for y in brick.get_y_range() {
                min_z = *heights.get(&(x, y)).unwrap_or(&0).max(&min_z);
            }
        }

        let mut new_brick = brick.clone();
        new_brick.move_to_z(min_z + 1);

        let brick_max_z = new_brick.get_z_range().end - 1;

        // Now adjust the heights for these positions.
        for x in brick.get_x_range() {
            for y in brick.get_y_range() {
                heights
                    .entry((x, y))
                    .and_modify(|val| *val = brick_max_z)
                    .or_insert(brick_max_z);
            }
        }

        bricks.push(new_brick);
    }

    bricks
}

fn count_disintegrated_bricks(bricks: &Vec<Brick>) -> usize {
    let bricks_below = get_bricks_below(bricks);

    let required_bricks = get_required_bricks(&bricks_below);

    bricks.len() - required_bricks.len()
}

fn get_bricks_below(bricks: &Vec<Brick>) -> HashMap<BrickId, HashSet<BrickId>> {
    // Determine the brick arrangements in each column.
    let mut heights: HashMap<(usize, usize), HashMap<usize, BrickId>> = HashMap::new();

    for brick in bricks {
        let brick_z = brick.get_z_range().end - 1;

        for x in brick.get_x_range() {
            for y in brick.get_y_range() {
                let key = (x, y);

                if !heights.contains_key(&key) {
                    heights.insert(key, HashMap::new());
                }

                heights.get_mut(&key).unwrap().insert(brick_z, brick.id);
            }
        }
    }

    // Look at the bricks that are immediately below the current brick for each position. If a given brick
    // is immediately held up by only 1 brick, then that brick cannot be disintegrated.
    let mut bricks_below: HashMap<BrickId, HashSet<BrickId>> = HashMap::new();

    for brick in bricks {
        let mut below: HashSet<BrickId> = HashSet::new();

        let below_z = brick.get_z_range().start - 1;

        if below_z != 0 {
            for x in brick.get_x_range() {
                for y in brick.get_y_range() {
                    let key = (x, y);

                    if let Some(column) = heights.get(&key) {
                        if let Some(other_brick) = column.get(&below_z) {
                            below.insert(*other_brick);
                        }
                    }
                }
            }
        }

        bricks_below.insert(brick.id, below);
    }

    bricks_below
}

fn get_required_bricks(bricks_below: &HashMap<BrickId, HashSet<BrickId>>) -> HashSet<BrickId> {
    // If any brick has only 1 brick below, than that below brick is required.
    let mut required_bricks: HashSet<BrickId> = HashSet::new();

    for (_, below) in bricks_below {
        if below.len() == 1 {
            let required: &BrickId = below.iter().next().unwrap();

            required_bricks.insert(*required);
        }
    }

    required_bricks
}

// For each brick, determine the number of bricks that would fall if the given brick was disintegrated, and calculate
// the sum of all bricks that would fall.
fn part_2(data: &str) -> usize {
    let falling_bricks = parse_input(data);

    let bricks = get_final_bricks(&falling_bricks);

    let bricks_below = get_bricks_below(&bricks);
    let bricks_above = get_bricks_above(&bricks_below);

    let required_bricks = get_required_bricks(&bricks_below);

    required_bricks
        .iter()
        .map(|brick| count_above(*brick, &bricks_above, &bricks_below))
        .sum()
}

fn get_bricks_above(
    bricks_below: &HashMap<BrickId, HashSet<BrickId>>,
) -> HashMap<BrickId, HashSet<BrickId>> {
    let mut bricks_above = HashMap::new();

    for (brick, below) in bricks_below {
        for below_brick in below {
            if !bricks_above.contains_key(below_brick) {
                bricks_above.insert(*below_brick, HashSet::new());
            }

            bricks_above.get_mut(below_brick).unwrap().insert(*brick);
        }
    }

    bricks_above
}

fn count_above(
    start_brick: BrickId,
    bricks_above: &HashMap<BrickId, HashSet<BrickId>>,
    bricks_below: &HashMap<BrickId, HashSet<BrickId>>,
) -> usize {
    let mut visited = HashSet::new();
    let mut falling = HashSet::new();

    let mut queue = VecDeque::new();
    falling.insert(start_brick);
    queue.push_back(start_brick);

    while let Some(brick) = queue.pop_front() {
        if visited.contains(&brick) {
            continue;
        }

        if let Some(above) = bricks_above.get(&brick) {
            for other_brick in above {
                // Other brick may be supported by another supported tower.
                let below = bricks_below.get(other_brick).unwrap();

                if falling.is_superset(below) {
                    falling.insert(*other_brick);
                    queue.push_back(*other_brick);
                }
            }
        }

        visited.insert(brick);
    }

    falling.len() - 1
}

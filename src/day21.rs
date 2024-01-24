use std::{
    collections::{HashMap, HashSet, VecDeque},
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn get_row(&self, num_rows: usize) -> usize {
        Point::adjust(self.row, num_rows as isize) as usize
    }

    fn get_col(&self, num_cols: usize) -> usize {
        Point::adjust(self.col, num_cols as isize) as usize
    }

    fn adjust(val: isize, max_val: isize) -> isize {
        if val >= 0 {
            val % max_val
        } else {
            ((val % max_val) + max_val) % max_val
        }
    }

    fn component_wrap_level(val: isize, max_val: isize) -> isize {
        if val >= 0 {
            val / max_val
        } else {
            ((val.abs() - 1) / max_val) + 1
        }
    }

    fn get_grid_coordinates(&self, num_rows: usize, num_cols: usize) -> Point {
        let row = Point::component_wrap_level(self.row, num_rows as isize);
        let col = Point::component_wrap_level(self.col, num_cols as isize);

        Point::new(row, col)
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
                return Some(Point::new(row as isize, col as isize));
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

    queue.push_back(*starting_point);

    let mut prev_level_size = 0;
    let mut current_level_count = queue.len();
    let mut current_level = 0;

    while let Some(current) = queue.pop_front() {
        current_level_count -= 1;

        if !visited.contains(&current) {
            // Handle neighbors
            let neighbors = get_neighbors(&current, num_rows, num_cols, false);

            for neighbor in neighbors {
                if grid[neighbor.get_row(num_rows)][neighbor.get_col(num_cols)] != '#' {
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

fn get_neighbors(
    current: &Point,
    num_rows: usize,
    num_cols: usize,
    allow_wrapping: bool,
) -> Vec<Point> {
    let mut neighbors = vec![];

    if current.row > 0 || allow_wrapping {
        neighbors.push(Point::new(current.row - 1, current.col));
    }

    if current.col < (num_cols - 1) as isize || allow_wrapping {
        neighbors.push(Point::new(current.row, current.col + 1));
    }

    if current.row < (num_rows - 1) as isize || allow_wrapping {
        neighbors.push(Point::new(current.row + 1, current.col));
    }

    if current.col > 0 || allow_wrapping {
        neighbors.push(Point::new(current.row, current.col - 1));
    }

    neighbors
}

// Grid can repeat indefinitely in any direction, and it just tiled.
fn part_2(data: &str) -> usize {
    let grid = parse_input(data);

    let starting_point = find_start(&grid).unwrap();

    // Seems to spread in a diamond pattern, and that the max step value will perfectly inscribe an integer number
    // of grids.
    let grid_data = get_grid_data(&grid, &starting_point);

    find_far_reachable_plots(&grid_data, &starting_point, grid.len(), 26501365)
}

fn get_grid_data(grid: &CharGrid, starting_point: &Point) -> Vec<Vec<(usize, usize)>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut prev_points: HashSet<Point> = HashSet::new();
    let mut points: HashSet<Point> = HashSet::new();

    points.insert(*starting_point);

    // Gather the step data for processing. 5 is used to give enough information about the diagonals of the diamond.
    let scale = 5;
    let mut step_data: Vec<Vec<i32>> = vec![vec![-1; num_cols * scale]; num_rows * scale];

    set_step_data(&mut step_data, starting_point, 0, scale);

    let mut step_count = 1;

    'step_loop: loop {
        let new_points = points
            .iter()
            .flat_map(|&p| get_neighbors(&p, num_rows, num_cols, true))
            .filter(|p| {
                grid[p.get_row(num_rows)][p.get_col(num_cols)] != '#' && !prev_points.contains(p)
            })
            .collect::<HashSet<Point>>();

        // Check if the pattern has spread out of the 5x5 grid. The first will be any points that spill into rows -3, 3
        // or columns -3, 3.
        for point in &new_points {
            let grid_point = point.get_grid_coordinates(num_rows, num_cols);

            if grid_point.row.abs() == 3 || grid_point.col.abs() == 3 {
                break 'step_loop;
            }
        }

        for point in &new_points {
            set_step_data(&mut step_data, point, step_count, scale);
        }

        prev_points = points;
        points = new_points;

        step_count += 1;
    }

    process_step_data(&step_data, grid, scale)
}

fn set_step_data(step_data: &mut Vec<Vec<i32>>, point: &Point, step: i32, scale: usize) {
    let num_rows = step_data.len() as isize;
    let num_cols = step_data[0].len() as isize;

    let scale = scale as isize;

    let row_bias = num_rows / scale * ((scale - 1) / 2);
    let col_bias = num_cols / scale * ((scale - 1) / 2);

    let row = point.row + row_bias;
    let col = point.col + col_bias;

    assert!(row >= 0 && row < num_rows);
    assert!(col >= 0 && col < num_cols);

    let row = row as usize;
    let col = col as usize;

    assert!(step_data[row][col] == -1);

    step_data[row][col] = step;
}

fn process_step_data(
    step_data: &Vec<Vec<i32>>,
    grid: &CharGrid,
    scale: usize,
) -> Vec<Vec<(usize, usize)>> {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut data = vec![vec![(0, 0); scale]; scale];

    for ri in 0..scale {
        for ci in 0..scale {
            let point = Point::new((ri * num_rows) as isize, (ci * num_cols) as isize);

            data[ri][ci] = process_tile(step_data, grid, &point);
        }
    }

    data
}

fn process_tile(step_data: &Vec<Vec<i32>>, grid: &CharGrid, point: &Point) -> (usize, usize) {
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    let mut counts = HashMap::new();

    for ri in 0..num_rows {
        for ci in 0..num_cols {
            if grid[ri][ci] == '#' {
                continue;
            }

            let val = step_data[point.row as usize + ri][point.col as usize + ci];

            counts.entry(val).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let mut evens = 0;
    let mut odds = 0;

    counts
        .iter()
        .filter(|(&step, _)| step >= 0)
        .for_each(|(step, count)| {
            if step % 2 == 0 {
                evens += count;
            } else {
                odds += count;
            }
        });

    (evens, odds)
}

fn find_far_reachable_plots(
    grid_data: &Vec<Vec<(usize, usize)>>,
    starting_point: &Point,
    grid_len: usize,
    max_steps: usize,
) -> usize {
    // Radius gives how many "grids" away from the starting grid that the steps spread to.
    let radius = (max_steps - starting_point.row as usize) / grid_len;

    let select_0 = (max_steps % 2 == 1) == (radius % 2 == 1);

    // Total reachable plots is the sum of the 4 inner and outer diagonals, the 4 corners, and the full inside grids.
    let outer_diagonal = if select_0 {
        grid_data[0][1].0 + grid_data[0][3].0 + grid_data[4][1].0 + grid_data[4][3].0
    } else {
        grid_data[0][1].1 + grid_data[0][3].1 + grid_data[4][1].1 + grid_data[4][3].1
    };

    let inner_diagonal = if select_0 {
        grid_data[1][1].0 + grid_data[1][3].0 + grid_data[3][1].0 + grid_data[3][3].0
    } else {
        grid_data[1][1].1 + grid_data[1][3].1 + grid_data[3][1].1 + grid_data[3][3].1
    };

    let corners = if select_0 {
        grid_data[0][2].0 + grid_data[2][4].0 + grid_data[4][2].0 + grid_data[2][0].0
    } else {
        grid_data[0][2].1 + grid_data[2][4].1 + grid_data[4][2].1 + grid_data[2][0].1
    };

    let (full_inside_even, full_inside_odd) = if select_0 {
        (grid_data[2][2].0, grid_data[2][2].1)
    } else {
        (grid_data[2][2].1, grid_data[2][2].0)
    };

    let reachable_plots = radius * outer_diagonal
        + (radius - 1) * inner_diagonal
        + corners
        + radius * radius * full_inside_odd
        + (radius - 1) * (radius - 1) * full_inside_even;

    reachable_plots
}

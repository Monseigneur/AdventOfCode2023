use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day23/input.txt").unwrap();

    utilities::print_results(23, || part_1(&contents), || part_2(&contents));
}

type CharGrid = Vec<Vec<char>>;
type Graph = HashMap<Point, Vec<(Point, usize)>>;

// Given a map of hiking trails, find the longest hike from the start to the end without going back
// over tiles already visited.
fn part_1(data: &str) -> usize {
    let grid: CharGrid = data.lines().map(|line| line.chars().collect()).collect();

    let (start, end) = find_ends(&grid);

    // The grid can be thought of as a graph, where the paths are edges, and then any location where there
    // is more than 1 way out is a node. Since there are the sliding points, the graph becomes a DAG. First
    // walk through the grid to find the lengths of all edges, and then find the longest path.

    let graph = build_graph(&grid, &start);
    let sorted = topological_sort(&graph);

    find_longest_path(&graph, sorted, &start, &end)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn direction_from(&self, other: &Point) -> Direction {
        let row_delta = self.row as isize - other.row as isize;
        let col_delta = self.col as isize - other.col as isize;

        match (row_delta, col_delta) {
            (1, 0) => Direction::Down,
            (0, 1) => Direction::Right,
            (-1, 0) => Direction::Up,
            (0, -1) => Direction::Left,
            _ => panic!("Illegal distance from other point!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn find_ends(grid: &CharGrid) -> (Point, Point) {
    let start_row = 0;
    let start_col = grid[start_row].iter().position(|&c| c == '.').unwrap();

    let end_row = grid.len() - 1;
    let end_col = grid[end_row].iter().position(|&c| c == '.').unwrap();

    (
        Point::new(start_row, start_col),
        Point::new(end_row, end_col),
    )
}

fn get_neighbors(grid: &CharGrid, current: &Point, direction: &Direction) -> Vec<(Point, bool)> {
    let mut neighbors = vec![];

    // Up
    if current.row > 0 && direction != &Direction::Down {
        let tile = grid[current.row - 1][current.col];

        if tile != '#' {
            neighbors.push((Point::new(current.row - 1, current.col), tile != 'v'));
        }
    }

    // Right
    if current.col < grid[0].len() - 1 && direction != &Direction::Left {
        let tile = grid[current.row][current.col + 1];

        if tile != '#' {
            neighbors.push((Point::new(current.row, current.col + 1), tile != '<'));
        }
    }

    // Down
    if current.row < grid.len() - 1 && direction != &Direction::Up {
        let tile = grid[current.row + 1][current.col];

        if tile != '#' {
            neighbors.push((Point::new(current.row + 1, current.col), tile != '^'));
        }
    }

    // Left
    if current.col > 0 && direction != &Direction::Right {
        let tile = grid[current.row][current.col - 1];

        if tile != '#' {
            neighbors.push((Point::new(current.row, current.col - 1), tile != '>'));
        }
    }

    neighbors
}

fn walk_path(grid: &CharGrid, start: &Point, direction: &Direction) -> (Point, usize, Direction) {
    let mut step_count = 1;

    let mut current = *start;
    let mut current_direction = *direction;

    loop {
        let neighbors = get_neighbors(grid, &current, &current_direction);

        if neighbors.len() != 1 {
            break;
        }

        let (neighbor, _) = neighbors[0];

        current_direction = neighbor.direction_from(&current);
        current = neighbor;

        step_count += 1;
    }

    (current, step_count, current_direction)
}

fn build_graph(grid: &CharGrid, start: &Point) -> Graph {
    let mut graph = HashMap::new();
    let mut paths = VecDeque::new();

    paths.push_back((*start, Direction::Down));

    let mut visited = HashSet::new();

    while let Some((node, direction)) = paths.pop_front() {
        if visited.contains(&node) {
            continue;
        }

        if !graph.contains_key(&node) {
            graph.insert(node, vec![]);
        }

        let neighbors = get_neighbors(grid, &node, &direction);

        for (neighbor, moveable) in neighbors {
            if !moveable {
                continue;
            }

            let neighbor_direction = neighbor.direction_from(&node);

            let (end_point, distance, end_direction) =
                walk_path(grid, &neighbor, &neighbor_direction);

            graph
                .entry(node)
                .and_modify(|v| v.push((end_point, distance)));

            paths.push_back((end_point, end_direction));
        }

        visited.insert(node);
    }

    graph
}

fn topological_sort(graph: &Graph) -> Vec<Point> {
    let size = graph.keys().len();

    let mut sorted = vec![];
    let mut visited = HashSet::new();

    while visited.len() != size {
        // Find an unvisited node.
        let node = graph.keys().find(|&n| !visited.contains(n)).unwrap();

        visit(graph, &node, &mut sorted, &mut visited);
    }

    sorted.reverse();

    sorted
}

fn visit(graph: &Graph, node: &Point, sorted: &mut Vec<Point>, visited: &mut HashSet<Point>) {
    if visited.contains(node) {
        return;
    }

    let neighbors = graph.get(node).unwrap();

    for (neighbor, _) in neighbors {
        visit(graph, neighbor, sorted, visited);
    }

    visited.insert(*node);
    sorted.push(*node);
}

fn find_longest_path(graph: &Graph, sorted: Vec<Point>, start: &Point, end: &Point) -> usize {
    let mut distances: HashMap<&Point, usize> = HashMap::new();

    distances.insert(start, 0);

    for node in sorted {
        let current_distance = *distances.get(&node).unwrap();

        if let Some(neighbors) = graph.get(&node) {
            for (neighbor, distance) in neighbors {
                let new_distance = current_distance + distance;

                distances
                    .entry(neighbor)
                    .and_modify(|dist| {
                        if new_distance > *dist {
                            *dist = new_distance;
                        }
                    })
                    .or_insert(new_distance);
            }
        }
    }

    *distances.get(end).unwrap()
}

// The slope parts aren't as slippery, so you can go up them. What is the longest path in this case? The graph
// is now an undirected graph.
fn part_2(data: &str) -> usize {
    let grid: CharGrid = data.lines().map(|line| line.chars().collect()).collect();

    let (start, end) = find_ends(&grid);

    let directed_graph = build_graph(&grid, &start);
    let graph = fill_graph(directed_graph);

    dfs(&graph, &start, &end, &mut HashSet::new(), 0)
}

fn fill_graph(directed_graph: Graph) -> Graph {
    let mut graph = HashMap::new();

    for (point, neighbors) in directed_graph {
        // for each point, add its neighbors, and add itself to each of it's neighbor's edges.
        for (neighbor, dist) in neighbors.iter() {
            let value = (point, *dist);
            graph
                .entry(*neighbor)
                .and_modify(|v: &mut Vec<(Point, usize)>| v.push(value))
                .or_insert(vec![value]);
        }

        graph
            .entry(point)
            .and_modify(|v| v.extend(neighbors.iter()))
            .or_insert(neighbors);
    }

    graph
}

fn dfs(
    graph: &Graph,
    current: &Point,
    end: &Point,
    visited: &mut HashSet<Point>,
    dist: usize,
) -> usize {
    if current == end {
        return dist;
    }

    let mut end_distances = vec![];

    // Get neighbors of current and try them all if they haven't been visited yet.
    let neighbors = graph.get(current).unwrap();

    for (neighbor, neighbor_dist) in neighbors {
        if visited.contains(neighbor) {
            continue;
        }

        visited.insert(*neighbor);
        end_distances.push(dfs(graph, neighbor, end, visited, dist + neighbor_dist));
        visited.remove(neighbor);
    }

    *end_distances.iter().max().unwrap_or(&0)
}

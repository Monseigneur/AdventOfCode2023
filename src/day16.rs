use std::hash::Hash;
use std::{collections::HashSet, collections::VecDeque, fs};

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day16/input.txt").unwrap();

    utilities::print_results(16, || part_1(&contents), || part_2(&contents));
}

type CharGrid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    col: usize,
    direction: u8,
}

impl Node {
    const NORTH: u8 = 0;
    const EAST: u8 = 1;
    const SOUTH: u8 = 2;
    const WEST: u8 = 3;

    fn new(row: usize, col: usize, direction: u8) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }
}

// Starting in the top left, the light beam moves right. Hitting splitters or mirrors causes it to change
// directions and branch. In the end, calculate the number of tiles that are covered by a beam.
fn part_1(data: &str) -> usize {
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<CharGrid>();

    process_beams(&data, Node::new(0, 0, Node::EAST))
}

fn process_beams(data: &CharGrid, starting_node: Node) -> usize {
    // Seems like I can do BFS with the stop conditions being hitting a node in the same "direction"
    // (vertical or horizontal). The reason being that if a beam is travelling to the right, there
    // shouldn't be a need to continue exploring if a beam was on the same row going left, because
    // either it would reach a splitter and proceed in the other direction, or a mirror, in which case
    // it will be the same pattern.
    let num_rows = data.len();
    let num_cols = data[0].len();

    let mut covered = vec![vec![0; num_cols]; num_rows];

    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited_nodes: HashSet<Node> = HashSet::new();

    queue.push_back(starting_node);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if visited_nodes.contains(&node) {
            continue;
        }

        covered[node.row][node.col] = 1;

        match data[node.row][node.col] {
            '.' => {
                // Keep going in the same direction
                match node.direction {
                    Node::NORTH => {
                        if node.row > 0 {
                            queue.push_back(Node::new(node.row - 1, node.col, node.direction));
                        }
                    }
                    Node::EAST => {
                        if node.col < num_cols - 1 {
                            queue.push_back(Node::new(node.row, node.col + 1, node.direction));
                        }
                    }
                    Node::SOUTH => {
                        if node.row < num_rows - 1 {
                            queue.push_back(Node::new(node.row + 1, node.col, node.direction));
                        }
                    }
                    Node::WEST => {
                        if node.col > 0 {
                            queue.push_back(Node::new(node.row, node.col - 1, node.direction));
                        }
                    }
                    _ => panic!("Illegal direction!"),
                }
            }
            '|' => {
                // If moving vertically, just pass through. Otherwise split.
                match node.direction {
                    Node::NORTH => {
                        if node.row > 0 {
                            queue.push_back(Node::new(node.row - 1, node.col, node.direction));
                        }
                    }
                    Node::EAST | Node::WEST => {
                        // Move up and down
                        if node.row > 0 {
                            queue.push_back(Node::new(node.row - 1, node.col, Node::NORTH));
                        }

                        if node.row < num_rows - 1 {
                            queue.push_back(Node::new(node.row + 1, node.col, Node::SOUTH));
                        }
                    }
                    Node::SOUTH => {
                        if node.row < num_rows - 1 {
                            queue.push_back(Node::new(node.row + 1, node.col, node.direction));
                        }
                    }
                    _ => panic!("Illegal direction!"),
                }
            }
            '-' => {
                // If moving horizontally, just pass through. Otherwise split.
                match node.direction {
                    Node::EAST => {
                        if node.col < num_cols - 1 {
                            queue.push_back(Node::new(node.row, node.col + 1, node.direction));
                        }
                    }
                    Node::NORTH | Node::SOUTH => {
                        if node.col < num_cols - 1 {
                            queue.push_back(Node::new(node.row, node.col + 1, Node::EAST));
                        }

                        if node.col > 0 {
                            queue.push_back(Node::new(node.row, node.col - 1, Node::WEST));
                        }
                    }
                    Node::WEST => {
                        if node.col > 0 {
                            queue.push_back(Node::new(node.row, node.col - 1, node.direction));
                        }
                    }
                    _ => panic!("Illegal direction!"),
                }
            }
            '/' => {
                // Mirror 90 degrees
                match node.direction {
                    Node::NORTH => {
                        // Go east
                        if node.col < num_cols - 1 {
                            queue.push_back(Node::new(node.row, node.col + 1, Node::EAST));
                        }
                    }
                    Node::EAST => {
                        // Go north
                        if node.row > 0 {
                            queue.push_back(Node::new(node.row - 1, node.col, Node::NORTH));
                        }
                    }
                    Node::SOUTH => {
                        // Go west
                        if node.col > 0 {
                            queue.push_back(Node::new(node.row, node.col - 1, Node::WEST));
                        }
                    }
                    Node::WEST => {
                        // Go south
                        if node.row < num_rows - 1 {
                            queue.push_back(Node::new(node.row + 1, node.col, Node::SOUTH));
                        }
                    }
                    _ => panic!("Illegal direction!"),
                }
            }
            '\\' => {
                // Mirror 90 degrees
                match node.direction {
                    Node::NORTH => {
                        // Go west
                        if node.col > 0 {
                            queue.push_back(Node::new(node.row, node.col - 1, Node::WEST));
                        }
                    }
                    Node::EAST => {
                        // Go south
                        if node.row < num_rows - 1 {
                            queue.push_back(Node::new(node.row + 1, node.col, Node::SOUTH));
                        }
                    }
                    Node::SOUTH => {
                        // Go east
                        if node.col < num_cols - 1 {
                            queue.push_back(Node::new(node.row, node.col + 1, Node::EAST));
                        }
                    }
                    Node::WEST => {
                        // Go north
                        if node.row > 0 {
                            queue.push_back(Node::new(node.row - 1, node.col, Node::NORTH));
                        }
                    }
                    _ => panic!("Illegal direction!"),
                }
            }
            _ => panic!("Illegal tile!"),
        }

        visited_nodes.insert(node);
    }

    covered
        .iter()
        .fold(0, |acc, row_data| acc + row_data.iter().sum::<usize>())
}

fn part_2(data: &str) -> usize {
    let data = data
        .lines()
        .map(|line| line.chars().collect())
        .collect::<CharGrid>();

    let mut max_count = 0;

    // Check columns
    for col in 0..data[0].len() {
        let a = process_beams(&data, Node::new(0, col, Node::SOUTH));
        let b = process_beams(&data, Node::new(data.len() - 1, col, Node::NORTH));

        max_count = max_count.max(a.max(b));
    }

    for row in 0..data.len() {
        let a = process_beams(&data, Node::new(row, 0, Node::EAST));
        let b = process_beams(&data, Node::new(row, data[0].len() - 1, Node::WEST));

        max_count = max_count.max(a.max(b));
    }

    max_count
}

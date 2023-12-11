use std::collections::HashMap;
use std::fs;
use std::str::Lines;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day8/input.txt").unwrap();

    utilities::print_results(8, || part_1(&contents), || part_2(&contents));
}

fn part_1(data: &str) -> usize {
    let mut line_iter = data.lines();

    let instructions = line_iter.next().unwrap();
    line_iter.next();

    let node_map = parse_nodes(line_iter);

    let mut current_node = "AAA";
    let mut count = 0;
    let mut complete = false;

    loop {
        for direction in instructions.chars() {
            match direction {
                'L' => current_node = node_map.get(current_node).unwrap().0,
                'R' => current_node = node_map.get(current_node).unwrap().1,
                _ => panic!("Illegal direction!"),
            }

            count += 1;

            if current_node == "ZZZ" {
                complete = true;
                break;
            }
        }

        if complete {
            break;
        }
    }

    count
}

fn parse_nodes(line_iter: Lines<'_>) -> HashMap<&str, (&str, &str)> {
    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in line_iter {
        let line_pieces: Vec<&str> = line.split("=").collect();

        let node_name = line_pieces[0].trim();
        let neighbors = parse_node_neighbors(line_pieces[1].trim());

        node_map.insert(node_name, neighbors);
    }

    node_map
}

fn parse_node_neighbors(neighbors: &str) -> (&str, &str) {
    let pieces: Vec<&str> = neighbors.split(",").collect();

    (
        pieces[0].strip_prefix("(").unwrap().trim(),
        pieces[1].strip_suffix(")").unwrap().trim(),
    )
}

// This was tricky. Originally I started with finding each start and advancing one at a time from there
// but that was taking forever with no end in sight.
fn part_2(data: &str) -> usize {
    let mut line_iter = data.lines();

    let instructions = line_iter.next().unwrap();
    line_iter.next();

    let node_map = parse_nodes(line_iter);

    let current_nodes: Vec<&&str> = node_map.keys().filter(|s| s.ends_with('A')).collect();

    let mut results = vec![];
    for node in &current_nodes {
        results.push(find_end(instructions, &node_map, &node, current_nodes.len()));
    }

    let mut mult = 1;
    for result in results {
        mult = lcm(mult, result);
    } 

    mult
}

fn gcd(a: usize, b: usize) -> usize {
    let mut first = a;
    let mut second = b;

    while first != second {
        if first > second {
            first = first - second;
        } else {
            second = second - first;
        }
    }
    
    first
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// For a given instruction and starting node, find all end nodes that it reaches. Note that if the same end
// node is reached at the same position through the instruction, then the entire thing is a cycle and can
// exit early (I got this from the subreddit). Analysis of the data showed that each starting location only
// ever reaches a single end node and cycles through it.
fn find_end(instructions: &str, node_map: &HashMap<&str, (&str, &str)>, start: &str, max_ends: usize) -> usize {
    let mut end_map: HashMap<&str, usize> = HashMap::new();
    let mut end_instruction_pos: HashMap<&str, usize> = HashMap::new();

    let mut count = 0;

    let mut current_node = start;

    'outer: loop {
        for (i, direction) in instructions.char_indices() {
            let new_node = match direction {
                'L' => &node_map.get(current_node).unwrap().0,
                'R' => &node_map.get(current_node).unwrap().1,
                _ => panic!("Illegal direction!"),
            };

            count += 1;
            current_node = new_node;  

            // Check exit conditions.
            if new_node.ends_with('Z') {
                if !end_map.contains_key(new_node) {
                    end_map.insert(&new_node, count);
                }

                if end_instruction_pos.contains_key(new_node) {
                    if end_instruction_pos.get(new_node).unwrap() == &i {
                        break 'outer;
                    }
                } else {
                    end_instruction_pos.insert(&new_node, i);
                }
            }          

            if end_map.keys().len() == max_ends {
                break 'outer;
            }
        }
    }

    assert!(end_map.values().len() == 1);

    *end_map.values().next().unwrap()
}

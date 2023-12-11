use std::collections::HashMap;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day8/input.txt").unwrap();

    utilities::print_results(8, || part_1(&contents), || part_2(&contents));
}

fn part_1(data: &str) -> usize {
    let mut line_iter = data.lines();

    let instructions = line_iter.next().unwrap();
    line_iter.next();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();
    for line in line_iter {
        let line_pieces: Vec<&str> = line.split("=").collect();

        let node_name = line_pieces[0].trim();
        let neighbors = parse_node_neighbors(line_pieces[1].trim());

        node_map.insert(node_name, neighbors);
    }

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

fn parse_node_neighbors(neighbors: &str) -> (&str, &str) {
    let pieces: Vec<&str> = neighbors.split(",").collect();

    (
        pieces[0].strip_prefix("(").unwrap().trim(),
        pieces[1].strip_suffix(")").unwrap().trim(),
    )
}

fn part_2(data: &str) -> usize {
    0
}

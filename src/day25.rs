use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use utilities;

pub fn run() {
    utilities::run_puzzle(25, false, part_1, part_2)
}

type Graph = HashMap<String, HashSet<String>>;

// Given a wiring diagram of "module: <other modules>", where connections are undirectional, cut 3 connections
// such that the result is 2 disconnected groups. Return the product of the group sizes.
fn part_1(data: &str) -> usize {
    let graph = build_graph(data);

    // Start with some node, prepare the first set and set of other nodes
    let mut nodes = graph.keys().cloned();

    let first_node = nodes.next().unwrap();

    let mut first_set = HashSet::new();

    first_set.insert(first_node.clone());

    let mut second_set = HashSet::new();

    let unknown: HashSet<String> = nodes.collect();

    let edge_count = 3;
    for node in unknown.iter() {
        let mut used_graph = Graph::new();

        for i in 0..=edge_count {
            if let Some(path) = find_path(&graph, &used_graph, &first_node, &node) {
                // Found a path
                update_used_path(&path, &mut used_graph);

                if i == edge_count {
                    first_set.insert(node.clone());
                }
            } else {
                // No path
                if i == edge_count {
                    // On the other side
                    second_set.insert(node.clone());
                } else {
                    println!("Error! Didnt' find a path on iteration {i}!");
                }
            }
        }
    }

    first_set.len() * second_set.len()
}

fn find_path(
    graph: &Graph,
    used_graph: &Graph,
    start_node: &String,
    end_node: &String,
) -> Option<Vec<String>> {
    let mut visited = HashSet::new();
    let mut distances: HashMap<String, (u32, Option<String>)> = HashMap::new();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start_node.clone(), None));

    let mut found = false;

    while let Some((Reverse(dist), node, parent)) = queue.pop() {
        // Have a node and its distance.
        if visited.contains(&node) {
            continue;
        }

        // Haven't processed this node yet.
        distances.insert(node.clone(), (dist, parent));

        if &node == end_node {
            found = true;
            break;
        }

        // Process neighbors
        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                // Check for edges that are already used.
                if let Some(used_neighbors) = used_graph.get(&node) {
                    if used_neighbors.contains(neighbor) {
                        continue;
                    }
                }

                // Found some neighbors we could go to
                let new_dist = dist + 1;

                queue.push((Reverse(new_dist), neighbor.clone(), Some(node.clone())));
            }
        }

        visited.insert(node);
    }

    if !found {
        return None;
    }

    let mut current = Some(end_node);
    let mut path = vec![];

    while let Some(node) = current {
        path.push(node.clone());

        let (_, new_current) = distances.get(node).unwrap();

        current = new_current.as_ref();
    }

    Some(path.into_iter().rev().collect())
}

fn update_used_path(path: &Vec<String>, used_graph: &mut Graph) {
    for (src, dest) in path.iter().zip(path.iter().skip(1)) {
        used_graph
            .entry(src.clone())
            .and_modify(|adjacent| {
                adjacent.insert(dest.clone());
            })
            .or_insert(HashSet::from([dest.clone()]));

        used_graph
            .entry(dest.clone())
            .and_modify(|adjacent| {
                adjacent.insert(src.clone());
            })
            .or_insert(HashSet::from([src.clone()]));
    }
}

fn build_graph(data: &str) -> Graph {
    let mut graph: Graph = HashMap::new();

    for line in data.lines() {
        let pieces = line.split(":").collect::<Vec<_>>();

        let dest = pieces[1]
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();

        let start_node = pieces[0].to_string();

        let dest_nodes: HashSet<String> = dest.iter().cloned().collect();

        graph
            .entry(start_node.clone())
            .and_modify(|adjacent| {
                adjacent.extend(dest_nodes.clone());
            })
            .or_insert(dest_nodes);

        for d in dest.iter() {
            graph
                .entry(d.clone())
                .and_modify(|adjacent| {
                    adjacent.insert(start_node.clone());
                })
                .or_insert(HashSet::from([start_node.clone()]));
        }
    }

    graph
}

fn part_2(_data: &str) -> usize {
    0
}

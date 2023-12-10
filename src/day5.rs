use std::fs;
use std::str::Lines;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day5/example.txt").unwrap();

    utilities::print_results(5, || part_1(&contents), || part_2(&contents));
}

// Example data:
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// The map lines are <DEST_START> <SOURCE_START> <LEN>, and it's linear in that range.
// ex: 50 98 2
//     DEST_START=50, SOURCE_START=98, LEN=2 ->
//      98 <= seed <= 99 -> seed - 98 + 50
//      seed 98 -> soil 50
//      seed 99 -> soil 51
//     52 50 48
//     DEST_START=52, SOURCE_START=50, LEN=48 ->
//      50 <= seed <= 97 -> seed - 50 + 52
//      seed 50 -> soil 52
//      ...
//      seed 97 -> soil
//     If doesn't match a range, then DEST = SOURCE
//      SOURCE_START <= seed <= SOURCE_START + LEN - 1 -> seed - SOURCE_START + DEST_START

fn part_1(data: &str) -> isize {
    let mut line_iter = data.lines();

    let seeds = parse_seeds(line_iter.next().unwrap());
    line_iter.next();

    let maps = parse_data(line_iter);

    let min = seeds
        .iter()
        .map(|seed| apply_maps(*seed, &maps))
        .min()
        .unwrap();

    min
}

fn parse_seeds(line: &str) -> Vec<isize> {
    let seed_numbers = line.split(":").last().unwrap();

    let seeds: Vec<isize> = seed_numbers
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap())
        .collect();

    seeds
}

fn parse_data(mut line_iter: Lines<'_>) -> Vec<Map> {
    let mut maps = vec![];

    let mut map_started = false;
    let mut map_lines = vec![];
    while let Some(line) = line_iter.next() {
        if line.contains("map") {
            map_started = true;

            continue;
        }

        if line.is_empty() {
            maps.push(Map::new(&map_lines));
            map_lines.clear();

            map_started = false;

            continue;
        }

        if !map_started {
            continue;
        }

        map_lines.push(line);
    }

    if !map_lines.is_empty() {
        maps.push(Map::new(&map_lines));
    }

    maps
}

#[derive(Debug)]
struct Range {
    start: isize,
    len: isize,
    delta: isize,
}

impl Range {
    fn new(dest: isize, start: isize, len: isize) -> Self {
        Self {
            start,
            len,
            delta: dest - start,
        }
    }

    fn contains(&self, value: isize) -> bool {
        value >= self.start && value < self.start + self.len
    }

    fn apply(&self, value: isize) -> isize {
        value + self.delta
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

impl Map {
    fn new(lines: &Vec<&str>) -> Self {
        let mut ranges = Vec::new();

        for line in lines {
            let pieces = line.split_ascii_whitespace().collect::<Vec<&str>>();

            assert!(pieces.len() == 3);

            let pieces: Vec<isize> = pieces.iter().map(|s| s.parse::<isize>().unwrap()).collect();

            ranges.push(Range::new(pieces[0], pieces[1], pieces[2]));
        }

        Self { ranges }
    }

    fn map(&self, input: isize) -> isize {
        for range in &self.ranges {
            if range.contains(input) {
                let result = range.apply(input);
                return result;
            }
        }

        input
    }
}

fn apply_maps(seed: isize, maps: &Vec<Map>) -> isize {
    let mut result = seed;

    for map in maps {
        result = map.map(result);
    }

    result
}

fn part_2(data: &str) -> isize {
    let mut line_iter = data.lines();

    let seeds = parse_seeds_v2(line_iter.next().unwrap());
    line_iter.next();

    let maps = parse_data(line_iter);

    let min = seeds
        .iter()
        .map(|seed| apply_maps(*seed, &maps))
        .min()
        .unwrap();

    min
}

fn parse_seeds_v2(line: &str) -> Vec<isize> {
    let seed_line = line.split(":").last().unwrap();

    let mut seed_numbers = seed_line
        .split_ascii_whitespace()
        .map(|s| s.parse::<isize>().unwrap());

    let mut seeds = vec![];

    while let Some(n) = seed_numbers.next() {
        let length = seed_numbers.next().unwrap();

        for i in n..(n + length) {
            seeds.push(i);
        }
    }

    seeds
}

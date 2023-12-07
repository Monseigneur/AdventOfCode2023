use std::{collections::HashMap, collections::HashSet, fs};

pub fn run() {
    let contents = fs::read_to_string("test_files/day3/input.txt").unwrap();

    let part_1_result = part_1(&contents);
    let part_2_result = part_2(&contents);

    println!("[Day 3]: part 1: {part_1_result}, part 2: {part_2_result}");
}

fn part_1(data: &str) -> u32 {
    // Can walk through rows r and col c, when finding a symbol (or maybe !number && !period),
    // then a symbol is there. Mark the area around (c-1 -> c+1, r-1 -> r+1) as valid. Then,
    // walk again to find numbers, only adding if the number is completed and near a symbol.

    let symbol_map = fill_map(data);

    // print map
    // println!("symbol_map {:?}", symbol_map);

    // print_map(&symbol_map);

    let sum = process_data(data, &symbol_map);

    sum
}

fn _print_map(symbol_map: &HashMap<usize, Vec<bool>>) {
    let mut v = vec![];

    for row in symbol_map.keys() {
        v.push(row);
    }

    v.sort();

    for row in v {
        println!("row {row}: {:?}", symbol_map.get(row).unwrap());
    }
}

fn process_data(data: &str, symbol_map: &HashMap<usize, Vec<bool>>) -> u32 {
    let mut sum = 0;

    for (row, line) in data.lines().enumerate() {
        let mut number = 0;
        let mut included = false;

        for (col, c) in line.char_indices() {
            if c.is_numeric() {
                number *= 10;
                number += c.to_digit(10).unwrap();

                included = included || check_map(row, col, &symbol_map);
            } else {
                // Number is not there or done.

                // if number != 0 {
                //     println!("Row {row}: {number} [{included}]");
                // }

                if included {
                    sum += number;
                }

                included = false;
                number = 0;
            }
        }

        if included {
            sum += number;
        }
    }

    sum
}

fn check_map(row: usize, col: usize, symbol_map: &HashMap<usize, Vec<bool>>) -> bool {
    if let Some(row_entry) = symbol_map.get(&row) {
        if row_entry.len() > col {
            return row_entry[col];
        }
    }

    return false;
}

fn fill_map(data: &str) -> HashMap<usize, Vec<bool>> {
    let mut symbol_map: HashMap<usize, Vec<bool>> = HashMap::new();

    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.char_indices() {
            if c.is_numeric() || c == '.' {
                continue;
            }

            update_map(row, col, &mut symbol_map);
        }
    }

    symbol_map
}

fn update_map(row: usize, col: usize, symbol_map: &mut HashMap<usize, Vec<bool>>) {
    // For a given row and col, add true to the range [-1, 1] around row and col

    // println!("  Update map row {row} col {col}");

    for r in (row - 1).max(0)..=(row + 1) {
        if !symbol_map.contains_key(&r) {
            symbol_map.insert(r, vec![]);
        }

        let row_entry = symbol_map.get_mut(&r).unwrap();

        for _ in row_entry.len()..=(col + 1) {
            row_entry.push(false);
        }

        for c in (col - 1).max(0)..=(col + 1) {
            row_entry[c] = true;
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn adjacent(&self, other: &Point) -> bool {
        // How to know if two points are adjacent?
        // abs(self.row - other.row) <= 1
        // abs(self.col - other.col) <= 1

        let adjacent_row = (self.row.max(other.row) - self.row.min(other.row)) <= 1;
        let adjacent_col = (self.col.max(other.col) - self.col.min(other.col)) <= 1;

        adjacent_row && adjacent_col
    }
}

#[derive(Debug)]
struct GearMap {
    gear_locations: HashSet<Point>,
}

impl GearMap {
    fn new(gear_locations: HashSet<Point>) -> Self {
        Self { gear_locations }
    }

    fn check_point(&self, point: Point) -> HashSet<&Point> {
        let mut adjacent_gears: HashSet<&Point> = HashSet::new();

        for gear in &self.gear_locations {
            if point.adjacent(&gear) {
                // MJMJ This feels wrong
                adjacent_gears.insert(&gear);
            }
        }

        adjacent_gears
    }
}

// Find the sum of all gear ratios, where a gear is a * with exactly 2 adjacent parts.
fn part_2(data: &str) -> u32 {
    // First find all of the gears. Build some kind of structure so that given a point, find
    // all of the gears that it is adjacent to.
    // Go through the data again, building numbers. Collect which gears they are adjacent to
    // and once complete, add to the map for those gears.
    // Go through the final map, summing the products of gears that only have 2 numbers.

    let gear_map = find_gears(data);

    // println!("gear_map {:?}", gear_map);

    let mut gear_results: HashMap<&Point, Vec<u32>> = HashMap::new();

    for gear in &gear_map.gear_locations {
        gear_results.insert(&gear, vec![]);
    }

    for (row, line) in data.lines().enumerate() {
        let mut number = 0;
        let mut adjacent_gears: HashSet<&Point> = HashSet::new();

        for (col, c) in line.char_indices() {
            if c.is_numeric() {
                number *= 10;
                number += c.to_digit(10).unwrap();

                // Gather adjacent gears
                let gears = gear_map.check_point(Point::new(row, col));

                adjacent_gears.extend(&gears);
            } else if number != 0 {
                // Number is not there or done.

                for gear in &adjacent_gears {
                    gear_results.entry(gear).and_modify(|v| v.push(number));
                }

                number = 0;
                adjacent_gears.clear();
            }
        }

        if number != 0 {
            for gear in &adjacent_gears {
                gear_results.entry(gear).and_modify(|v| v.push(number));
            }
        }
    }

    let mut sum: u32 = 0;

    // println!("gear_results {:?}", gear_results);

    for (_, adjacent_numbers) in gear_results.iter() {
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0] * adjacent_numbers[1];
        }
    }

    sum
}

fn find_gears(data: &str) -> GearMap {
    let mut gear_locations: HashSet<Point> = HashSet::new();

    for (row, line) in data.lines().enumerate() {
        for (col, c) in line.char_indices() {
            if c != '*' {
                continue;
            }

            gear_locations.insert(Point::new(row, col));
        }
    }

    GearMap::new(gear_locations)
}

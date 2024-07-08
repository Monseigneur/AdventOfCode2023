use std::collections::HashMap;
use std::collections::HashSet;

use utilities;

pub fn run() {
    utilities::run_puzzle(11, true, part_1, part_2);
}

// Given a map of the galaxy containing . for empty space and # for galaxies, find the sum of all
// of the distances between each pair of galaxies. Note that there is some space expansion, so any
// rows and columns that don't have any galaxies double in width.
fn part_1(data: &str) -> usize {
    let space = Space::new(data, 2);

    calculate_distances(&space)
}

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug)]
struct Space {
    galaxies: HashMap<usize, Point>,
    row_offsets: Vec<usize>,
    col_offsets: Vec<usize>,
}

impl Space {
    fn new(input: &str, expansion_factor: usize) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        let galaxies = Space::find_galaxies(&data);

        let (row_offsets, col_offsets) =
            Space::build_offset_tables(&galaxies, expansion_factor, data.len(), data[0].len());

        Self {
            galaxies,
            row_offsets,
            col_offsets,
        }
    }

    fn find_galaxies(data: &Vec<Vec<char>>) -> HashMap<usize, Point> {
        let mut galaxies = HashMap::new();

        let mut galaxy_index = 0;

        for (row, row_data) in data.iter().enumerate() {
            for (col, cell) in row_data.iter().enumerate() {
                if cell == &'#' {
                    galaxies.insert(galaxy_index, Point::new(row, col));
                    galaxy_index += 1;
                }
            }
        }

        galaxies
    }

    fn build_offset_tables(
        galaxies: &HashMap<usize, Point>,
        expansion_factor: usize,
        num_rows: usize,
        num_cols: usize,
    ) -> (Vec<usize>, Vec<usize>) {
        // Given all of the galaxies, find the rows and columns that don't have any galaxies and would
        // cause space expansion. If the top left (0, 0) point is fixed, then a given galaxy at (r, c)
        // will have its row adjusted by the number of expansion rows before it, and its column adjusted
        // by the number of expansion columns before it. Find the space expansion rows and columns by
        // the indexes not present in the set of galaxy rows and columns, and build the offset tables.

        let mut galaxy_rows = HashSet::new();
        let mut galaxy_cols = HashSet::new();

        for (_, location) in galaxies {
            galaxy_rows.insert(location.row);
            galaxy_cols.insert(location.col);
        }

        // The expansion factor gives how much the empty row or column becomes (2x, 10x, etc). Since the
        // row or column is "replaced" by that many rows or columns, and it scales from 1, the additional
        // rows or columns are given by expansion_factor - 1 (2x -> 1 additional rows, 10x -> 9 additional
        // cols, etc).
        let additional = expansion_factor - 1;

        let mut row_offsets = vec![];
        let mut offset = 0;
        for row in 0..num_rows {
            row_offsets.push(offset);

            if !galaxy_rows.contains(&row) {
                offset += additional;
            }
        }

        let mut col_offsets = vec![];
        let mut offset = 0;
        for col in 0..num_cols {
            col_offsets.push(offset);

            if !galaxy_cols.contains(&col) {
                offset += additional;
            }
        }

        (row_offsets, col_offsets)
    }

    fn galaxy_count(&self) -> usize {
        self.galaxies.len()
    }

    fn get_galaxy_location(&self, index: usize) -> Point {
        // Using the row and column offset tables, adjust the original galaxy location.
        let original_location = self.galaxies.get(&index).unwrap();

        Point::new(
            original_location.row + self.row_offsets[original_location.row],
            original_location.col + self.col_offsets[original_location.col],
        )
    }
}

fn calculate_distances(space: &Space) -> usize {
    let mut galaxies = HashMap::new();

    for index in 0..space.galaxy_count() {
        galaxies.insert(index, space.get_galaxy_location(index));
    }

    let mut distance_sum = 0;

    // The pairs forms a triangle, half of n^2
    let galaxy_count = space.galaxy_count();
    for first_index in 0..(galaxy_count - 1) {
        let first_point = galaxies.get(&first_index).unwrap();

        for second_index in (first_index + 1)..galaxy_count {
            let second_point = galaxies.get(&second_index).unwrap();

            distance_sum += point_delta(first_point, second_point);
        }
    }

    distance_sum
}

fn point_delta(a: &Point, b: &Point) -> usize {
    (a.row.max(b.row) - a.row.min(b.row)) + (a.col.max(b.col) - a.col.min(b.col))
}

fn part_2(data: &str) -> usize {
    let space = Space::new(data, 1000000);

    calculate_distances(&space)
}

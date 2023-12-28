use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day24/input.txt").unwrap();

    utilities::print_results(24, || part_1(&contents), || part_2(&contents));
}

// Given a list of hailstone positions and velocity vectors, calculate if their paths will intersect
// in the test area. Only consider the X and Y axes.
fn part_1(data: &str) -> usize {
    let hailstones = parse_input(data);

    let test_min = 200000000000000;
    let test_max = 400000000000000;

    let mut intersection_count = 0;

    for i in 0..(hailstones.len() - 1) {
        let first = &hailstones[i];

        for j in (i + 1)..hailstones.len() {
            let second = &hailstones[j];

            if check_intersection(&first, &second, test_min, test_max) {
                intersection_count += 1
            }
        }
    }

    intersection_count
}

#[derive(Debug)]
struct Hailstone {
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,
}

impl Hailstone {
    fn new(px: f64, py: f64, vx: f64, vy: f64) -> Self {
        Self { px, py, vx, vy }
    }

    // The hailstone motion is given by the parametric equations x = px + vx * t and y = py + vy * t.

    fn calc_x(&self, t: f64) -> f64 {
        assert!(t >= 0.0);

        self.px as f64 + self.vx as f64 * t
    }

    fn calc_y(&self, t: f64) -> f64 {
        assert!(t >= 0.0);

        self.py as f64 + self.vy as f64 * t
    }

    fn calc_t(&self, x: f64) -> f64 {
        // x = px + vx * t -> t = (x - px) / vx;

        (x - self.px) / self.vx
    }
}

fn parse_input(data: &str) -> Vec<Hailstone> {
    let mut hailstones = vec![];

    for line in data.lines() {
        let pieces = line
            .split(|c: char| c == ',' || c == '@' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();

        hailstones.push(Hailstone::new(pieces[0], pieces[1], pieces[3], pieces[4]));
    }

    hailstones
}

fn check_intersection(a: &Hailstone, b: &Hailstone, test_min: isize, test_max: isize) -> bool {
    let bound_min = test_min as f64;
    let bound_max = test_max as f64;

    let ta = calc_ta(a, b);
    if ta.is_none() {
        return false;
    }

    let ta = ta.unwrap();

    let intersection_x = a.calc_x(ta);
    let intersection_y = a.calc_y(ta);

    let tb = b.calc_t(intersection_x);

    if tb < 0.0 {
        return false;
    }

    let intersect = in_bounds(intersection_x, bound_min, bound_max)
        && in_bounds(intersection_y, bound_min, bound_max);

    intersect
}

fn in_bounds(val: f64, bound_min: f64, bound_max: f64) -> bool {
    val >= bound_min && val <= bound_max
}

fn calc_ta(a: &Hailstone, b: &Hailstone) -> Option<f64> {
    // The result of heavy algebra to solve for the parameter t_a of Hailstone a
    // given the 4 parameteric equations of Hailstone a and Hailstone b.
    let numerator = b.vx * (a.py - b.py) + b.vy * (b.px - a.px);
    let denominator = b.vy * a.vx - b.vx * a.vy;

    if denominator == 0.0 {
        return None;
    }

    let fraction = numerator / denominator;

    if fraction < 0.0 {
        return None;
    }

    Some(fraction)
}

fn part_2(data: &str) -> usize {
    0
}

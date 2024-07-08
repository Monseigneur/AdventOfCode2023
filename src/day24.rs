use num::{BigRational, Signed, ToPrimitive, Zero};
use utilities;

pub fn run() {
    utilities::run_puzzle(24, true, part_1, part_2);
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
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }

    // The hailstone motion is given by the parametric equations x = px + vx * t and y = py + vy * t.

    fn calc_x(&self, t: f64) -> f64 {
        assert!(t >= 0.0);

        self.px + self.vx * t
    }

    fn calc_y(&self, t: f64) -> f64 {
        assert!(t >= 0.0);

        self.py + self.vy * t
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

        hailstones.push(Hailstone::new(
            pieces[0], pieces[1], pieces[2], pieces[3], pieces[4], pieces[5],
        ));
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

// Using the Z axis now, at what integer position is it possible to throw a rock with an
// integer velocity such that it will collide with every hailstone? Calculate the sum of
// the x, y, and z coordinates of the start point.
//
// This part was ridiculous. It took a while to get the right equations, but many iterations
// to calculate the answer without significant rounding errors or overflowing, including
// using i128 and writing a Rational number class. In the end, I had to bring in a crate to
// to rational numbers with big integers to handle overflowing. This didn't feel good.
fn part_2(data: &str) -> usize {
    let hailstones = parse_input(data);
    let equations = gather_equations(&hailstones);

    solve(&equations)
}

fn gather_equations(hailstones: &[Hailstone]) -> Vec<Vec<f64>> {
    let mut equations = vec![];

    // Probably can do better than this, like choosing values that are linearly independent.
    'outer: for ai in 0..(hailstones.len() - 1) {
        for bi in (ai + 2)..hailstones.len() {
            let a = &hailstones[ai];
            let b = &hailstones[bi];

            let xy = equations.len() % 3 != 2;

            let row = build_equation_row(a, b, xy);

            if row[equations.len()] == 0.0 {
                continue;
            }

            equations.push(row);

            if equations.len() == 6 {
                break 'outer;
            }
        }
    }

    assert!(equations.len() == 6);

    equations
}

fn build_equation_row(a: &Hailstone, b: &Hailstone, xy: bool) -> Vec<f64> {
    // (B_vy-A_vy)*R_px + (A_vx-B_vx)*R_py + (A_py-B_py)*R_vx + (B_px-A_px)*R_vy
    // = A_py*A_vx - A_px*A_vy + B_px*B_vy - B_py*B_vx
    // let px = b.vy - a.vy;
    // let py = a.vx - b.vx;
    // let vx = a.py - b.py;
    // let vy = b.px - a.px;

    // let constant = (a.py * a.vx) - (a.px * a.vy) + (b.px * b.vy) - (b.py * b.vx);

    let (px, py, pz, vx, vy, vz, constant) = if xy {
        (
            b.vy - a.vy,
            a.vx - b.vx,
            0.0,
            a.py - b.py,
            b.px - a.px,
            0.0,
            ((a.py as isize * a.vx as isize) + (b.px as isize * b.vy as isize)
                - (a.px as isize * a.vy as isize)
                - (b.py as isize * b.vx as isize)) as f64,
        )
    } else {
        (
            0.0,
            b.vz - a.vz,
            a.vy - b.vy,
            0.0,
            a.pz - b.pz,
            b.py - a.py,
            ((a.pz as isize * a.vy as isize) + (b.py as isize * b.vz as isize)
                - (a.py as isize * a.vz as isize)
                - (b.pz as isize * b.vy as isize)) as f64,
        )
    };

    vec![px, py, pz, vx, vy, vz, constant]
}

fn solve(equations: &Vec<Vec<f64>>) -> usize {
    let num_rows = equations.len();
    let num_cols = equations[0].len();

    assert!(num_rows == 6 && num_cols == 7);

    let mut data = equations
        .iter()
        .map(|v| {
            v.iter()
                .map(|&f| BigRational::from_float(f).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let mut pivot_row = 0;
    let mut pivot_col = 0;

    while pivot_row < num_rows && pivot_col < num_rows {
        let max_elem = find_max_elem(&data, pivot_row, pivot_col);

        if max_elem.1.is_zero() {
            pivot_col += 1;

            continue;
        }

        // Swap rows with the maximum element.
        swap_rows(&mut data, max_elem.0, pivot_row);

        // Adjust rows below.
        for r in (pivot_row + 1)..num_rows {
            if data[r][pivot_col].is_zero() {
                continue;
            }

            let factor = &data[r][pivot_col] / &data[pivot_row][pivot_col];

            data[r][pivot_col] = BigRational::zero();

            for c in (pivot_col + 1)..num_cols {
                let elem = data[pivot_row][c].clone();
                data[r][c] -= elem * &factor;
            }
        }

        pivot_row += 1;
        pivot_col += 1;
    }

    // Solve for each variable with back substitution.
    let mut results = vec![BigRational::zero(); num_rows];

    let last_col = num_cols - 1;
    for r in (0..num_rows).rev() {
        let mut other_value = BigRational::zero();
        for c in (r + 1)..num_rows {
            other_value += &data[r][c] * &results[c];
        }

        let value = (&data[r][last_col] - other_value) / &data[r][r];

        results[r] = value;
    }

    let px = results[0].to_f64().unwrap();
    let py = results[1].to_f64().unwrap();
    let pz = results[2].to_f64().unwrap();

    (px.round() + py.round() + pz.round()) as usize
}

fn find_max_elem(data: &Vec<Vec<BigRational>>, row: usize, col: usize) -> (usize, BigRational) {
    let num_rows = data.len();

    let mut max_elem = (row, data[row][col].abs());
    for i in (row + 1)..num_rows {
        let elem = data[i][col].abs();
        if elem > max_elem.1 {
            max_elem = (i, elem)
        }
    }

    max_elem
}

fn swap_rows(data: &mut Vec<Vec<BigRational>>, src_row: usize, dest_row: usize) {
    if src_row == dest_row {
        return;
    }

    for c in 0..data[0].len() {
        let temp = data[src_row][c].clone();

        data[src_row][c] = data[dest_row][c].clone();
        data[dest_row][c] = temp;
    }
}

// Given a hailstone A and rock R,
// (A_px - R_px) / (R_vx - A_vx) == (A_py - R_py) / (R_vy - A_vy) == (A_pz - R_pz) / (R_vz - A_vz)
//
// Using x and y
// (A_px - R_px) / (R_vx - A_vx) == (A_py - R_py) / (R_vy - A_vy)
// (A_px - R_px)(R_vy - A_vy) == (A_py - R_py)(R_vx - A_vx)
// (A_px * R_vy) - (A_px * A_vy) - (R_px * R_vy) + (R_px * A_vy) == (A_py * R_vx) - (A_py * A_vx) - (R_py * R_vx) + (R_py * A_vx)
// (R_py * R_vx) - (R_px * R_vy) = (A_py * R_vx) - (A_py * A_vx) + (R_py * A_vx) - (A_px * R_vy) + (A_px * A_vy) - (R_px * A_vy)
//
// Using another hailstone B
// (R_py * R_vx) - (R_px * R_vy) = (B_py * R_vx) - (B_py * B_vx) + (R_py * B_vx) - (B_px * R_vy) + (B_px * B_vy) - (R_px * B_vy)
//
// Setting the right sides equal
//      A_py*R_vx - A_py*A_vx + R_py*A_vx - A_px*R_vy + A_px*A_vy - R_px*A_vy
//    = B_py*R_vx - B_py*B_vx + R_py*B_vx - B_px*R_vy + B_px*B_vy - R_px*B_vy
//
// Subtracting the right side and factoring out the Rock components
// (A_py-B_py)*R_vx - A_py*A_vx + B_py*B_vx + (A_vx-B_vx)*R_py - (A_px-B_px)*R_vy + A_px*A_vy - B_px*B_vy - (A_vy-B_vy)*R_px = 0
//
// Rearrange Rock components on the left
// (B_vy-A_vy)*R_px + (A_vx-B_vx)*R_py + (A_py-B_py)*R_vx + (B_px-A_px)*R_vy = A_py*A_vx - A_px*A_vy + B_px*B_vy - B_py*B_vx
//
// Can repeat for other combinations
// A: 19, 13, 30 @ -2,  1, -2
// B: 18, 19, 22 @ -1, -1, -2
// C: 20, 25, 34 @ -2, -2, -4
// D: 12, 31, 28 @ -1, -2, -1
// E: 20, 19, 15 @  1, -5, -3
//
// A,B: (19, 13, 30 @ -2,  1, -2) vs (18, 19, 22 @ -1, -1, -2)
// (-1 - 1)R_px + (-2 - -1)R_py + (13 - 19)R_vx + (18 - 19)R_vy = (13 * -2) - (19 * 1) + (18 * -1) - (19 * -1)
// -2R_px - R_py - 6R_vx - R_vy = -26 - 19 - 18 + 19 = -44
//
// With expected rock data: 24, 13, 10 @ -3, 1, 2
// -2(24) - 13 - 6(-3) - 1 == -44
// -48 - 13 + 18 -1 == -44
//
// A,C (19, 13, 30 @ -2,  1, -2) vs (20, 25, 34 @ -2, -2, -4)
// (-2 - 1)R_px + (-2 - -2)R_py + (13 - 25)R_vx + (20 - 19)R_vy = (13 * -2) - (19 * 1) + (20 * -2) - (25 * -2)
// -3R_px + 0R_py - 12R_vx + R_vy = -26 - 19 - 40 + 50 = -35

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
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hailstone {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Self { px, py, pz, vx, vy, vz }
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

        hailstones.push(Hailstone::new(pieces[0], pieces[1], pieces[2], pieces[3], pieces[4], pieces[5]));
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
fn part_2(data: &str) -> usize {
    let hailstones = parse_input(data);

    let equations_xy = gather_equations(&hailstones);
    let (px, py, pz, vx, vy, vz) = solve(&equations_xy);

    // let equations_yz = gather_equations(&hailstones, false);
    // let (py2, pz, vy2, vz) = solve(&equations_yz);

    println!("results px: {px} py: {py} pz: {pz} vx: {vx} vy: {vy} vz: {vz}");
    // println!("results py2: {py2} pz: {pz} vy2: {vy2} vz: {vz}");

    let result = (px + py + pz) as usize;

    println!("result {result}");

    // assert!(py == py2);
    // assert!(vy == vy2);

    let (a, b, c, d, e, f) = solve_v2(&equations_xy);
    println!("solve v2 px: {a} py: {b} pz: {c} vx: {d} vy: {e} vz: {f}");

    let equations = gather_equations(&hailstones);
    // let (a, b, c, d, e, f) = solve_v3(&equations);
    // println!("solve v3 px: {a} py: {b} pz: {c} vx: {d} vy: {e} vz: {f}");

    // (px + py + pz) as usize

    // (a + b + c) as usize

    // solve_v4(data)

    let (a, b, c, d, e, f) = solve_v5(&equations);
    println!("solve v5 px: {a} py: {b} pz: {c} vx: {d} vy: {e} vz: {f}");



    // 566373506408013 is too low
    // 566373506408009
    // 566373506408022 is too high
    // 566373506408017 got it
    0
}

fn gather_equations(hailstones: &[Hailstone]) -> Vec<Vec<f64>> {
    let mut equations = vec![];

    'outer: for ai in 0..(hailstones.len() - 1) {
        for bi in (ai + 2)..hailstones.len() {
            let a = &hailstones[ai];
            let b = &hailstones[bi];

            let xy = equations.len() % 3 != 2;

            let row = build_equation_row(a, b, xy);

            if row[equations.len()] == 0.0 {
                continue;
            }

            println!("using equations between {ai} and {bi}");

            equations.push(row);

            if equations.len() == 6 {
                break 'outer;
            }

            // continue 'outer;
        }
    }

    assert!(equations.len() == 6);

    equations
}

fn gather_equations_v2(hailstones: &[Hailstone]) -> Vec<Vec<f64>> {
    let mut equations = vec![];

    let ai = 0;
    let bi = hailstones.len() - 1;
    let ci = bi / 2;

    let a = &hailstones[ai];
    let b = &hailstones[bi];
    let c = &hailstones[ci];

    equations.push(build_equation_row(a, b, true));
    equations.push(build_equation_row(a, b, false));

    equations.push(build_equation_row(b, c, true));
    equations.push(build_equation_row(b, c, false));

    equations.push(build_equation_row(a, c, true));
    equations.push(build_equation_row(a, c, false));

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
                - (a.px as isize * a.vy as isize) - (b.py as isize * b.vx as isize)) as f64,
            // ((a.py as usize * a.vx as usize) - (a.px as usize * a.vy as usize)
            //     + (b.px as usize * b.vy as usize) - (b.py as usize * b.vx as usize)) as f64,
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
                - (a.py as isize * a.vz as isize) - (b.pz as isize * b.vy as isize)) as f64,
            // ((a.pz as usize * a.vy as usize) - (a.py as usize * a.vz as usize)
            //     + (b.py as usize * b.vz as usize) - (b.pz as usize * b.vy as usize)) as f64,
        )
    };

    vec![px, py, pz, vx, vy, vz, constant]
}

fn solve(equations: &Vec<Vec<f64>>) -> (f64, f64, f64, f64, f64, f64) {
    let num_rows = equations.len();
    let num_cols = equations[0].len();

    assert!(num_rows == 6 && num_cols == 7);

    let mut data = equations.clone();

    // for c in 0..num_cols {
    //     for r in 0..num_rows {
    //         // [0][0] -> [1][0] -> ...
    //         if r == c {

    //         }
    //     }
    // }

    print_equations(&data, "start");

    // Handle column 0
    // let value = data[0][0];
    // scale_row(&mut data, 0, 1.0 / value);

    // for r in 1..num_rows {
    //     let value = data[r][0];

    //     if value == 0.0 {
    //         continue;
    //     }

    //     subtract_row(&mut data, r, 0, value);
    // }

    let last_col = num_cols - 1;

    for c in 0..last_col {
        modify_column(&mut data, c);
    }

    print_equations(&data, "after modifying columns");

    (data[0][last_col], data[1][last_col], data[2][last_col], data[3][last_col], data[4][last_col], data[5][last_col])
}

fn scale_row(data: &mut Vec<Vec<f64>>, row: usize, factor: f64) {
    for c in 0..data[0].len() {
        if data[row][c] == 0.0 {
            continue;
        }

        data[row][c] = data[row][c] * factor;
    }
}

fn subtract_row(data: &mut Vec<Vec<f64>>, dest_row: usize, src_row: usize, factor: f64) {
    for c in 0..data[0].len() {
        data[dest_row][c] = data[dest_row][c] - factor * data[src_row][c];
    }
}

fn modify_column(data: &mut Vec<Vec<f64>>, col: usize) {
    assert!(col < data[0].len());
    assert!(col < data.len());

    // First scale the matching row
    let value = data[col][col];
    scale_row(data, col, 1.0 / value);

    // Subtract the other rows.
    for r in 0..data.len() {
        if r == col {
            continue;
        }

        let value = data[r][col];

        if value == 0.0 {
            continue;
        }

        subtract_row(data, r, col, value);
    }

    print_equations(data, &format!("modified {}", col));
}

fn solve_v2(equations: &Vec<Vec<f64>>) -> (f64, f64, f64, f64, f64, f64) {
    let num_rows = equations.len();
    let num_cols = equations[0].len();

    assert!(num_rows == 6 && num_cols == 7);

    let mut data = equations.clone();

    print_equations(&data, "before solve v2");

    let mut pivot = (0, 0);
    let end = (num_rows, num_rows);

    while pivot < end {
        // Find the max in the column from pivot.row to end
        // let mut max_elem = (pivot.0, data[pivot.0][pivot.1].abs());
        // for i in (pivot.0 + 1)..num_rows {
        //     let elem = data[i][pivot.1].abs();
        //     if elem > max_elem.1 {
        //         max_elem = (i, elem)
        //     }
        // }

        let max_elem = find_max_elem(&mut data, pivot.0, pivot.1);

        if max_elem.1 == 0.0 {
            pivot.1 += 1;

            continue;
        }

        swap_rows(&mut data, max_elem.0, pivot.0);

        // Adjust rows below
        for i in (pivot.0 + 1)..num_rows {
            let factor = data[i][pivot.1] / data[pivot.0][pivot.1]; //max_elem.1;

            data[i][pivot.1] = 0.0;

            for j in (pivot.1 + 1)..num_cols {
                data[i][j] = data[i][j] - data[pivot.0][j] * factor;
            }
        }

        pivot = (pivot.0 + 1, pivot.1 + 1);
    }

    print_equations(&data, "after solve v2");

    // Back substitution

    for i in 0..num_rows {
        let factor = 1.0 / data[i][i];
        scale_row(&mut data, i, factor);
    }

    print_equations(&data, "after scale");

    let last_col = num_cols - 1;

    let mut results = vec![0.0; num_rows];

    for r in (0..num_rows).rev() {
        let mut other_value = 0.0;
        for c in (r + 1)..num_rows {
            other_value += data[r][c] * results[c];
        }

        let value = (data[r][last_col] - other_value) / data[r][r];

        let value = if value.signum() == 1.0 {
            value.floor()
        } else {
            -value.abs().floor()
        };

        results[r] = value;
    }

    println!("results {:?}", results);

    // (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    (data[0][last_col], data[1][last_col], data[2][last_col], data[3][last_col], data[4][last_col], data[5][last_col])
}

fn solve_v3(equations: &Vec<Vec<f64>>) -> (f64, f64, f64, f64, f64, f64) {
    let num_rows = equations.len();
    let num_cols = equations[0].len();

    assert!(num_rows == 6 && num_cols == 7);

    let mut data = equations.iter().map(|v|
        v.iter().map(|&f| Rational::new(f as i128)).collect()).collect::<Vec<Vec<Rational>>>();

    let mut pivot = (0, 0);
    let end = (num_rows, num_rows);

    print_equations_v2(&data, "start of solve_v3");

    while pivot.0 < end.0 && pivot.1 < end.1 {
        // Find the max in the column from pivot.row to end
        // let mut max_elem = (pivot.0, data[pivot.0][pivot.1].abs());
        // for i in (pivot.0 + 1)..num_rows {
        //     let elem = data[i][pivot.1].abs();
        //     if elem > max_elem.1 {
        //         max_elem = (i, elem)
        //     }
        // }

        println!("==Processing from pivot {:?}==", pivot);

        let max_elem = find_max_elem_v2(&mut data, pivot.0, pivot.1);

        if max_elem.1.is_zero() {
            pivot.1 += 1;

            continue;
        }

        // Swap rows
        swap_rows_v2(&mut data, max_elem.0, pivot.0);

        // Adjust rows below
        for r in (pivot.0 + 1)..num_rows {
            // let factor = data[i][pivot.1] / data[pivot.0][pivot.1]; //max_elem.1;
            if data[r][pivot.0].is_zero() {
                continue;
            }

            let factor = data[r][pivot.1].calc_scale(&data[pivot.0][pivot.1]);

            println!("R{r} - {} * R{} [factor: {}.calc_scale({})]", factor.get_str(), pivot.0, data[r][pivot.1].get_str(), data[pivot.0][pivot.1].get_str());

            // MJMJ something seems wrong, it prints out "Rx - 0 * Ry", but the values change. Maybe I'm looking at
            // the wrong values?
            data[r][pivot.1].set(0);

            for c in (pivot.1 + 1)..num_cols {
                data[r][c] = data[r][c].subtract(&data[pivot.0][c], &factor);
            }
        }

        print_equations_v2(&data, &format!("end of iteration for pivot {:?}", pivot));

        // Fails on the cycle with pivot = (3,3)
        pivot = (pivot.0 + 1, pivot.1 + 1);
    }

    // let x = Rational::new_from_fract(10, 16);
    // let y = Rational::new(2);
    // let z = x.calc_scale(&y);
    // let w = x.subtract(&y, &z);

    // println!("x {:?} y {:?} z {:?} -> w {:?}", x, y, z, w);

    print_equations_v2(&data, "final");

    let last_col = num_cols - 1;

    let mut results = vec![Rational::new(0); num_rows];

    for r in (0..num_rows).rev() {
        let mut other_value = Rational::new(0);
        for c in (r + 1)..num_rows {
            other_value = other_value.add(&data[r][c], &results[c]);
        }

        // let value = (data[r][last_col] - other_value) / data[r][r];

        let value = data[r][last_col].subtract(&other_value, &Rational::new(1));

        let div_factor = &data[r][r];
        let recip = Rational::new_from_fract(div_factor.denom, div_factor.numer);

        let value = value.mult(&recip);

        results[r] = value;
    }

    // println!("results {:?}", results);

    println!("results {:?}", results.iter().map(|r| r.get_str()).collect::<Vec<_>>());

    // (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)
    // (data[0][last_col], data[1][last_col], data[2][last_col], data[3][last_col], data[4][last_col], data[5][last_col])


    // (0.0, 0.0, 0.0, 0.0, 0.0, 0.0)

    (results[0].val(), results[1].val(), results[2].val(), results[3].val(), results[4].val(), results[5].val())
}

fn find_max_elem(data: &mut Vec<Vec<f64>>, row: usize, col: usize) -> (usize, f64) {
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

fn find_max_elem_v2(data: &mut Vec<Vec<Rational>>, row: usize, col: usize) -> (usize, Rational) {
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

fn swap_rows(data: &mut Vec<Vec<f64>>, src_row: usize, dest_row: usize) {
    if src_row == dest_row {
        return;
    }

    for c in 0..data[0].len() {
        let temp = data[src_row][c];

        data[src_row][c] = data[dest_row][c];
        data[dest_row][c] = temp;
    }
}

fn swap_rows_v2(data: &mut Vec<Vec<Rational>>, src_row: usize, dest_row: usize) {
    if src_row == dest_row {
        println!("R{src_row} is already the abs max");
        return;
    }

    println!("Swapping R{src_row} with R{dest_row}");

    for c in 0..data[0].len() {
        let temp = data[src_row][c];

        data[src_row][c] = data[dest_row][c];
        data[dest_row][c] = temp;
    }
}

fn print_equations(data: &Vec<Vec<f64>>, s: &str) {
    println!("{s}");
    for (r, row_data) in data.iter().enumerate() {
        println!("{:2} -> {:?}", r, row_data);
    }
}

fn print_equations_v2(data: &Vec<Vec<Rational>>, s: &str) {
    println!("{s}");
    for (r, row_data) in data.iter().enumerate() {
        let row_data = row_data.iter().map(|r| r.get_str()).collect::<Vec<_>>();
        println!("{:2} -> {:?}", r, row_data);
    }
}

// 1/17/2024: even with a Rational class using 128 bit integers, I still can't get it to work. The issue is overflow. How can I
// overflow with 128 bit math? This problem is insane.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Rational {
    numer: i128,
    denom: i128,
}

impl Rational {
    fn new_from_fract(numer: i128, denom: i128) -> Self {
        assert!(denom != 0);

        let mut r = Self { numer, denom };

        r.reduce();

        r
    }

    fn new(val: i128) -> Self {
        Rational::new_from_fract(val, 1)
    }

    fn reduce(&mut self) {
        if self.numer.signum() != self.denom.signum() {
            self.numer = -self.numer.abs();
        } else {
            self.numer = self.numer.abs();
        }

        self.denom = self.denom.abs();

        if self.denom == 1 {
            return;
        }

        if self.numer == 1 {
            return;
        }

        if self.numer == 0 {
            self.denom = 1;
            return;
        }

        let factor = Rational::gcd(self.numer.abs(), self.denom);

        self.numer /= factor;
        self.denom /= factor;
    }

    // fn gcd(a: isize, b: isize) -> isize {
    //     let mut first = a;
    //     let mut second = b;

    //     while first != second {
    //         if first > second {
    //             first = first - second;
    //         } else {
    //             second = second - first;
    //         }
    //     }

    //     first
    // }

    fn gcd(a: i128, b: i128) -> i128 {
        let mut first = a;
        let mut second = b;

        while (first % second) > 0 {
            let temp = first % second;
            first = second;
            second = temp;
        }

        second
    }

    fn lcm(a: i128, b: i128) -> i128 {
        a / Rational::gcd(a, b) * b
    }

    fn calc_scale(&self, other: &Rational) -> Self {
        // a.calc_scale(b) = a/b

        let other_recip = Rational::new_from_fract(other.denom, other.numer);

        self.mult(&other_recip)

        // (a/b) / (c/d) = ad/bc // 1/2 / 1/4 = 4/2 = 2
        // Rational::new_from_fract(self.numer * other.denom, self.denom * other.numer)
    }

    fn subtract(&self, other: &Rational, scale: &Rational) -> Self {
        let scaled = other.mult(&scale);

        if self.denom == scaled.denom {
            return Rational::new_from_fract(self.numer - scaled.numer, self.denom);
        }

        let lcm = Rational::lcm(self.denom, scaled.denom);
        let lhs_numer = self.numer * (lcm / self.denom);
        let rhs_numer = scaled.numer * (lcm / scaled.denom);

        Rational::new_from_fract(lhs_numer - rhs_numer, lcm)
    }

    fn add(&self, other: &Rational, scale: &Rational) -> Self {
        let scaled = other.mult(&scale);

        if self.denom == scaled.denom {
            return Rational::new_from_fract(self.numer + scaled.numer, self.denom);
        }

        let lcm = Rational::lcm(self.denom, scaled.denom);
        let lhs_numer = self.numer * (lcm / self.denom);
        let rhs_numer = scaled.numer * (lcm / scaled.denom);

        Rational::new_from_fract(lhs_numer + rhs_numer, lcm)
    }

    fn mult(&self, other: &Rational) -> Self {
        // a/b * c/d == (a/gcd_ad) * (c/gcd_bc) / ((d/gcd_ad) * (b/gcd_bc))
        if self.is_zero() || other.is_zero() {
            return Rational::new_from_fract(0, 1);
        }

        let negative = self.numer.signum() != other.numer.signum();

        let gcd_ad = Rational::gcd(self.numer.abs(), other.denom);
        let gcd_bc = Rational::gcd(self.denom, other.numer.abs());

        let numer = (self.numer.abs() / gcd_ad) * (other.numer.abs() / gcd_bc);
        let denom = (self.denom / gcd_bc) * (other.denom / gcd_ad);

        let numer = if negative {-numer} else {numer};

        Rational::new_from_fract(numer, denom)
    }

    fn set(&mut self, val: i128) {
        self.numer = val;
        self.denom = 1;
    }

    fn is_zero(&self) -> bool {
        self.numer == 0
    }

    fn abs(&self) -> Self {
        Rational::new_from_fract(self.numer.abs(), self.denom.abs())
    }

    fn get_str(&self) -> String {
        if self.denom == 1 {
            format!("{}", self.numer)
        } else {
            format!("{}/{}", self.numer, self.denom)
        }
    }

    fn val(&self) -> f64 {
        self.numer as f64 / self.denom as f64
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // a/b >=< c/d === ad >=< bc
        // let a = self.numer * other.denom;
        // let b = self.denom * other.numer;

        if self.denom == other.denom {
            // Compare numerators
            return self.numer.partial_cmp(&other.numer);
        }

        if self.numer == other.numer {
            if self.is_zero() {
                return Some(std::cmp::Ordering::Equal);
            }

            if self.numer < 0 {
                return self.denom.partial_cmp(&other.denom);
            } else {
                return other.denom.partial_cmp(&self.denom);
            }
        }

        let self_int = self.numer / self.denom;
        let self_rem = self.numer % self.denom;

        let other_int = other.numer / other.denom;
        let other_rem = other.numer % other.denom;

        match self_int.partial_cmp(&other_int) {
            Some(std::cmp::Ordering::Equal) => {
                match (self_rem == 0, other_rem == 0) {
                    (true, true) => Some(std::cmp::Ordering::Equal),
                    (true, false) => Some(std::cmp::Ordering::Less),
                    (false, true) => Some(std::cmp::Ordering::Greater),
                    (false, false) => {
                        let self_recip = Rational::new_from_fract(self.denom, self_rem);
                        let other_recip = Rational::new_from_fract(other.denom, other_rem);
                        other_recip.partial_cmp(&self_recip)
                    }
                }
            },
            x @ _ => x,
        }

        // a.partial_cmp(&b)
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

fn solve_v4(data: &str) -> usize {
    // dot and cross product way?
    // https://topaz.github.io/paste/#XQAAAQCTCQAAAAAAAAA0m0pnuFI8c8h14kUamL+XYzBvHppm9lCBjoan1Q0sYYlqcLWQS0njG9969tZsWjQCla5prwqFlBf7NmX9kjiOCY7TX+bWvefIHPFw0kJtw27ueQjYL0mdn7Q6FDAoLATUFRdcl0NIZ7Ws0uhB+tljhHjpGsc1roo/acxal6l3MZPN/ALVENlwSdFKVmnz6EME/+g78MDO1aQ8PLZcU94Ji3CPqsDOgne3qQDPxCGIQSM/5ne9o/rfay+iN5g5flZBPyTc9wLxTYBt39aGtOJLshNTd0/FMJq5XtqBogaMLjwJ3Sx/DlVP5j+Q/eAqKcuJUAbYzIK2yEIutwdIWvWfAq9eZB5dH6a4Y7UUqYCgB0a1oZJzJ57mJBa7sB+hAmeGOjjDjaGdsrbt0kVL+nzHVzzuHElv8kZxp4nm/oi6nLKS+gfbt5v7tbjxI0vA6KNI7RTF0c7l/U0jGZinbC/TaPPPg/oicam2RfQ2HjlJwChFB6sMiEHJsXU3bP3rd6/Of6VfztN32WlUGSRCkPfyLZS+2XykU0fjYdd0gtU986YbX3tpWTduaOf+mgaaSt17D0VKdsxXV1AvaWDj+uCviXx4yTepZKV2/IpOlewmyC4njPHkcT/g364Zh5+UkgsA4aYPzAxGEUAxVEu2tb8eGJ6iwbHSsqXUdPa5sR82uDJxBfozM8avAcAgK/K40Ud7+zDdUXFjiDe4eq1hB9Qd13wI7lhY6s5bcxB5wtUkGsCBD3n+1xdk+ChVsAgBiasGl368HsiO8fvSO7g3B2VAiay5a9d5xNWjW2aDPkGzO9jJrnfYjluL162BwQX4q6xaaeR0NH4dNw5okgJXUJJGJhhbxPQ0jVbhtV4lRY6ewjZljRK4b+Un6yRmy6g21+8KtwbLoxwDDjzgmpmSypOP4ByIOsZJugqXnTl5g0wb1pAYWTzLAGRX7ReND+BE5uIHOCmBI/nxdEN5pH3TJHhBglmYOuSPmMPY2dKLbLz/n+B6KD/DX7FMlYuMzNZgfJqhQrRhq0mG4G28eM582M5GHGdZvmTfL3xKWtoiB8Xf5aSgSHvjHbuXL7yvU9S88o7fKy8uGNlaZlAWMbR8GzPRuEW8q6AepxqszNFpWNFq+TIpb1TkG7CgCCQ5PupBVRTKh5ap5yE8CjXWotfC7XtaKOSpRbTpHTNr9iQKICftUeSq1Lzse6YbwJ7r0t+9YPhiE6cPjRy+hstAPOZG55HdyQxRjeb8A3n1VnSo3uINnkDxYYhzYeELF3pmiVhDwSYhPJ9+FqAan+pTSEgpu4FXjprXy9aeJcXq1AeEaPpI/96yqPo=

    let hailstones = parse_input_v2(data);

    // solve prepare
    let (p1, v1) = &hailstones[0];

    let mut p2 = None;
    let mut v2 = None;
    let mut i2 = None;

    for i in 1..hailstones.len() {
        let (p, v) = &hailstones[i];

        if !v1.cross(v).is_zero() {
            p2 = Some(p);
            v2 = Some(v);
            i2 = Some(i);

            break;
        }
    }

    let p2 = p2.unwrap();
    let v2 = v2.unwrap();
    let i2 = i2.unwrap();

    let mut p3 = None;
    let mut v3 = None;

    for j in (i2 + 1)..hailstones.len() {
        let (p, v) = &hailstones[j];

        if !v1.cross(v).is_zero() && !v2.cross(v).is_zero() {
            p3 = Some(p);
            v3 = Some(v);

            break;
        }
    }

    let p3 = p3.unwrap();
    let v3 = v3.unwrap();

    println!("p1 {:?} v1 {:?} p2 {:?} v2 {:?} p3 {:?} v3 {:?}", p1, v1, p2, v2, p3, v3);

    // find_rock
    let (a, A) = find_plane(&p1, &v1, &p2, &v2);
    let (b, B) = find_plane(&p1, &v1, &p3, &v3);
    let (c, C) = find_plane(&p2, &v2, &p3, &v3);

    println!("a {:?} A {:?}", a, A);
    println!("b {:?} B {:?}", b, B);
    println!("c {:?} C {:?}", c, C);

    let w = lin(A, &b.cross(&c), B, &c.cross(&a), C, &a.cross(&b));
    let t = a.dot(&b.cross(&c));

    let w = Vec3::new((w.x as f64 / t as f64).round() as i128,
        (w.y as f64 / t as f64).round() as i128,
        (w.z as f64 / t as f64).round() as i128);

    println!("w {:?}", w);

    let w1 = v1.sub(&w);
    let w2 = v2.sub(&w);
    let ww = w1.cross(&w2);

    let E = ww.dot(&p2.cross(&w2));
    let F = ww.dot(&p1.cross(&w1));
    let G = p1.dot(&ww);
    let S = ww.dot(&ww);

    let rock = lin(E, &w1, -F, &w2, G, &ww);

    println!("rock {:?} S {}", rock, S);


    0
}

fn parse_input_v2(data: &str) -> Vec<(Vec3, Vec3)> {
    let mut hailstones = vec![];

    for line in data.lines() {
        let pieces = line
            .split(|c: char| c == ',' || c == '@' || c.is_ascii_whitespace())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i128>().unwrap())
            .collect::<Vec<_>>();

        let pos = Vec3::new(pieces[0], pieces[1], pieces[2]);
        let vel = Vec3::new(pieces[3], pieces[4], pieces[5]);

        hailstones.push((pos, vel));
    }

    hailstones
}

fn find_plane(p1: &Vec3, v1: &Vec3, p2: &Vec3, v2: &Vec3) -> (Vec3, i128) {
    let p12 = p1.sub(&p2);
    let v12 = v1.sub(&v2);

    let v = v1.cross(&v12);

    (v, p12.dot(&v))
}

fn lin(r: i128, a: &Vec3, s: i128, b: &Vec3, t: i128, c: &Vec3) -> Vec3 {
    let x = r * a.x + s * b.x + t * c.x;
    let y = r * a.y + s * b.y + t * c.y;
    let z = r * a.z + s * b.z + t * c.z;

    Vec3::new(x, y, z)
}

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Vec3 {
    fn new(x: i128, y: i128, z: i128) -> Self {
        Self { x, y, z, }
    }

    fn cross(&self, other: &Vec3) -> Self {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        Self { x, y, z }
    }

    fn dot(&self, other: &Vec3) -> i128 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }

    fn sub(&self, other: &Vec3) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}


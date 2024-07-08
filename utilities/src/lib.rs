use std::fs;
use std::time::Duration;
use std::time::Instant;

fn instrument<F, T>(f: F, data: &str) -> (T, Duration)
where
    F: Fn(&str) -> T,
{
    let now = Instant::now();
    let result = f(data);

    (result, now.elapsed())
}

pub fn run_puzzle<A, B, F, G>(day: usize, use_input: bool, f1: F, f2: G)
where
    F: Fn(&str) -> A,
    G: Fn(&str) -> B,
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    let file_name = if use_input { "input.txt" } else { "example.txt" };
    let file_path = format!("test_files/day{day}/{file_name}");

    let contents = fs::read_to_string(file_path).unwrap();

    let part_1 = instrument(f1, &contents);
    let part_2 = instrument(f2, &contents);

    println!(
        "[Day {day}]: part 1: {} ({:?}), part 2: {} ({:?})",
        part_1.0, part_1.1, part_2.0, part_2.1
    );
}
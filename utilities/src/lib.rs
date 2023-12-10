use std::time::Duration;
use std::time::Instant;

pub fn print_results<A, B, F, G>(day: usize, f1: F, f2: G)
where
    F: Fn() -> A,
    G: Fn() -> B,
    A: std::fmt::Display,
    B: std::fmt::Display,
{
    let part_1 = instrument(f1);
    let part_2 = instrument(f2);

    println!(
        "[Day {day}]: part 1: {} ({:?}), part 2: {} ({:?})",
        part_1.0, part_1.1, part_2.0, part_2.1
    );
}

fn instrument<F, T>(f: F) -> (T, Duration)
where
    F: Fn() -> T,
{
    let now = Instant::now();
    let result = f();

    (result, now.elapsed())
}

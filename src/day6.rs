use std::fs;

pub fn run() {
    let contents = fs::read_to_string("test_files/day6/input.txt").unwrap();

    let part_1_result = part_1(&contents);
    let part_2_result = part_2(&contents);

    println!("[Day 6]: part 1: {part_1_result}, part 2: {part_2_result}");
}

// This is an algebraic optimization problem. For a given race of time T and best distance B,
// holding the button for x seconds gives x mm/ms speed, which travels for (T - x) ms. The
// distance travelled is then given by:
//      distance = x * (T - x) = Tx - x^2
//
// In order to beat the be distance, the equation needs to be:
//      B < Tx - x^2
//  ->  -x^2 - Tx - B > 0
//
// The quadratic equation can be used to find the min and max values of x that will beat the
// best distance.

fn part_1(data: &str) -> usize {
    let races = parse_races(data);

    let mut combos = 1;
    for race in races {
        combos *= race.get_best_time_range();
    }

    combos
}

#[derive(Debug)]
struct Race {
    time: usize,
    best_distance: usize,
}

impl Race {
    fn new(time: usize, best_distance: usize) -> Self {
        Self {
            time,
            best_distance,
        }
    }

    fn get_best_time_range(&self) -> usize {
        let b = self.time as f64;
        let c = self.best_distance as f64;

        let sqrt_body: f64 = (b.powf(2.0) - 4.0 * c).sqrt();

        let min = (-b + sqrt_body) / -2.0;
        let max = (-b - sqrt_body) / -2.0;

        // Since the boat must not tie with the best time, it must be greater than the best
        // distance. In the case of a tie, move to the next integer for the min and the previous
        // integer for the max.
        let min = if min % 1.0 == 0.0 {
            min as usize + 1
        } else {
            min.ceil() as usize
        };

        let max = if max % 1.0 == 0.0 {
            max as usize - 1
        } else {
            max.floor() as usize
        };

        max - min + 1
    }
}

fn parse_races(data: &str) -> Vec<Race> {
    let mut line_iter = data.lines();

    let times = parse_numbers(line_iter.next().unwrap());
    let distances = parse_numbers(line_iter.next().unwrap());

    assert!(times.len() == distances.len());

    let mut races = vec![];
    for (time, distance) in times.iter().zip(distances.iter()) {
        races.push(Race::new(*time, *distance));
    }

    races
}

fn parse_numbers(line: &str) -> Vec<usize> {
    let number_data = line.split(":").last().unwrap();

    let numbers = number_data
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    numbers
}

fn part_2(data: &str) -> usize {
    let race = parse_races_v2(data);

    race.get_best_time_range()
}

fn parse_races_v2(data: &str) -> Race {
    let mut line_iter = data.lines();

    let time = parse_numbers_v2(line_iter.next().unwrap());
    let distance = parse_numbers_v2(line_iter.next().unwrap());

    Race::new(time, distance)
}

fn parse_numbers_v2(line: &str) -> usize {
    let number_data = line.split(":").last().unwrap();

    let numbers = number_data.split_ascii_whitespace().collect::<Vec<&str>>();

    let number = numbers.iter().fold(String::new(), |s, piece| s + piece);

    number.parse::<usize>().unwrap()
}

use std::collections::HashMap;

use utilities;

pub fn run() {
    utilities::run_puzzle(15, true, part_1, part_2);
}

// Hash each string in the comma-separated list of tokens.
fn part_1(data: &str) -> usize {
    let mut current = 0;

    for s in data.split(",") {
        current += hash_str(s);
    }

    current
}

fn hash_str(s: &str) -> usize {
    let mut current = 0;

    for c in s.chars() {
        let code = c as usize;

        current = (current + code) * 17 % 256;
    }

    current
}

type LensMap = HashMap<usize, Vec<Lens>>;

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    num: usize,
}

impl Lens {
    fn new(label: &str, num: usize) -> Self {
        let label = label.to_owned();

        Self { label, num }
    }
}

// Each token in the comma-separated input controls what happens to the numbered lens in the box at index
// of the hash. Once all lenses are added, find the sum of the focal powers by (box_num * slot_num * lens_number)
fn part_2(data: &str) -> usize {
    let mut lens_map: LensMap = HashMap::new();

    for token in data.split(",") {
        process_token(token, &mut lens_map);
    }

    calculate_focal_power(&lens_map)
}

fn process_token(token: &str, lens_map: &mut LensMap) {
    let add = token.contains('=');
    let pieces = token.split(|c| c == '-' || c == '=').collect::<Vec<&str>>();

    let label = pieces[0];
    let bucket_index = hash_str(label);

    if add {
        let lens_number = pieces[1].parse::<usize>().unwrap();

        lens_map
            .entry(bucket_index)
            .and_modify(|lenses| {
                let mut lens_index = None;

                for (index, lens) in lenses.iter_mut().enumerate() {
                    if lens.label == label {
                        lens_index = Some(index);

                        break;
                    }
                }

                if let Some(index) = lens_index {
                    lenses[index].num = lens_number;
                } else {
                    lenses.push(Lens::new(label, lens_number));
                }
            })
            .or_insert(vec![Lens::new(label, lens_number)]);
    } else {
        // Remove a lens from the bucket.
        lens_map.entry(bucket_index).and_modify(|lenses| {
            let mut lens_index = None;

            for (index, lens) in lenses.iter_mut().enumerate() {
                if lens.label == label {
                    lens_index = Some(index);
                    break;
                }
            }

            if let Some(index) = lens_index {
                lenses.remove(index);
            }
        });
    }
}

fn calculate_focal_power(lens_map: &LensMap) -> usize {
    let mut focal_power = 0;

    for bucket_index in 0..256 {
        if !lens_map.contains_key(&bucket_index) {
            continue;
        }

        let lenses = lens_map.get(&bucket_index).unwrap();

        let mut lens_powers = 0;

        for (lens_index, lens) in lenses.iter().enumerate() {
            let lens_power = (lens_index + 1) * lens.num;

            lens_powers += lens_power;
        }

        let bucket_multiplier = bucket_index + 1;

        focal_power += lens_powers * bucket_multiplier;
    }

    focal_power
}

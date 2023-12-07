use std::fs;

pub fn run() {
    let contents = fs::read_to_string("test_files/day1/input.txt").unwrap();

    let part_1_result = part_1(&contents);
    let part_2_result = part_2(&contents);

    println!("[Day 1]: part 1: {part_1_result}, part 2: {part_2_result}");
}

// Sum of first_digit_in_line * 10 + last_digit_in_line for each line
// If only one number in a line, it counts for both digits
//
// Example
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet
//
// -> 12 + 38 + 15 + 77 = 142

fn part_1(contents: &str) -> u32 {
    let mut number = 0;

    for line in contents.lines() {
        number += number_for_line(line);
    }

    number
}

fn number_for_line(line: &str) -> u32 {
    let first_digit = line
        .chars()
        .find(|c| c.is_numeric())
        .unwrap_or('0')
        .to_digit(10)
        .unwrap();
    let last_digit = line
        .chars()
        .rev()
        .find(|c| c.is_numeric())
        .unwrap_or('0')
        .to_digit(10)
        .unwrap();

    first_digit * 10 + last_digit
}

fn part_2(contents: &str) -> u32 {
    let mut number = 0;

    for line in contents.lines() {
        let num = number_for_line2(line);

        // println!("Value {num} for {line}");

        // number += number_for_line2(line);
        number += num;
    }

    number
}

fn number_for_line2(line: &str) -> u32 {
    // String may have a number or text.

    // println!("Searching [{line}]");

    let first_digit = line.char_indices().find_map(|(i, c)| {
        if c.is_numeric() {
            return Some((i, c.to_digit(10).unwrap()));
        }

        None
    });

    let last_digit = line.char_indices().rev().find_map(|(i, c)| {
        if c.is_numeric() {
            return Some((i, c.to_digit(10).unwrap()));
        }

        None
    });

    // println!("  first_digit {:?} last_digit {:?}", first_digit, last_digit);

    // Check for words, seeing if any are better than the best found digits.
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut first_best = first_digit.clone();
    let mut last_best = last_digit.clone();

    for (i, word) in words.iter().enumerate() {
        match line.find(word) {
            Some(position) => {
                // Found the word, check if it's better than what we already have.
                // first_best = match first_best {
                //     None => Some((position, i + 1)),
                //     Some(j) => {

                //     }
                // }

                let potential_pair = (position, i as u32 + 1);

                if first_best.is_none() || first_best.unwrap().0 > position {
                    first_best = Some(potential_pair);
                }
            }
            None => (),
        }

        match line.rfind(word) {
            Some(position) => {
                let potential_pair = (position, i as u32 + 1);

                last_best = match last_best {
                    None => Some(potential_pair),
                    Some(current_best) if current_best.0 < position => Some(potential_pair),
                    Some(current_best) => Some(current_best),
                };
            }
            None => (),
        }

        // println!("    after processing {word}({i}), first_best {:?}, last_best {:?}", first_best, last_best);
    }

    first_best.unwrap_or((0, 0)).1 * 10 + last_best.unwrap_or((0, 0)).1
}

use std::collections::HashMap;
use std::fs;

use utilities;

pub fn run() {
    let contents = fs::read_to_string("test_files/day19/input.txt").unwrap();

    utilities::print_results(19, || part_1(&contents), || part_2(&contents));
}

// The input contains instructions and part ratings, determine for each part if they are accepted or
// rejected.
fn part_1(data: &str) -> usize {
    let (instructions, part_ratings) = parse_input(data);

    let parts = part_ratings
        .iter()
        .map(|rating| Part::new(rating))
        .collect::<Vec<Part>>();

    let instruction_map = parse_instructions(&instructions);

    parts
        .iter()
        .map(|part| process_part(part, &instruction_map))
        .sum()
}

fn parse_input(data: &str) -> (Vec<&str>, Vec<&str>) {
    let mut instructions = vec![];
    let mut part_ratings = vec![];
    let mut part_section = false;

    for line in data.lines() {
        if line.is_empty() {
            part_section = true;
            continue;
        }

        if part_section {
            part_ratings.push(line);
        } else {
            instructions.push(line);
        }
    }

    (instructions, part_ratings)
}

#[derive(Debug)]
enum Variable {
    X,
    M,
    A,
    S,
}

impl Variable {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Illegal string"),
        }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(data: &str) -> Self {
        let values = data
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        Self {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3],
        }
    }

    fn get_val(&self, var: &Variable) -> usize {
        match var {
            Variable::X => self.x,
            Variable::M => self.m,
            Variable::A => self.a,
            Variable::S => self.s,
            _ => panic!("Illegal variable"),
        }
    }

    fn get_score(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug)]
struct Instruction {
    var: Variable,
    limit: usize,
    lt: bool,
    dest: String,
    unconditional: bool,
}

impl Instruction {
    fn new(piece: &str) -> Self {
        // Piece can be either "a<2006:qkq", or "rfg"
        if piece.contains(":") {
            let pieces = piece.split(":").collect::<Vec<&str>>();

            let function = pieces[0];
            let lt = function.contains("<");

            let function_pieces = function
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();

            Self {
                var: Variable::from_str(function_pieces[0]),
                limit: function_pieces[1].parse::<usize>().unwrap(),
                lt,
                dest: pieces[1].to_owned(),
                unconditional: false,
            }
        } else {
            Self {
                var: Variable::X,
                limit: 0,
                lt: false,
                dest: piece.to_owned(),
                unconditional: true,
            }
        }
    }

    fn apply(&self, part: &Part) -> Option<&str> {
        if self.unconditional {
            return Some(&self.dest);
        }

        let part_val = part.get_val(&self.var);

        if self.lt && part_val < self.limit {
            Some(&self.dest)
        } else if !self.lt && part_val > self.limit {
            Some(&self.dest)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct InstructionNode {
    instructions: Vec<Instruction>,
}

impl InstructionNode {
    fn new(data: &str) -> Self {
        let mut instructions = vec![];

        for instruction in data.split(",").filter(|s| !s.is_empty()) {
            instructions.push(Instruction::new(instruction));
        }

        Self { instructions }
    }

    fn apply(&self, part: &Part) -> &str {
        for instruction in &self.instructions {
            if let Some(dest) = instruction.apply(part) {
                return dest;
            }
        }

        panic!("No instruction!")
    }
}

fn parse_instructions(instructions: &[&str]) -> HashMap<String, InstructionNode> {
    let mut instruction_map = HashMap::new();

    for instruction in instructions {
        let instruction_pieces = instruction
            .split(|c| c == '}' || c == '{')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();

        instruction_map.insert(
            instruction_pieces[0].to_owned(),
            InstructionNode::new(instruction_pieces[1]),
        );
    }

    instruction_map
}

fn process_part(part: &Part, instruction_map: &HashMap<String, InstructionNode>) -> usize {
    // Start at in, go until A or R

    let mut current_instruction = "in";

    while current_instruction != "A" && current_instruction != "R" {
        current_instruction = instruction_map
            .get(current_instruction)
            .unwrap()
            .apply(part);
    }

    if current_instruction == "A" {
        part.get_score()
    } else {
        0
    }
}

fn part_2(data: &str) -> usize {
    0
}

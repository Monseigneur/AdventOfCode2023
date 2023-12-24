use std::collections::{HashMap, HashSet, VecDeque};
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

// Instead of sorting parts by the ratings, determine instead the number of combinations of ratings
// that will yield an accepted part, assuming that each of the four ratings can be 1 to 4000.
//
// The example and input data appear to be DAGs and only branch; only the A and R nodes have more than
// one other node pointing to them. This means that each node should just cut up the input range they
// receive to send to each of their neighbors, so the ranges can be pushed through to figure out what
// ranges reach the A node.
fn part_2(data: &str) -> usize {
    let (instructions, _) = parse_input(data);

    let instruction_map = parse_instructions(&instructions);

    // Do a BFS through the graph to calculate the ranges for each node and what reaches the Accepted state.
    let mut in_ranges: HashMap<&str, RatingRange> = HashMap::new();
    let mut a_ranges = vec![];

    let mut queue: VecDeque<&str> = VecDeque::new();
    let mut visited_nodes: HashSet<&str> = HashSet::new();

    let starting_node = "in";

    queue.push_back(starting_node);
    in_ranges.insert(&starting_node, RatingRange::default());

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        if visited_nodes.contains(&node) {
            continue;
        }

        let node_instructions = instruction_map.get(node).unwrap();

        // Find the neighbors and build the ranges for them.
        // For a given neighbor, there are a few ranges to consider:
        //  - The in_range for the current node.
        //  - The instruction_range containing what the instruction to the neighbor calls for.
        //  - The else_range containing the inverses of the previous instructions.

        let mut else_range = RatingRange::default();

        for instruction in node_instructions.instructions.iter() {
            let mut instruction_range = RatingRange::default();
            let mut instruction_else_range = RatingRange::default();

            if !instruction.unconditional {
                match instruction.var {
                    Variable::X => {
                        if instruction.lt {
                            // var < limit -> 1..limit
                            // else range: var >= limit -> limit..4001
                            instruction_range.x.max = instruction.limit;
                            instruction_else_range.x.min = instruction.limit;
                        } else {
                            // var > limit -> (limit + 1)..4001
                            // else range: var <= limit -> 1..(limit + 1)
                            instruction_range.x.min = instruction.limit + 1;
                            instruction_else_range.x.max = instruction.limit + 1;
                        }
                    }
                    Variable::M => {
                        if instruction.lt {
                            instruction_range.m.max = instruction.limit;
                            instruction_else_range.m.min = instruction.limit;
                        } else {
                            instruction_range.m.min = instruction.limit + 1;
                            instruction_else_range.m.max = instruction.limit + 1;
                        }
                    }
                    Variable::A => {
                        if instruction.lt {
                            instruction_range.a.max = instruction.limit;
                            instruction_else_range.a.min = instruction.limit;
                        } else {
                            instruction_range.a.min = instruction.limit + 1;
                            instruction_else_range.a.max = instruction.limit + 1;
                        }
                    }
                    Variable::S => {
                        if instruction.lt {
                            instruction_range.s.max = instruction.limit;
                            instruction_else_range.s.min = instruction.limit;
                        } else {
                            instruction_range.s.min = instruction.limit + 1;
                            instruction_else_range.s.max = instruction.limit + 1;
                        }
                    }
                }
            }

            let in_range = in_ranges.get(node).unwrap();

            let dest_range = in_range
                .intersection(&instruction_range)
                .intersection(&else_range);

            if instruction.dest == "A" {
                a_ranges.push((node, dest_range));
            } else if instruction.dest != "R" {
                in_ranges.insert(&instruction.dest, dest_range);
                queue.push_back(&instruction.dest);
            }

            else_range = else_range.intersection(&instruction_else_range);
        }

        visited_nodes.insert(node);
    }

    a_ranges.iter().map(|(_, range)| range.size()).sum()
}

#[derive(Debug)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }

    fn default() -> Self {
        Range::new(1, 4001)
    }

    fn intersection(&self, other: &Range) -> Self {
        let mut min = 0;
        let mut max = 0;

        if self.min <= other.min && other.min < self.max {
            min = other.min;
            max = self.max.min(other.max);
        } else if other.min <= self.min && self.min < other.max {
            min = self.min;
            max = self.max.min(other.max);
        }

        Self { min, max }
    }

    fn size(&self) -> usize {
        self.max - self.min
    }
}

#[derive(Debug)]
struct RatingRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl RatingRange {
    fn default() -> Self {
        let x = Range::default();
        let m = Range::default();
        let a = Range::default();
        let s = Range::default();

        Self { x, m, a, s }
    }

    fn intersection(&self, other: &RatingRange) -> Self {
        Self {
            x: self.x.intersection(&other.x),
            m: self.m.intersection(&other.m),
            a: self.a.intersection(&other.a),
            s: self.s.intersection(&other.s),
        }
    }

    fn size(&self) -> usize {
        self.x.size() * self.m.size() * self.a.size() * self.s.size()
    }
}

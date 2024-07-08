use std::collections::{HashMap, HashSet, VecDeque};

use utilities;

pub fn run() {
    utilities::run_puzzle(20, true, part_1, part_2);
}

// Given an arrangement of connected modules, calculate the product of low and high pulses that are sent
// after pushing the start button 1000 times.
fn part_1(data: &str) -> usize {
    const PUSH_BUTTON_COUNT: usize = 1000;

    let mut modules = parse_modules(data);

    count_pulses(&mut modules, PUSH_BUTTON_COUNT)
}

// I think this can be implemented by holding all of the modules, and then keeping a queue of
// pulses. While queue is not empty, pop off a pulse and apply its effects, possibly generating
// more pulses that are pushed into the queue. Keep going until the queue is empty.
#[derive(Debug)]
struct Pulse {
    start: String,
    end: String,
    high_pulse: bool,
}

impl Pulse {
    fn new(start: &str, end: &str, high_pulse: bool) -> Self {
        Self {
            start: start.to_string(),
            end: end.to_string(),
            high_pulse,
        }
    }

    fn start() -> Self {
        Self {
            start: "button".to_string(),
            end: "broadcaster".to_string(),
            high_pulse: false,
        }
    }
}

#[derive(Debug)]
struct Broadcast {
    name: String,
    destinations: Vec<String>,
    pulses_sent: usize,
}

impl Broadcast {
    fn new(name: &str, destinations: &[&str]) -> Self {
        let destinations = destinations
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        Self {
            name: name.to_string(),
            destinations,
            pulses_sent: 0,
        }
    }

    fn apply(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.pulses_sent += self.destinations.len();

        self.destinations
            .iter()
            .map(|dest| Pulse::new(&self.name, dest, pulse.high_pulse))
            .collect::<Vec<Pulse>>()
    }

    fn get_pulses_sent(&self) -> (usize, usize) {
        (self.pulses_sent, 0)
    }

    fn get_state(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    destinations: Vec<String>,
    on: bool,
    low_pulses_sent: usize,
    high_pulses_sent: usize,
}

impl FlipFlop {
    fn new(name: &str, destinations: &[&str]) -> Self {
        let destinations = destinations
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        Self {
            name: name.to_string(),
            destinations,
            on: false,
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }

    fn apply(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.high_pulse {
            return vec![];
        }

        self.on = !self.on;
        let high_pulse = self.on;

        if high_pulse {
            self.high_pulses_sent += self.destinations.len();
        } else {
            self.low_pulses_sent += self.destinations.len();
        }

        self.destinations
            .iter()
            .map(|dest| Pulse::new(&self.name, dest, high_pulse))
            .collect::<Vec<Pulse>>()
    }

    fn get_pulses_sent(&self) -> (usize, usize) {
        (self.low_pulses_sent, self.high_pulses_sent)
    }

    fn get_state(&self) -> String {
        self.name.clone() + &bool_str(self.on)
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    destinations: Vec<String>,
    inputs: Vec<String>,
    input_last_pulse: Vec<bool>,

    low_pulses_sent: usize,
    high_pulses_sent: usize,
}

impl Conjunction {
    fn new(name: &str, destinations: &[&str]) -> Self {
        let destinations = destinations
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        Self {
            name: name.to_string(),
            destinations,
            inputs: vec![],
            input_last_pulse: vec![],
            low_pulses_sent: 0,
            high_pulses_sent: 0,
        }
    }

    fn add_input(&mut self, name: &str) {
        self.inputs.push(name.to_string());
        self.input_last_pulse.push(false);
    }

    fn apply(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        // Initially remembers low pulses for all connected inputs. When receiving a pulse
        // from an input, update the last value for that input. If all inputs last sent a
        // high pulse, send a low pulse. Otherwise send a high pulse.

        let mut all_inputs_high = true;

        for (i, input) in self.inputs.iter().enumerate() {
            if input == &pulse.start {
                self.input_last_pulse[i] = pulse.high_pulse;
            }

            all_inputs_high = all_inputs_high && self.input_last_pulse[i];
        }

        if all_inputs_high {
            self.low_pulses_sent += self.destinations.len();
        } else {
            self.high_pulses_sent += self.destinations.len();
        }

        self.destinations
            .iter()
            .map(|dest| Pulse::new(&self.name, dest, !all_inputs_high))
            .collect::<Vec<Pulse>>()
    }

    fn get_pulses_sent(&self) -> (usize, usize) {
        (self.low_pulses_sent, self.high_pulses_sent)
    }

    fn get_state(&self) -> String {
        let input_state = self
            .inputs
            .iter()
            .zip(self.input_last_pulse.iter())
            .map(|(input, last)| input.clone() + &bool_str(*last))
            .fold(String::new(), |acc, s| acc + &s);

        self.name.clone() + &input_state
    }
}

#[derive(Debug)]
struct Button {
    name: String,
    low_pulses_sent: usize,
}

impl Button {
    fn new() -> Self {
        Self {
            name: "button".to_string(),
            low_pulses_sent: 0,
        }
    }

    fn start(&mut self) -> Pulse {
        self.low_pulses_sent += 1;

        Pulse::start()
    }

    fn get_pulses_sent(&self) -> (usize, usize) {
        (self.low_pulses_sent, 0)
    }

    fn get_state(&self) -> String {
        self.name.clone()
    }
}

fn bool_str(val: bool) -> String {
    match val {
        true => "T".to_string(),
        false => "F".to_string(),
    }
}

#[derive(Debug)]
enum Module {
    Broadcast(Broadcast),
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Button(Button),
}

type Modules = HashMap<String, Module>;

fn parse_modules(data: &str) -> Modules {
    let mut modules = HashMap::new();

    let mut conjunction_modules: HashMap<String, Vec<String>> = HashMap::new();

    for line in data.lines() {
        let pieces = line
            .split(|c: char| c.is_ascii_whitespace() || c == ',')
            .filter(|&s| !s.is_empty() && s != "->")
            .collect::<Vec<&str>>();

        let name = pieces[0];

        let (module_name, module) = match &name[..1] {
            "b" => (name, Module::Broadcast(Broadcast::new(&name, &pieces[1..]))),
            "%" => (
                &name[1..],
                Module::FlipFlop(FlipFlop::new(&name[1..], &pieces[1..])),
            ),
            "&" => {
                let module_name = &name[1..];

                conjunction_modules.insert(module_name.to_string(), vec![]);

                (
                    module_name,
                    Module::Conjunction(Conjunction::new(module_name, &pieces[1..])),
                )
            }
            _ => panic!("Illegal module name"),
        };

        modules.insert(module_name.to_string(), module);
    }

    modules.insert("button".to_string(), Module::Button(Button::new()));

    // Search for all of the modules that have a conjunction module as a destination.
    for (name, module) in &modules {
        let destinations = match module {
            Module::Broadcast(broadcast) => &broadcast.destinations,
            Module::FlipFlop(flipflop) => &flipflop.destinations,
            Module::Conjunction(conjunction) => &conjunction.destinations,
            _ => continue, // Button modules can't point to a conjunction module.
        };

        destinations.iter().for_each(|dest| {
            conjunction_modules
                .entry(dest.clone())
                .and_modify(|v| v.push(name.clone()));
        });
    }

    // Now add all of those modules to their respective conjunction module inputs.
    for (name, inputs) in conjunction_modules {
        modules.entry(name.to_string()).and_modify(|module| {
            if let Module::Conjunction(conjunction) = module {
                inputs.iter().for_each(|input| conjunction.add_input(input));
            }
        });
    }

    modules
}

fn count_pulses(modules: &mut Modules, push_button_count: usize) -> usize {
    let (_, _, initial_state) = gather_state(modules);

    let mut state_map = HashMap::new();

    state_map.insert(initial_state, (0, 0, 0));

    let mut cycle_count = None;
    let mut cycle_length = None;

    for i in 0..push_button_count {
        let push_count = i + 1;

        let (low, high, state) = push_once(modules);

        if state_map.contains_key(&state) {
            let (original_push_count, _, _) = state_map.get(&state).unwrap();

            cycle_count = Some((low, high));
            cycle_length = Some(push_count - original_push_count);

            break;
        }

        state_map.insert(state, (push_count, low, high));
    }

    // If there was a cycle, use that to extrapolate the final result.
    let (total_low, total_high) = if cycle_count.is_some() {
        let (cycle_low, cycle_high) = cycle_count.unwrap();
        let cycle_length = cycle_length.unwrap();

        let full_cycles = push_button_count / cycle_length;
        let rem = push_button_count % cycle_length;

        let mut rem_count = None;

        for (_, (push_count, low, high)) in state_map {
            if push_count == rem {
                rem_count = Some((low, high));
                break;
            }
        }

        let (rem_low, rem_high) = rem_count.unwrap();

        let total_low = cycle_low * full_cycles + rem_low;
        let total_high = cycle_high * full_cycles + rem_high;

        (total_low, total_high)
    } else {
        let mut total = None;

        for (_, (push_count, low, high)) in state_map {
            if push_count == push_button_count {
                total = Some((low, high));
                break;
            }
        }

        total.unwrap()
    };

    total_low * total_high
}

fn push_once(modules: &mut Modules) -> (usize, usize, String) {
    let mut queue: VecDeque<Pulse> = VecDeque::new();

    if let Some(Module::Button(button)) = modules.get_mut("button") {
        queue.push_back(button.start());
    }

    while let Some(pulse) = queue.pop_front() {
        // Apply the pulse to the destination module, which can generate more pulses
        if let Some(dest_module) = modules.get_mut(&pulse.end) {
            let new_pulses = match dest_module {
                Module::Broadcast(broadcast) => broadcast.apply(&pulse),
                Module::FlipFlop(flipflop) => flipflop.apply(&pulse),
                Module::Conjunction(conjunction) => conjunction.apply(&pulse),
                _ => panic!("Illegal destination for pulse!"),
            };

            // Add the new pulses to the queue.
            new_pulses
                .into_iter()
                .for_each(|pulse| queue.push_back(pulse));
        }
    }

    gather_state(modules)
}

fn gather_state(modules: &Modules) -> (usize, usize, String) {
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    let mut all_state = String::new();

    for module in modules.values() {
        let ((low, high), state) = match module {
            Module::Broadcast(broadcast) => (broadcast.get_pulses_sent(), broadcast.get_state()),
            Module::FlipFlop(flipflop) => (flipflop.get_pulses_sent(), flipflop.get_state()),
            Module::Conjunction(conjunction) => {
                (conjunction.get_pulses_sent(), conjunction.get_state())
            }
            Module::Button(button) => (button.get_pulses_sent(), button.get_state()),
        };

        low_pulse_count += low;
        high_pulse_count += high;

        all_state += &state;
    }

    (low_pulse_count, high_pulse_count, all_state)
}

// Determine how many button pushes is required to receive a low pulse at rx.
fn part_2(data: &str) -> usize {
    let mut modules = parse_modules(data);

    // Analysis of the input showed that there are 4 separate chains that come together to produce the final
    // result at rx, so find the cycle of each one.
    let stop_modules = vec!["xm", "tr", "dr", "nh"];
    let stop_modules = stop_modules.iter().fold(HashSet::new(), |mut acc, s| {
        acc.insert(s.to_string());
        return acc;
    });

    let mut found_stop_modules = HashMap::new();

    let mut push_count: usize = 1;
    while found_stop_modules.len() != stop_modules.len() {
        if let Some(stop_module) = push_once_with_stop(&mut modules, &stop_modules) {
            found_stop_modules.insert(stop_module, push_count);
        }

        push_count += 1;
    }

    let mut cycle_lcm = 1;
    for cycle_len in found_stop_modules.values() {
        cycle_lcm = lcm(cycle_lcm, *cycle_len);
    }

    cycle_lcm
}

fn push_once_with_stop(modules: &mut Modules, stop_modules: &HashSet<String>) -> Option<String> {
    let mut stop_module = None;

    let mut queue: VecDeque<Pulse> = VecDeque::new();

    if let Some(Module::Button(button)) = modules.get_mut("button") {
        queue.push_back(button.start());
    }

    while let Some(pulse) = queue.pop_front() {
        if !pulse.high_pulse && stop_modules.contains(&pulse.end) {
            stop_module = Some(pulse.end.to_string());
        }

        // Apply the pulse to the destination module, which can generate more pulses
        if let Some(dest_module) = modules.get_mut(&pulse.end) {
            let new_pulses = match dest_module {
                Module::Broadcast(broadcast) => broadcast.apply(&pulse),
                Module::FlipFlop(flipflop) => flipflop.apply(&pulse),
                Module::Conjunction(conjunction) => conjunction.apply(&pulse),
                _ => panic!("Illegal destination for pulse!"),
            };

            // Add the new pulses to the queue.
            new_pulses
                .into_iter()
                .for_each(|pulse| queue.push_back(pulse));
        }
    }

    stop_module
}

fn gcd(a: usize, b: usize) -> usize {
    let mut first = a;
    let mut second = b;

    while first != second {
        if first > second {
            first = first - second;
        } else {
            second = second - first;
        }
    }

    first
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

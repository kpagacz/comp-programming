#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    state: bool,
}

impl FlipFlop {
    fn new(name: String) -> Self {
        Self { name, state: false }
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
}

impl Broadcaster {
    fn new(name: String) -> Self {
        Self { name }
    }
}

use std::collections::BTreeMap;
#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    sources: BTreeMap<String, bool>,
}

impl Conjunction {
    fn new(name: String, sources: Vec<String>) -> Self {
        Self {
            name,
            sources: sources.into_iter().fold(BTreeMap::new(), |mut map, name| {
                map.insert(name, false);
                map
            }),
        }
    }
}

#[derive(Debug, Clone)]
struct Signal {
    source: String,
    high: bool,
}

impl Signal {
    fn new(source: String, high: bool) -> Self {
        Self { source, high }
    }
}

trait Module: std::fmt::Debug {
    fn receive(&mut self, signal: &Signal) -> Option<Signal>;
}

impl Module for Broadcaster {
    fn receive(&mut self, signal: &Signal) -> Option<Signal> {
        Some(Signal::new(self.name.clone(), signal.high))
    }
}

impl Module for FlipFlop {
    fn receive(&mut self, signal: &Signal) -> Option<Signal> {
        if signal.high {
            None
        } else {
            self.state = !self.state;
            Some(Signal::new(self.name.clone(), self.state))
        }
    }
}

impl Module for Conjunction {
    fn receive(&mut self, signal: &Signal) -> Option<Signal> {
        self.sources.insert(signal.source.to_owned(), signal.high);
        if self.sources.values().all(|signal| *signal) {
            Some(Signal::new(self.name.clone(), false))
        } else {
            Some(Signal::new(self.name.clone(), true))
        }
    }
}

fn parse_input(
    input: &str,
) -> (
    BTreeMap<String, Box<dyn Module>>,
    BTreeMap<String, Vec<String>>,
) {
    let (mut modules, mut mapping) = (BTreeMap::new(), BTreeMap::new());

    let mut publishers = BTreeMap::new();
    input.lines().for_each(|line| {
        let (module_name, targets) = line.split_once(" -> ").unwrap();
        let name = &module_name[1..];
        targets.split(", ").map(str::to_string).for_each(|target| {
            publishers
                .entry(target)
                .or_insert(vec![])
                .push(name.to_string())
        });
    });
    // for (key, value) in &publishers {
    //     println!("{key} receives from: {value:?}");
    // }

    input.lines().for_each(|line| {
        let (module_name, targets) = line.split_once(" -> ").unwrap();
        let module_type = module_name.as_bytes()[0];

        let mut name = module_name[1..].to_string();
        if module_type == b'b' {
            name = format!("b{name}")
        }
        let targets = targets.split(", ").map(str::to_string).collect::<Vec<_>>();
        mapping.insert(name.to_string(), targets);

        let module: Box<dyn Module> = match module_type {
            b'b' => Box::new(Broadcaster::new(name.clone())),
            b'%' => Box::new(FlipFlop::new(name.clone())),
            b'&' => Box::new(Conjunction::new(
                name.clone(),
                publishers.get(&name).unwrap().clone(),
            )),
            _ => unreachable!(),
        };
        modules.insert(name, module);
    });

    // println!("Modules:");
    // for (key, module) in &modules {
    //     println!("{key}: {module:?}");
    // }
    (modules, mapping)
}

use std::collections::VecDeque;
fn part1(input: &str) -> usize {
    let (mut modules, mut mapping) = parse_input(input);
    mapping.insert("button".to_string(), vec!["broadcaster".to_string()]);

    let mut high_count = 0;
    let mut low_count = 0;
    for _ in 0..1000 {
        let mut signals = VecDeque::new();
        signals.push_back(Signal::new("button".to_string(), false));
        while let Some(signal) = signals.pop_front() {
            let recipients = mapping.get(&signal.source).unwrap();
            if signal.high {
                high_count += recipients.len();
            } else {
                low_count += recipients.len();
            }

            for recipient in recipients {
                if let Some(module) = modules.get_mut(recipient) {
                    if let Some(response) = module.receive(&signal) {
                        signals.push_back(response)
                    }
                } else {
                    // println!("Did not find a module for the recipient: {recipient:?}");
                }
            }
        }
    }

    high_count * low_count
}

fn part2(input: &str) -> usize {
    let (mut modules, mut mapping) = parse_input(input);
    mapping.insert("button".to_string(), vec!["broadcaster".to_string()]);

    let mut button_presses = 0;
    loop {
        button_presses += 1;
        if button_presses % 1048576 == 0 {
            println!("Pressed button {button_presses} times");
        }
        let mut signals = VecDeque::new();
        signals.push_back(Signal::new("button".to_string(), false));
        while let Some(signal) = signals.pop_front() {
            let recipients = mapping.get(&signal.source).unwrap();

            for conj in ["kk", "sk", "vt", "xc"] {
                if signal.source == conj && signal.high {
                    println!("Got a high signal from {conj} at {button_presses}");
                }
            }

            for recipient in recipients {
                if let Some(module) = modules.get_mut(recipient) {
                    if let Some(response) = module.receive(&signal) {
                        signals.push_back(response)
                    }
                } else {
                    // println!("Did not find a module for the recipient: {recipient:?}");
                    if recipient == "rx" && !signal.high {
                        return button_presses;
                    }
                }
            }
        }
    }
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));

    println!("TEST2");
    let test = include_str!("../test2");
    println!("Part 1: {}", part1(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

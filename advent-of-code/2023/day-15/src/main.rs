fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .replace('\n', "")
        .split(',')
        .map(|step| step.as_bytes().to_vec())
        .collect()
}

fn hash(instruction: &[u8]) -> usize {
    instruction.iter().fold(0, |mut acc, c| {
        acc += *c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part1(input: &str) -> usize {
    let instructions = parse_input(input);
    instructions
        .into_iter()
        .map(|instruction| hash(&instruction))
        .sum::<usize>()
}

#[derive(Debug, Clone)]
enum Instruction {
    Remove(Vec<u8>),        // lens label
    Insert(Vec<u8>, usize), // lens label, focal length
}

fn parse_instruction(instruction: Vec<u8>) -> Instruction {
    let split: Vec<Vec<u8>> = instruction
        .split(|&c| c == b'-' || c == b'=')
        .filter_map(|slice| {
            if !slice.is_empty() {
                Some(slice.to_vec())
            } else {
                None
            }
        })
        .collect();
    match split.len() {
        2 => {
            let focal_length = String::from_utf8(split[1].clone())
                .unwrap()
                .parse::<usize>()
                .unwrap();
            Instruction::Insert(split[0].clone(), focal_length)
        }
        1 => Instruction::Remove(split[0].clone()),
        _ => unreachable!(),
    }
}

use std::collections::BTreeMap;
#[derive(Debug, Clone)]
struct Box {
    lenses: BTreeMap<String, usize>,
    insertion_order: BTreeMap<String, usize>,
    ops: usize,
}

impl Box {
    fn new() -> Self {
        Self {
            lenses: BTreeMap::new(),
            insertion_order: BTreeMap::new(),
            ops: 0,
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) {
        self.ops += 1;
        match instruction {
            Instruction::Remove(lens) => {
                let label = Self::get_label(lens);
                self.insertion_order.remove(&label);
                self.lenses.remove(&label);
            }
            Instruction::Insert(lens, focal_length) => {
                let label = Self::get_label(lens);
                self.lenses.insert(label.clone(), *focal_length);
                if !self.insertion_order.contains_key(&label) {
                    self.insertion_order.insert(label.clone(), self.ops);
                }
            }
        }
    }

    fn focus_power(&self, box_number: usize) -> usize {
        let mut lenses: Vec<_> = self.insertion_order.keys().collect();
        lenses.sort_by_key(|&k| self.insertion_order.get(k).unwrap());
        lenses
            .into_iter()
            .enumerate()
            .map(|(pos, lens)| (box_number + 1) * (pos + 1) * (self.lenses.get(lens).unwrap()))
            .sum()
    }

    fn get_label(label: &[u8]) -> String {
        String::from_utf8(label.to_vec()).unwrap()
    }
}

fn part2(input: &str) -> usize {
    let instructions = parse_input(input);
    let instructions: Vec<_> = instructions.into_iter().map(parse_instruction).collect();

    let mut boxes = vec![Box::new(); 256];
    instructions.into_iter().for_each(|instruction| {
        let mut destination_box = 0;
        match &instruction {
            Instruction::Remove(lens) => {
                destination_box = hash(lens);
            }
            Instruction::Insert(lens, focal_length) => {
                destination_box = hash(lens);
            }
        }
        boxes[destination_box].execute_instruction(&instruction);
    });
    boxes
        .into_iter()
        .enumerate()
        .map(|(pos, single_box)| single_box.focus_power(pos))
        .sum()
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

use std::collections::VecDeque;

fn make_move(cups: &mut VecDeque<u8>, current_cup: u8) {
    let mut destination_cup = current_cup - 1;
    let current_cup = cups.iter().position(|&el| el == current_cup).unwrap();
    let mut moved_cups: VecDeque<u8> = VecDeque::new();
    let mut removed_elements = [
        (current_cup + 1) % cups.len(),
        (current_cup + 2) % cups.len(),
        (current_cup + 3) % cups.len(),
    ];
    removed_elements
        .iter()
        .for_each(|&removed_cup| moved_cups.push_back(*cups.get(removed_cup).unwrap()));
    removed_elements.sort();
    removed_elements.iter().rev().for_each(|&removed_cup| {
        cups.remove(removed_cup);
    });

    while cups.iter().find(|&&el| el == destination_cup) == None {
        if destination_cup < 1 {
            destination_cup = cups.iter().max().unwrap().clone();
        } else {
            destination_cup = destination_cup - 1;
        }
    }
    let destination = cups.iter().position(|&el| el == destination_cup).unwrap() + 1;
    let second_part = cups.split_off(destination);
    cups.extend(moved_cups.iter());
    cups.extend(second_part.iter());
}

fn make_move_two(cups: &mut VecDeque<u64>, current_cup: u64) {
    let mut destination_cup = current_cup - 1;
    let current_cup = cups.iter().position(|&el| el == current_cup).unwrap();
    let mut moved_cups: VecDeque<u64> = VecDeque::new();
    let mut removed_elements = [
        (current_cup + 1) % cups.len(),
        (current_cup + 2) % cups.len(),
        (current_cup + 3) % cups.len(),
    ];
    removed_elements
        .iter()
        .for_each(|&removed_cup| moved_cups.push_back(*cups.get(removed_cup).unwrap()));
    removed_elements.sort();
    removed_elements.iter().rev().for_each(|&removed_cup| {
        cups.remove(removed_cup);
    });

    while cups.iter().find(|&&el| el == destination_cup) == None {
        if destination_cup < 1 {
            destination_cup = cups.iter().max().unwrap().clone();
        } else {
            destination_cup = destination_cup - 1;
        }
    }
    let destination = cups.iter().position(|&el| el == destination_cup).unwrap() + 1;
    let second_part = cups.split_off(destination);
    cups.extend(moved_cups.iter());
    cups.extend(second_part.iter());
}

fn parse_input(path: &str) -> VecDeque<u8> {
    let input = std::fs::read_to_string(path).unwrap();
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}
fn part1(path: &str) -> VecDeque<u8> {
    let mut cups = parse_input(path);

    let mut current_cup = cups[0];
    for _ in 0..100 {
        make_move(&mut cups, current_cup);
        let current_cup_position = cups.iter().position(|&el| el == current_cup).unwrap();
        let next_cup_position = (current_cup_position + 1) % cups.len();
        current_cup = cups[next_cup_position];
    }

    cups
}

fn part2(path: &str) -> u64 {
    let mut cups = parse_input(path);
    let mut cups: VecDeque<u64> = cups.iter().map(|&el| el as u64).collect();
    cups.reserve(1000000);

    let max = cups.iter().max().unwrap();
    for cup in (max + 1)..=1e6 as u64 {
        cups.push_back(cup);
    }

    let mut current_cup = cups[0];
    for _ in 0..100 {
        make_move_two(&mut cups, current_cup);
        let current_cup_position = cups.iter().position(|&el| el == current_cup).unwrap();
        let next_cup_position = (current_cup_position + 1) % cups.len();
        current_cup = cups[next_cup_position];
    }

    let one_position = 0;
    0
}

fn main() {
    let path = "test";
    println!("Part 1: {:?}", part1(path));
    println!("Part 2: {:?}", part2(path));
}

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

// Part 1
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

// Part 2
#[derive(Debug)]
struct Node {
    value: u64,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: u64) -> Self {
        Node {
            value,
            next: Option::None,
        }
    }
}

type Link = Rc<RefCell<Node>>;

fn make_move_using_lists(cups: &Vec<Link>, current_cup: usize) {
    let chosen_cup = cups[current_cup].clone();
    let moved_head = chosen_cup.as_ref().borrow().next.clone().unwrap().clone();
    let moved_tail = moved_head
        .as_ref()
        .borrow()
        .next
        .clone()
        .unwrap()
        .clone()
        .as_ref()
        .borrow()
        .next
        .clone()
        .unwrap()
        .clone();
    let after_moved = moved_tail.as_ref().borrow().next.clone().unwrap();
    chosen_cup.as_ref().borrow_mut().next = Some(after_moved);
}

fn find_cup_destination(cups: &Vec<Link>, current_cup: usize) -> Link {

}

fn part2(path: &str, size: u64) -> u64 {
    let cups = parse_input(path);
    let mut cups: VecDeque<u64> = cups.iter().map(|&el| el as u64).collect();
    cups.reserve(size as usize);

    let max = cups.iter().max().unwrap();
    for cup in (max + 1)..=size {
        cups.push_back(cup);
    }

    let mut cups: Vec<_> = cups
        .iter()
        .map(|&el| Rc::new(RefCell::new(Node::new(el))))
        .collect();
    println!("Cups size: {}", cups.len());

    cups.windows(2).for_each(|slice| {
        let (first_node, second_node) = (slice[0].clone(), slice[1].clone());
        first_node.borrow_mut().next = Some(second_node);
    });
    cups.last().unwrap().borrow_mut().next = Some(cups.first().unwrap().clone());

    let mut current_cup = cups[0].as_ref().borrow().value;
    cups.sort_by(|a, b| a.as_ref().borrow().value.cmp(&b.as_ref().borrow().value));

    0
}

fn main() {
    let path = "test";
    println!("Part 1: {:?}", part1(path));
    println!("Part 2: {:?}", part2(path, 10));
}

use std::{
    cell::RefCell, collections::VecDeque, fmt::Display, iter::once, rc::Rc, thread::current,
};

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
#[derive(Debug, PartialEq, Clone)]
struct Node {
    value: usize,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: usize) -> Self {
        Node {
            value,
            next: Option::None,
        }
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = Some(Rc::new(RefCell::new(self.clone())));
        while let Some(node) = it {
            f.write_fmt(format_args!("{} -> ", node.borrow().value))?;
            it = node.borrow().next.clone();
        }
        Ok(())
    }
}

type Link = Rc<RefCell<Node>>;

fn make_move_using_lists(arena_cups: &Vec<Link>, head: &mut Link, current_cup: &mut Link) {
    let mut removed = vec![];
    let mut count = 3;
    while count > 0 {
        let next_cup = current_cup.borrow_mut().next.take();
        if let Some(next_cup) = next_cup {
            current_cup.borrow_mut().next = next_cup.borrow_mut().next.take();
            removed.push(next_cup);
            count -= 1;
        } else {
            break;
        }
    }
    while count > 0 {
        removed.push(head.clone());
        let new_head = head.borrow_mut().next.take().unwrap();
        *head = new_head;
        count -= 1;
    }
    // println!("removed cups: {removed:?}");
    // println!("Head after removal: {}", head.borrow());

    let destination_cup =
        find_destination_cup(&removed, current_cup.borrow().value, arena_cups.len());
    let destination_cup = &arena_cups[destination_cup];
    // println!("destination cup: {}", destination_cup.borrow());
    let after_destination = destination_cup.borrow_mut().next.take();
    removed[2].borrow_mut().next = after_destination;

    removed[1].borrow_mut().next = Some(removed.pop().unwrap());
    removed[0].borrow_mut().next = Some(removed.pop().unwrap());
    destination_cup.borrow_mut().next = Some(removed.pop().unwrap());
    // println!("after move {}", head.borrow());

    if current_cup.borrow().next.is_some() {
        let next = current_cup.borrow().next.clone();
        *current_cup = next.unwrap();
    } else {
        *current_cup = head.clone();
    }
    // println!("current cup after move: {}", current_cup.borrow());
}

fn find_destination_cup(removed: &Vec<Link>, current_cup: usize, arena_size: usize) -> usize {
    let mut next_current = current_cup - 1;
    let removed_values: Vec<usize> = removed.iter().map(|link| link.borrow().value).collect();
    while removed_values.contains(&next_current) {
        next_current -= 1;
    }
    if next_current == 0 {
        next_current = arena_size - 1;
        while removed_values.contains(&next_current) {
            next_current -= 1;
        }
        next_current
    } else {
        next_current
    }
}

fn part2(path: &str) -> usize {
    let cups: Vec<u8> = parse_input(path).into();
    let mut cups = once(0_usize)
        .chain(cups.into_iter().map(u8::into))
        .chain(10..=1000000)
        .collect::<Vec<usize>>();
    cups.sort();

    let arena_cups: Vec<Link> = cups
        .into_iter()
        .map(|val| Rc::new(RefCell::new(Node::new(val as usize))))
        .collect();
    // println!("cups: {arena_cups:?}");
    arena_cups.windows(2).for_each(|window| {
        let (first, second) = (&window[0], &window[1]);
        first.borrow_mut().next = Some(second.clone());
    });
    // test: (3) 8  9  1  2  5  4  6  7
    // arena_cups[3].borrow_mut().next = Some(arena_cups[8].clone());
    // arena_cups[8].borrow_mut().next = Some(arena_cups[9].clone());
    // arena_cups[9].borrow_mut().next = Some(arena_cups[1].clone());
    // arena_cups[1].borrow_mut().next = Some(arena_cups[2].clone());
    // arena_cups[2].borrow_mut().next = Some(arena_cups[5].clone());
    // arena_cups[5].borrow_mut().next = Some(arena_cups[4].clone());
    // arena_cups[4].borrow_mut().next = Some(arena_cups[6].clone());
    // arena_cups[6].borrow_mut().next = Some(arena_cups[7].clone());
    // arena_cups[7].borrow_mut().next = Some(arena_cups[10].clone());

    // real: 3 1 5 6 7 9 8 2 4
    arena_cups[3].borrow_mut().next = Some(arena_cups[1].clone());
    arena_cups[1].borrow_mut().next = Some(arena_cups[5].clone());
    arena_cups[5].borrow_mut().next = Some(arena_cups[6].clone());
    arena_cups[6].borrow_mut().next = Some(arena_cups[7].clone());
    arena_cups[7].borrow_mut().next = Some(arena_cups[9].clone());
    arena_cups[9].borrow_mut().next = Some(arena_cups[8].clone());
    arena_cups[8].borrow_mut().next = Some(arena_cups[2].clone());
    arena_cups[2].borrow_mut().next = Some(arena_cups[4].clone());
    arena_cups[4].borrow_mut().next = Some(arena_cups[10].clone());

    let mut current_cup = arena_cups[3].clone();
    let mut head = arena_cups[3].clone();
    // println!("Arena cups initially:");
    // println!("{}", head.borrow());

    let moves = 10_000_000;
    for i in 0..moves {
        if i % (moves / 10) == 0 {
            println!("{i} moves done");
        }
        make_move_using_lists(&arena_cups, &mut head, &mut current_cup);
    }

    fn find_one(head: &Link) -> Link {
        let mut it = Some(head.clone());
        while let Some(node) = it {
            if node.borrow().value == 1 {
                return node.clone();
            } else {
                it = node.borrow().next.clone();
            }
        }
        unreachable!()
    }

    let one = find_one(&head);
    let a_one = one.borrow().next.clone().unwrap();
    let aa_one = a_one.borrow().clone().next.unwrap().clone();
    let (first, second) = (a_one.borrow().value, aa_one.borrow().value);
    println!("Part 2: {}", first * second);
    0
}

fn main() {
    let path = "test";
    println!("Part 1: {:?}", part1(path));
    part2(path);
}

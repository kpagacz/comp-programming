use std::collections::{HashSet, VecDeque};

fn part1(input: &str) -> u32 {
    let (mut deck_one, mut deck_two) = parse_input(input);

    while deck_one.is_empty() == false && deck_two.is_empty() == false {
        play_round(&mut deck_one, &mut deck_two);
    }
    let non_empty_deck: &VecDeque<u32>;
    if deck_one.is_empty() {
        non_empty_deck = &deck_two;
    } else {
        non_empty_deck = &deck_one;
    }

    non_empty_deck
        .iter()
        .rev()
        .zip(1..=non_empty_deck.len() as u32)
        .map(|(card, multiplier)| card * multiplier)
        .sum()
}

fn play_round(deck_one: &mut VecDeque<u32>, deck_two: &mut VecDeque<u32>) {
    if deck_one.front() > deck_two.front() {
        deck_one.rotate_left(1);
        deck_one.push_back(deck_two.pop_front().unwrap());
    } else {
        deck_two.rotate_left(1);
        deck_two.push_back(deck_one.pop_front().unwrap());
    }
}

fn part2(input: &str) -> u32 {
    let (deck_one, deck_two) = parse_input(input);

    let (_, winner_deck) = play_recursive_combat(deck_one, deck_two);
    winner_deck
        .iter()
        .rev()
        .zip(1..=winner_deck.len() as u32)
        .map(|(card, multiplier)| card * multiplier)
        .sum()
}

enum Winner {
    One,
    Two,
}
fn play_recursive_combat(
    mut deck_one: VecDeque<u32>,
    mut deck_two: VecDeque<u32>,
) -> (Winner, VecDeque<u32>) {
    let mut mem = HashSet::<(VecDeque<u32>, VecDeque<u32>)>::new();
    mem.insert((
        deck_one.iter().copied().collect(),
        deck_two.iter().copied().collect(),
    ));
    while deck_one.is_empty() == false && deck_two.is_empty() == false {
        // println!("{:?}\n{:?}\n\n", deck_one, deck_two);
        play_recursive_round(&mut deck_one, &mut deck_two);
        if mem.contains(&(
            deck_one.iter().copied().collect(),
            deck_two.iter().copied().collect(),
        )) {
            return (Winner::One, deck_one);
        }
    }

    if deck_one.is_empty() {
        (Winner::Two, deck_two)
    } else {
        (Winner::One, deck_one)
    }
}

fn play_recursive_round(deck_one: &mut VecDeque<u32>, deck_two: &mut VecDeque<u32>) {
    let (top_one, top_two) = (deck_one.pop_front().unwrap(), deck_two.pop_front().unwrap());
    if top_one <= deck_one.len() as u32 && top_two <= deck_two.len() as u32 {
        // println!("Recursive at {} {}", top_one, top_two);
        let (winner, _) = play_recursive_combat(
            deck_one.iter().take(top_one as usize).copied().collect(),
            deck_two.iter().take(top_two as usize).copied().collect(),
        );
        match winner {
            Winner::One => {
                deck_one.push_back(top_one);
                deck_one.push_back(top_two);
            }
            Winner::Two => {
                deck_two.push_back(top_two);
                deck_two.push_back(top_one);
            }
        }
    } else if top_one > top_two {
        deck_one.push_back(top_one);
        deck_one.push_back(top_two);
    } else {
        deck_two.push_back(top_two);
        deck_two.push_back(top_one);
    }
}

fn parse_input(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let players: Vec<&str> = input.split("\n\n").collect();
    let (player_one, player_two) = (players[0], players[1]);

    let deck_one: VecDeque<_> = player_one
        .lines()
        .skip(1)
        .map(str::parse::<u32>)
        .map(|i| i.unwrap())
        .collect();
    let deck_two: VecDeque<_> = player_two
        .lines()
        .skip(1)
        .map(str::parse::<u32>)
        .map(|i| i.unwrap())
        .collect();

    (deck_one, deck_two)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

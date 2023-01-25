use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet, VecDeque},
    hash::{Hash, Hasher},
};

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

    let mut cache = HashMap::<u64, (Winner, VecDeque<u32>)>::new();
    let (_, winner_deck) = play_recursive_combat(deck_one, deck_two, &mut cache);
    winner_deck
        .iter()
        .rev()
        .zip(1..=winner_deck.len() as u32)
        .map(|(card, multiplier)| card * multiplier)
        .sum()
}

#[derive(Copy, Clone)]
enum Winner {
    One,
    Two,
}
fn play_recursive_combat(
    mut deck_one: VecDeque<u32>,
    mut deck_two: VecDeque<u32>,
    cache: &mut HashMap<u64, (Winner, VecDeque<u32>)>,
) -> (Winner, VecDeque<u32>) {
    // println!("{:?} {:?}", deck_one, deck_two);
    let mut hasher = DefaultHasher::new();
    deck_one.hash(&mut hasher);
    deck_two.hash(&mut hasher);
    let hash = hasher.finish();
    if cache.contains_key(&hash) {
        let (winner, winning_deck) = cache.get(&hash).unwrap();
        return (winner.clone(), winning_deck.clone());
    }

    let mut mem: HashSet<u64> = HashSet::new();
    mem.insert(hash.clone());
    while deck_one.is_empty() == false && deck_two.is_empty() == false {
        // println!("{:?}\n{:?}\n\n", deck_one, deck_two);
        play_recursive_round(&mut deck_one, &mut deck_two, cache);
        let mut hasher = DefaultHasher::new();
        deck_one.hash(&mut hasher);
        deck_two.hash(&mut hasher);
        let new_hash = hasher.finish();
        if mem.contains(&new_hash) {
            cache.insert(hash, (Winner::One, deck_one.clone()));
            return (Winner::One, deck_one);
        } else {
            mem.insert(new_hash);
        }
    }

    if deck_one.is_empty() {
        cache.insert(hash, (Winner::Two, deck_two.clone()));
        (Winner::Two, deck_two)
    } else {
        cache.insert(hash, (Winner::One, deck_one.clone()));
        (Winner::One, deck_one)
    }
}

fn play_recursive_round(
    deck_one: &mut VecDeque<u32>,
    deck_two: &mut VecDeque<u32>,
    cache: &mut HashMap<u64, (Winner, VecDeque<u32>)>,
) {
    // println!("Another round: {:?} {:?}", deck_one, deck_two);
    let (top_one, top_two) = (deck_one.pop_front().unwrap(), deck_two.pop_front().unwrap());
    if top_one <= deck_one.len() as u32 && top_two <= deck_two.len() as u32 {
        let (winner, _) = play_recursive_combat(
            deck_one.iter().take(top_one as usize).copied().collect(),
            deck_two.iter().take(top_two as usize).copied().collect(),
            cache,
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

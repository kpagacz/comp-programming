use std::collections::BTreeSet;

fn parse_scratchcard(input: &str) -> (BTreeSet<usize>, Vec<usize>) {
    let s: Vec<_> = input.split(": ").collect();
    let numbers: Vec<_> = s[1].split(" | ").collect();
    let winning: BTreeSet<_> = numbers[0]
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("Always a number"))
        .collect();
    let gotten: Vec<_> = numbers[1]
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("Always a number"))
        .collect();

    (winning, gotten)
}

fn score_scratchcard(card: &(BTreeSet<usize>, Vec<usize>)) -> usize {
    let (winning, gotten) = (&card.0, &card.1);
    gotten.iter().fold(0, |acc, gotten_num| {
        if winning.contains(gotten_num) {
            if acc == 0 {
                1
            } else {
                2 * acc
            }
        } else {
            acc
        }
    })
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_scratchcard)
        .map(|card| score_scratchcard(&card))
        .sum()
}

fn part2(input: &str) -> usize {
    let scratchcards: Vec<_> = input.lines().map(parse_scratchcard).collect();
    let mut copies = vec![1usize; scratchcards.len()];
    scratchcards
        .into_iter()
        .enumerate()
        .for_each(|(pos, (winning, gotten))| {
            let matching_count =
                gotten.into_iter().fold(
                    0usize,
                    |acc, n| if winning.contains(&n) { acc + 1 } else { acc },
                );
            let current_copies = copies[pos];
            copies[pos + 1..]
                .iter_mut()
                .take(matching_count)
                .for_each(|copies_count| *copies_count += current_copies);
        });

    copies.into_iter().sum()
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

fn parse_input(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

use std::collections::HashMap;
fn split_number(num: i64, mem: &mut HashMap<i64, (i64, i64)>) -> (i64, i64) {
    if let Some(&pair) = mem.get(&(num)) {
        pair
    } else {
        let s = num.to_string();

        let (part1, part2) = (&s[..s.len() / 2], &s[s.len() / 2..]);
        let res = (part1.parse().unwrap(), part2.parse().unwrap());
        mem.insert(num, res);
        res
    }
}

fn part1(input: &str) -> usize {
    let nums = parse_input(input);
    let mut stones: HashMap<i64, i32> =
        nums.into_iter().fold(HashMap::default(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });
    let mut mem = HashMap::default();

    for _ in 0..25 {
        let mut new_stones = HashMap::with_capacity(stones.len());
        stones.into_iter().for_each(|(key, value)| match key {
            0 => {
                *new_stones.entry(1).or_default() += value;
            }
            1 => {
                *new_stones.entry(2024).or_default() += value;
            }
            key if key.ilog10() % 2 == 1 => {
                let (parta, partb) = split_number(key, &mut mem);
                *new_stones.entry(parta).or_insert(0) += value;
                *new_stones.entry(partb).or_insert(0) += value;
            }
            key => {
                *new_stones.entry(key * 2024).or_insert(0) += value;
            }
        });
        stones = new_stones;
    }

    stones.values().map(|v| *v as usize).sum::<usize>()
}

fn part2(input: &str) -> usize {
    let nums = parse_input(input);
    let mut stones: HashMap<i64, usize> =
        nums.into_iter().fold(HashMap::default(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });
    let mut mem = HashMap::default();

    for _ in 0..75 {
        let mut new_stones = HashMap::with_capacity(stones.len());
        stones.into_iter().for_each(|(key, value)| match key {
            0 => {
                *new_stones.entry(1).or_default() += value;
            }
            1 => {
                *new_stones.entry(2024).or_default() += value;
            }
            key if key.ilog10() % 2 == 1 => {
                let (parta, partb) = split_number(key, &mut mem);
                *new_stones.entry(parta).or_insert(0) += value;
                *new_stones.entry(partb).or_insert(0) += value;
            }
            key => {
                *new_stones.entry(key * 2024).or_insert(0) += value;
            }
        });
        stones = new_stones;
    }

    stones.values().sum::<usize>()
}

fn main() {
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    let input = include_str!("../input");
    println!("Part 1 (test): {}", part1(test));
    println!("Part 1 (test): {}", part1(test2));
    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

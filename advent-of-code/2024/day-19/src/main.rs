use std::collections::HashSet;
fn parse_input(input: &str) -> (HashSet<String>, Vec<String>) {
    let (available, patterns) = input.split_once("\n\n").unwrap();

    (
        available.split(", ").map(|s| s.to_owned()).collect(),
        patterns.lines().map(|l| l.to_owned()).collect(),
    )
}

use std::collections::HashMap;
fn rec(pattern: &str, available: &HashSet<String>, mem: &mut HashMap<String, bool>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if let Some(cached) = mem.get(pattern) {
        return *cached;
    } else {
        mem.insert(pattern.to_string(), false);
    }

    for i in 1..=pattern.len() {
        if available.contains(&pattern[..i]) && rec(&pattern[i..], available, mem) {
            let entry = mem.entry(pattern.to_string()).or_insert(false);
            *entry = true;
            return true;
        }
    }

    false
}

fn part1(input: &str) -> usize {
    let (available, patterns) = parse_input(input);

    let mut mem = HashMap::new();
    patterns
        .into_iter()
        .filter(|pattern| rec(pattern, &available, &mut mem))
        .count()
}

fn rec_count(
    pattern: &str,
    available: &HashSet<String>,
    mem: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(cached) = mem.get(pattern) {
        return *cached;
    } else {
        mem.insert(pattern.to_string(), 0);
    }

    for i in 1..=pattern.len() {
        if available.contains(&pattern[..i]) {
            let possible_ways = rec_count(&pattern[i..], available, mem);
            let entry = mem.entry(pattern.to_string()).or_insert(0);
            *entry += possible_ways;
        }
    }

    *mem.get(pattern).unwrap()
}

fn part2(input: &str) -> usize {
    let (available, patterns) = parse_input(input);

    let mut mem = HashMap::new();
    patterns
        .into_iter()
        .map(|pattern| rec_count(&pattern, &available, &mut mem))
        .sum::<usize>()
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");

    println!("Part 1 (test): {}, expected 6", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}, expected 16", part2(test));
    println!("Part 2: {}", part2(input));
}

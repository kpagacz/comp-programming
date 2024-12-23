fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect()
}

const PRUNE: usize = 0b111111111111111111111111;
fn next_secret(mut secret: usize, times: usize) -> usize {
    for _ in 0..times {
        let first = secret << 6;
        secret ^= first;
        secret &= PRUNE;

        let second = secret >> 5;
        secret ^= second;
        secret &= PRUNE;

        let third = secret << 11;
        secret ^= third;
        secret &= PRUNE;
    }

    secret
}

fn all_secrets(mut secret: usize, times: usize) -> Vec<usize> {
    let mut answer = vec![secret];

    for _ in 0..times {
        secret = next_secret(secret, 1);
        answer.push(secret);
    }

    answer
}

fn differences(secrets: &[usize]) -> Vec<(i32, usize)> {
    secrets
        .windows(2)
        .map(|window| {
            (
                window[1] as i32 % 10 - window[0] as i32 % 10,
                window[1] % 10,
            )
        })
        .collect()
}

use std::collections::HashMap;
use std::collections::HashSet;
fn differences_to_bananas(differences: &[(i32, usize)], bananas: &mut HashMap<Vec<i32>, usize>) {
    assert!(differences.len() == 2000);
    let mut seen = HashSet::new();
    differences.windows(4).for_each(|window| {
        let haul = window[3].1;
        let pattern: Vec<_> = window[..4].iter().map(|(diff, _)| *diff).collect();
        if !seen.contains(&pattern) {
            seen.insert(pattern.clone());
            *bananas.entry(pattern).or_insert(0) += haul;
        }
    })
}

fn part1(input: &str) -> usize {
    let secrets = parse_input(input);

    secrets
        .into_iter()
        .map(|secret| next_secret(secret, 2000))
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let secrets = parse_input(input);

    let mut bananas = HashMap::new();
    for secret in secrets {
        let all_secrets = all_secrets(secret, 2000);
        let diffs = differences(&all_secrets);
        differences_to_bananas(&diffs, &mut bananas);
    }

    *bananas.values().max().unwrap()
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {}, expected 37327623", part1(test));
    println!("Part 1: {}", part1(input));

    let test2 = include_str!("../test2");
    println!("Part 2 (test): {}, expected 23", part2(test2));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        let secret = 123;
        for time in 1..10 {
            println!("{}", next_secret(secret, time));
        }
        panic!()
    }

    #[test]
    fn test_next_secret_example() {
        let secret = 1;
        println!("{}", next_secret(secret, 2000));
        panic!()
    }
}

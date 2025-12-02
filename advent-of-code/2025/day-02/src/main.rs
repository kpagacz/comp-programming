// https://adventofcode.com/2025/day/2
use std::collections::HashSet;
fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input
        .trim_end()
        .split(",")
        .map(|range| range.split_once("-").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<i64>().unwrap(),
                second.parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn find_invalid_ids(
    lower: i64,
    upper: i64,
    max_repeats: usize,
    min_override: Option<i64>,
) -> HashSet<i64> {
    let lower_digits = (lower as f64).log10().floor() as u32 + 1;
    let upper_digits = (upper as f64).log10().floor() as u32 + 1;

    let min = min_override.unwrap_or(10i64.pow((lower_digits / 2).saturating_sub(1)));
    let max = 10i64.pow(upper_digits / 2);

    let mut answer = HashSet::new();
    for num in min..max {
        for repeats in 2..=10.min(max_repeats) {
            let repeated = format!("{}", num).repeat(repeats);
            if repeated.len() > 12 {
                continue;
            }
            let repeated_num = repeated.parse::<i64>().unwrap();
            if repeated_num >= lower && repeated_num <= upper {
                answer.insert(repeated_num);
            }
        }
    }
    answer
}

fn part1(input: &str) -> i64 {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .flat_map(|range| find_invalid_ids(range.0, range.1, 2, None))
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let ranges = parse_input(input);
    ranges
        .into_iter()
        .flat_map(|range| find_invalid_ids(range.0, range.1, 10, Some(1)))
        .sum::<i64>()
}
fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}

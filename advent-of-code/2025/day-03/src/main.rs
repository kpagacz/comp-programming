// https://adventofcode.com/2025/day/3
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn max_joltage(battery: &[char], digit_limit: usize) -> usize {
    let mut stack = vec![];
    for (pos, c) in battery.iter().enumerate() {
        let left = battery.len() - pos;
        while !stack.is_empty() && stack.len() + left > digit_limit && stack.last().unwrap() < c {
            stack.pop();
        }
        if stack.len() < digit_limit {
            stack.push(*c);
        }
    }
    stack.iter().collect::<String>().parse::<usize>().unwrap()
}

fn part1(input: &str, limit: usize) -> usize {
    let batteries = parse_input(input);
    batteries
        .into_iter()
        .map(|battery| max_joltage(&battery, limit))
        .sum()
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {}", part1(test, 2));
    println!("Part 1: {}", part1(input, 2));
    println!("Part 2 (test): {}", part1(test, 12));
    println!("Part 2: {}", part1(input, 12));
}

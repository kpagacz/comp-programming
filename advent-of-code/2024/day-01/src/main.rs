fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = vec![];
    let mut second = vec![];
    input.lines().for_each(|line| {
        let mut it = line.split_whitespace();
        first.push(it.next().unwrap().parse::<i32>().unwrap());
        second.push(it.next().unwrap().parse::<i32>().unwrap());
    });
    (first, second)
}

fn part1(input: &str) -> i32 {
    let (mut first, mut second) = parse_input(input);
    first.sort_unstable();
    second.sort_unstable();

    let mut answer = 0;
    for i in 0..first.len() {
        answer += first[i].abs_diff(second[i]) as i32;
    }
    answer
}

fn part2(input: &str) -> i32 {
    let (first, second) = parse_input(input);
    use std::collections::HashMap;

    let freq = second.iter().fold(HashMap::new(), |mut map, num| {
        *map.entry(num).or_insert(0) += 1;
        map
    });

    let mut answer = 0;
    for num in &first {
        answer += num * freq.get(&num).unwrap_or(&0);
    }
    answer
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1 (test): {}", part1(input));
    println!("Part 2 (test): {}", part2(input));
}

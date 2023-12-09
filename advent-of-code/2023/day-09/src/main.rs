use std::str::FromStr;

fn parse_input<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(str::parse::<T>)
                .map(Result::unwrap)
                .collect()
        })
        .collect()
}

fn calculate_differences_i(mut sequence: Vec<i64>) -> Vec<Vec<i64>> {
    let mut answer = vec![sequence.clone()];

    while !sequence.windows(2).all(|window| window[0] == window[1]) {
        sequence = sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        answer.push(sequence.clone());
    }
    answer
}

fn part1_brute(input: &str) -> i64 {
    let sequences = parse_input::<i64>(input);
    sequences
        .into_iter()
        .map(|sequence| {
            let mut all_differences_vec = calculate_differences_i(sequence);
            for i in (0..all_differences_vec.len() - 1).rev() {
                let added_value = all_differences_vec[i].last().unwrap()
                    + all_differences_vec[i + 1].last().unwrap();
                all_differences_vec[i].push(added_value);
            }
            let ans = *all_differences_vec[0].last().unwrap() as i64;
            ans
        })
        .sum()
}

fn part2_brute(input: &str) -> i64 {
    let sequences = parse_input::<i64>(input);
    sequences
        .into_iter()
        .map(|sequence| {
            let mut all_differences_vec = calculate_differences_i(sequence);
            for i in (0..all_differences_vec.len() - 1).rev() {
                let subtracted_value = all_differences_vec[i].first().unwrap()
                    - all_differences_vec[i + 1].first().unwrap();
                all_differences_vec[i][0] = subtracted_value;
            }
            all_differences_vec[0][0] as i64
        })
        .sum()
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1(brute): {}", part1_brute(test));
    println!("Part 2(brute): {}", part2_brute(test));

    // println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1(brute): {}", part1_brute(input));
    println!("Part 2(brute): {}", part2_brute(input));
}

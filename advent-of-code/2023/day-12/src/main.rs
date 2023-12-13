fn parse_input(input: &str) -> Vec<(Vec<u8>, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (chars, nums) = line.split_once(' ').unwrap();
            (
                chars.as_bytes().to_owned(),
                nums.split(',')
                    .map(str::parse::<usize>)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect()
}

use std::collections::BTreeMap;
fn solve2(
    line: &mut [u8],
    current_position: usize,
    current_grouping: usize,
    springs_so_far: usize,
    groupings: &[usize],
    cache: &mut BTreeMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(current_position, current_grouping, springs_so_far)) {
        return cached;
    }

    if current_position == line.len() {
        let ans = if current_grouping == groupings.len() {
            1
        } else {
            0
        };
        cache.insert((current_position, current_grouping, springs_so_far), ans);
        return ans;
    }

    let current_char = line[current_position];
    let ans = match current_char {
        b'.' => {
            if springs_so_far > 0 && springs_so_far < groupings[current_grouping] {
                return 0;
            }
            if current_grouping == groupings.len() || springs_so_far == 0 {
                solve2(
                    line,
                    current_position + 1,
                    current_grouping,
                    springs_so_far,
                    groupings,
                    cache,
                )
            } else {
                solve2(
                    line,
                    current_position + 1,
                    current_grouping + 1,
                    0,
                    groupings,
                    cache,
                )
            }
        }
        b'#' => {
            if current_grouping == groupings.len() {
                return 0;
            }
            if springs_so_far >= groupings[current_grouping] {
                return 0;
            }
            solve2(
                line,
                current_position + 1,
                current_grouping,
                springs_so_far + 1,
                groupings,
                cache,
            )
        }
        b'?' => {
            if current_grouping == groupings.len() {
                assert!(springs_so_far == 0);
                return solve2(
                    line,
                    current_position + 1,
                    current_grouping,
                    springs_so_far,
                    groupings,
                    cache,
                );
            }

            assert!(current_grouping < groupings.len());
            if springs_so_far == 0 {
                return solve2(
                    line,
                    current_position + 1,
                    current_grouping,
                    springs_so_far,
                    groupings,
                    cache,
                ) + solve2(
                    line,
                    current_position + 1,
                    current_grouping,
                    springs_so_far + 1,
                    groupings,
                    cache,
                );
            }

            if springs_so_far == groupings[current_grouping] {
                return solve2(
                    line,
                    current_position + 1,
                    current_grouping + 1,
                    0,
                    groupings,
                    cache,
                );
            }

            solve2(
                line,
                current_position + 1,
                current_grouping,
                springs_so_far + 1,
                groupings,
                cache,
            )
        }
        _ => unreachable!(),
    };

    cache.insert((current_position, current_grouping, springs_so_far), ans);
    ans
}

fn part1(input: &str) -> usize {
    let springs_and_groupings = parse_input(input);

    springs_and_groupings
        .into_iter()
        .map(|thing| multiply_by(thing, 1))
        .map(|(mut line, groupings)| {
            if line.iter().all(|&el| el == b'?') {
                fallback_solver(&line, &groupings)
            } else {
                line.push(b'.');
                let mut cache = BTreeMap::new();
                solve2(&mut line, 0, 0, 0, &groupings, &mut cache)
            }
        })
        .sum()
}

fn multiply_by(
    line_and_grouping: (Vec<u8>, Vec<usize>),
    multiplier: usize,
) -> (Vec<u8>, Vec<usize>) {
    let (line, groupings) = (line_and_grouping.0, line_and_grouping.1);
    let line_n = line.len();
    let groupings_n = groupings.len();
    (
        (line.into_iter().chain(std::iter::once(b'?')))
            .cycle()
            .take((line_n + 1) * multiplier - 1)
            .collect(),
        groupings
            .into_iter()
            .cycle()
            .take(groupings_n * multiplier)
            .collect(),
    )
}

fn n_choose_k(n: usize, k: usize) -> usize {
    let mut answer = 1;
    for i in 0..k {
        answer *= n - i;
        answer /= i + 1;
    }
    answer
}

fn fallback_solver(line: &[u8], grouping: &[usize]) -> usize {
    let empty_spaces: usize = line.len() - grouping.iter().sum::<usize>() - grouping.len() + 1;
    n_choose_k(grouping.len() + empty_spaces, grouping.len())
}

fn part2(input: &str) -> usize {
    let springs_and_groupings = parse_input(input);

    springs_and_groupings
        .into_iter()
        .map(|thing| multiply_by(thing, 5))
        .map(|(mut line, groupings)| {
            if line.iter().all(|&el| el == b'?') {
                let ans = fallback_solver(&line, &groupings);
                ans
            } else {
                line.push(b'.');
                let mut cache = BTreeMap::new();
                let answer = solve2(&mut line, 0, 0, 0, &groupings, &mut cache);
                answer
            }
        })
        .sum()
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

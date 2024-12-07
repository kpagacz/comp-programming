// https://adventofcode.com/2024/day/7
fn parse_input(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (result, nums) = line.split_once(": ").unwrap();
            let nums = nums
                .split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect();
            (result.parse::<i64>().unwrap(), nums)
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Multiply,
    Concatenate,
}

fn is_equation_valid(test_result: i64, nums: &[i64], possible_ops: &[Ops]) -> bool {
    fn rec(nums: &[i64], possible_ops: &[Ops], current_result: i64, target_result: i64) -> bool {
        if nums.is_empty() {
            return current_result == target_result;
        }
        if current_result > target_result {
            return false;
        }

        let next_num = nums[0];
        let mut result = false;
        for &op in possible_ops {
            match op {
                Ops::Add => {
                    result = result
                        || rec(
                            &nums[1..],
                            possible_ops,
                            current_result + next_num,
                            target_result,
                        );
                }
                Ops::Multiply => {
                    result = result
                        || rec(
                            &nums[1..],
                            possible_ops,
                            current_result * next_num,
                            target_result,
                        );
                }
                Ops::Concatenate => match format!("{current_result}{next_num}").parse::<i64>() {
                    Ok(current_result) => {
                        result =
                            result || rec(&nums[1..], possible_ops, current_result, target_result)
                    }
                    Err(_) => return false,
                },
            }
        }
        result
    }

    rec(&nums[1..], possible_ops, nums[0], test_result)
}

fn part1(input: &str) -> i64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter_map(|(test_result, nums)| {
            if is_equation_valid(test_result, &nums, &[Ops::Add, Ops::Multiply]) {
                Some(test_result)
            } else {
                None
            }
        })
        .sum::<i64>()
}

fn part2(input: &str) -> i64 {
    let equations = parse_input(input);

    equations
        .into_iter()
        .filter_map(|(test_result, nums)| {
            if is_equation_valid(
                test_result,
                &nums,
                &[Ops::Add, Ops::Multiply, Ops::Concatenate],
            ) {
                Some(test_result)
            } else {
                None
            }
        })
        .sum::<i64>()
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

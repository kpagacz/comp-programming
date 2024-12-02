// https://adventofcode.com/2024/day/02
fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .into_iter()
        .filter(|report| {
            let increasing = report[1] > report[0];
            report.windows(2).all(|window| {
                window[0].abs_diff(window[1]) <= 3 && window[0].abs_diff(window[1]) >= 1 && {
                    if increasing {
                        window[1] > window[0]
                    } else {
                        window[0] > window[1]
                    }
                }
            })
        })
        .count()
}

fn is_report_safe(report: &[i32]) -> bool {
    let increasing = report[1] > report[0];
    report.windows(2).all(|window| {
        window[0].abs_diff(window[1]) <= 3 && window[0].abs_diff(window[1]) >= 1 && {
            if increasing {
                window[1] > window[0]
            } else {
                window[0] > window[1]
            }
        }
    })
}

fn part2(input: &str) -> usize {
    let reports = parse_input(input);

    reports
        .into_iter()
        .filter(|report| {
            let mut any_report_safe = false;
            for i in 0..report.len() {
                let mut copy = report.clone();
                copy.remove(i);
                any_report_safe = is_report_safe(&copy) || any_report_safe;
            }
            any_report_safe || is_report_safe(report)
        })
        .count()
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

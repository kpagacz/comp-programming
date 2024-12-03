use std::sync::LazyLock;

static MUL_REGEX: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());

fn part1(input: &str) -> i64 {
    MUL_REGEX
        .captures_iter(input)
        .map(|caps| {
            let (_, [first, second]) = caps.extract();
            first.parse::<i64>().unwrap() * second.parse::<i64>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let mut spans = vec![];
    const DO: i32 = 0;
    const DONT: i32 = 1;
    for item in input.match_indices("do()") {
        spans.push((item.0, DO));
    }
    for item in input.match_indices("don't()") {
        spans.push((item.0, DONT));
    }
    spans.sort_unstable();

    let mut res = String::default();
    let mut last = 0usize;
    let mut enabled = true;
    for (id, t) in spans {
        match (enabled, t) {
            (true, DO) => {}
            (false, DO) => {
                enabled = true;
                last = id;
            }
            (true, DONT) => {
                res.push_str(&input[last..id]);
                enabled = false;
                last = id;
            }
            (false, DONT) => {}
            _ => unreachable!(),
        }
    }
    if enabled {
        res.push_str(&input[last..]);
    }

    part1(&res)
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

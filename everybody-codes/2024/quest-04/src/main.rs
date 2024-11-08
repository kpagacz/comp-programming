// https://everybody.codes/event/2024/quests/4
fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|nail| nail.parse::<usize>().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let nails = parse_input(input);
    let mut min = usize::MAX;
    let mut sum = 0;

    for &nail in &nails {
        min = min.min(nail);
        sum += nail;
    }

    sum - nails.len() * min
}

fn part3(input: &str) -> usize {
    let nails = parse_input(input);
    let min = *nails.iter().min().unwrap();
    let max = *nails.iter().max().unwrap();

    let calculate_hits = |level: usize| {
        nails
            .iter()
            .map(|&nail| level.abs_diff(nail))
            .sum::<usize>()
    };

    let possible = (min..=max).collect::<Vec<_>>();
    let ideal = possible.partition_point(|&candidate| {
        let current = calculate_hits(candidate);
        let next = calculate_hits(candidate + 1);

        current >= next
    }) + min;

    calculate_hits(ideal)
}

fn main() {
    let input = include_str!("../input1");
    println!("Part 1: {}", part1(input));
    let input2 = include_str!("../input2");
    println!("Part 2: {}", part1(input2));
    let input3 = include_str!("../input3");
    println!("Part 3: {}", part3(input3));
}

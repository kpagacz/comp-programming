fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            let num = line[1..].parse::<i32>().unwrap();
            if &line[0..1] == "R" { num } else { -num }
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let rotations = parse_input(input);
    let mut count = 0;
    let mut current_pos = 50;
    for rotation in rotations {
        current_pos += rotation;
        current_pos %= 100;
        if current_pos == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let rotations = parse_input(input);
    let mut count = 0;
    let mut current_pos = 50;
    for mut rotation in rotations {
        count += (rotation / 100).unsigned_abs() as usize;
        rotation %= 100;
        let after = current_pos + rotation;
        let mod_after = after % 100;
        if after * current_pos < 0 || after != mod_after || mod_after == 0 {
            count += 1;
        }
        current_pos = mod_after;
    }
    count
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}

use std::collections::HashSet;

fn part1(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split_whitespace().collect::<Vec<_>>();

    let mut answer = 0_i64;

    for line in lines {
        answer += line.parse::<i64>().unwrap();
    }

    println!("{}", answer);
}
fn part2(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let lines = input.split_whitespace().collect::<Vec<_>>();

    let mut mem = HashSet::new();
    let mut answer = 0_i64;
    let mut it = 0_usize;
    mem.insert(answer);

    loop {
        answer += lines[it].parse::<i64>().unwrap();
        it = (it + 1) % lines.len();
        if mem.contains(&answer) {
            println!("{} reached twice", answer);
            return;
        } else {
            mem.insert(answer);
        }
    }
}
fn main() {
    part1("input");
    part2("input");
}

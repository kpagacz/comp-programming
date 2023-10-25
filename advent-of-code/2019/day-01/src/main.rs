fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| 0.max(line.parse::<i64>().unwrap() / 3 - 2))
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut required_fuel = 0.max(line.parse::<i64>().unwrap() / 3 - 2);
            let mut last_fuel = required_fuel;
            while last_fuel > 0 {
                last_fuel = 0.max(last_fuel / 3 - 2);
                required_fuel += last_fuel;
            }
            required_fuel
        })
        .sum()
}
fn main() {
    let input = include_str!("../input");
    println!("{}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn parse_input(input: &str) -> (usize, usize, usize) {
    let mut lines = input.lines();
    (
        lines.next().unwrap().parse::<usize>().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
        lines.next().unwrap().parse::<usize>().unwrap(),
    )
}

fn part2(input: &str) -> usize {
    let (base, m, available_blocks) = parse_input(input);

    let mut current_blocks = 1usize;
    let mut current_width = 1usize;
    let mut curr_pow = 1;

    while current_blocks <= available_blocks {
        current_width += 2;
        curr_pow *= base;
        curr_pow %= m;

        current_blocks += curr_pow * current_width;
    }

    current_width * (current_blocks - available_blocks)
}

fn main() {
    let input2 = include_str!("../2-input");
    let test2 = include_str!("../2-test");
    println!("Part 2 (test): {}", part2(test2));
    println!("Part 2: {}", part2(input2));
}

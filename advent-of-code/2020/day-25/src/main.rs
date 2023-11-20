const MOD: usize = 20201227;

fn part1(input: &str) -> usize {
    const SUBJECT_NUMBER: usize = 7;
    let nums: Vec<usize> = input
        .split_whitespace()
        .map(|line| line.parse().unwrap())
        .collect();
    let (card_pkey, door_pkey) = (nums[0], nums[1]);

    fn find_loop_size(pkey: usize) -> usize {
        let mut start = 1;
        for i in 0.. {
            if start == pkey {
                return i;
            } else {
                start = (start * SUBJECT_NUMBER) % MOD;
            }
        }
        unreachable!()
    }

    let (card_loop_size, door_loop_size) = (find_loop_size(card_pkey), find_loop_size(door_pkey));
    println!("{card_loop_size} {door_loop_size}");
    let mut answer = 1;
    for _ in 0..(card_loop_size) {
        answer = (answer * door_pkey) % MOD;
    }
    answer
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
}

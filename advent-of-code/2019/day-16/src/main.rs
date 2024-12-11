fn parse_input(input: &str) -> Vec<i64> {
    input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|c| c as i64))
        .collect()
}

const BASE_PATTERN: [i64; 4] = [0, 1, 0, -1];
fn part1(input: &str) -> Vec<i64> {
    let mut digits = parse_input(input);

    for _ in 0..100 {
        let mut new_digits = vec![0; digits.len()];
        (0..digits.len()).for_each(|pos| {
            let pattern = BASE_PATTERN
                .into_iter()
                .flat_map(|dig| std::iter::repeat(dig).take(pos + 1))
                .cycle()
                .skip(1);

            let sum = digits
                .iter()
                .zip(pattern)
                .map(|(first, second)| *first * second)
                .sum::<i64>();
            new_digits[pos] = sum.abs() % 10;
        });
        digits = new_digits;
    }

    digits[..8].to_vec()
}

fn part2(input: &str) -> i32 {
    let input = parse_input(input);
    let input: Vec<i32> = input.into_iter().map(|i| i as i32).collect();
    let n = input.len();
    let mut digits: Vec<_> = input
        .clone()
        .into_iter()
        .map(|i| i as i32)
        .cycle()
        .take(n * 10_000)
        .collect();
    for i in 0..100 {
        println!("Phase: {i}");
        let mut new_digits = vec![0; digits.len()];
        (0..digits.len()).for_each(|pos| {
            let pattern = BASE_PATTERN
                .into_iter()
                .flat_map(|dig| std::iter::repeat(dig as i32).take(pos + 1))
                .cycle()
                .skip(1);

            let sum = digits
                .iter()
                .zip(pattern)
                .map(|(first, second)| *first * second)
                .sum::<i32>();
            new_digits[pos] = sum.abs() % 10;
        });
        digits = new_digits;
    }
    fn digs_to_num(v: &[i32], l: usize) -> usize {
        let mut offset = 1usize;
        for i in 0..l {
            offset *= 10;
            offset += v[i] as usize;
        }
        offset
    }
    let offset = digs_to_num(&input, 7);
    digs_to_num(&digits[offset..], 8) as i32
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    let base = include_str!("../base_test");
    println!("Part 1 (test): {:?}", part1(base));
    println!("Part 1 (test): {:?}", part1(test));
    println!("Part 1 (test): {:?}", part1(test2));
    println!("Part 1: {:?}", part1(input));

    let test3 = include_str!("../test3");
    println!("Part 2 (test): {}", part2(test3));
    println!("Part 2: {}", part2(input));
}

// https://adventofcode.com/2019/day/4

const PART1_RANGE: (u64, u64) = (134564, 585159);

fn digits(mut num: u64) -> Vec<u64> {
    let mut digits = vec![];

    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }

    digits
}

fn has_two_adjacent(digits: &[u64]) -> bool {
    digits.windows(2).any(|window| window[0] == window[1])
}

fn has_two_adjacent_only(digits: &[u64]) -> bool {
    (digits[0] == digits[1] && digits[1] != digits[2])
        || (digits[3] != digits[4] && digits[4] == digits[5])
        || digits.windows(4).any(|window| {
            window[0] != window[1] && window[3] != window[2] && window[1] == window[2]
        })
}

fn are_all_increasing(digits: &[u64]) -> bool {
    digits.windows(2).all(|window| window[0] >= window[1])
}

fn is_valid_password(
    num: u64,
    range: (u64, u64),
    adjacent_requirement: impl FnOnce(&[u64]) -> bool,
) -> bool {
    let digits = digits(num);
    (num >= range.0 && num <= range.1)
        && (adjacent_requirement(&digits))
        && (are_all_increasing(&digits))
}

fn answers(range: (u64, u64)) -> (u64, u64) {
    let ll = 100_000;
    let ul = 999_999;

    let mut first_part_count = 0;
    let mut second_part_count = 0;
    for num in ll..=ul {
        if is_valid_password(num, range, has_two_adjacent) {
            first_part_count += 1;
        }
        if is_valid_password(num, range, has_two_adjacent_only) {
            second_part_count += 1;
        }
    }

    (first_part_count, second_part_count)
}

fn main() {
    let (part1, part2) = answers(PART1_RANGE);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

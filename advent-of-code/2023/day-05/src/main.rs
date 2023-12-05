mod range;
use range::SeedRange;

fn map_numbers(number: &mut usize, map: &[&str]) {
    let map: Vec<_> = map.iter().filter_map(parse_line_to_map).collect();
    for (destination_range, source_range, length) in map {
        let old_end = source_range + length;
        let shift = destination_range as i64 - source_range as i64;
        if *number >= source_range && *number < old_end {
            *number = (*number as i64 + shift) as usize;
            return;
        }
    }
}

fn apply_map(numbers: &mut [usize], map: &[&str]) {
    let map = &map[1..];
    numbers.iter_mut().for_each(|num| map_numbers(num, map));
}

fn parse_line_to_map(line: &&str) -> Option<(usize, usize, usize)> {
    if line.is_empty() || line.starts_with(char::is_alphabetic) {
        None
    } else {
        let nums: Vec<_> = line
            .split_ascii_whitespace()
            .map(|num| num.parse::<usize>().unwrap())
            .collect();
        Some((nums[0], nums[1], nums[2]))
    }
}
fn part1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let mut numbers: Vec<_> = lines[0]
        .strip_prefix("seeds: ")
        .expect("It's static text")
        .split_ascii_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();

    let maps: Vec<_> = lines[2..].split(|line| line.is_empty()).collect();
    for map in maps {
        apply_map(&mut numbers, map);
    }
    numbers.into_iter().min().unwrap()
}

fn part2(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();
    let ranges: Vec<_> = lines[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .collect();
    let ranges: Vec<_> = ranges
        .chunks(2)
        .map(|chunk| {
            SeedRange::new(
                chunk[0].parse::<usize>().unwrap(),
                chunk[1].parse::<usize>().unwrap(),
            )
        })
        .collect();

    type Map = Vec<(SeedRange, i64)>;
    let maps: Vec<Map> = lines[2..]
        .split(|line| line.is_empty())
        .into_iter()
        .map(|map_lines| {
            map_lines[1..]
                .into_iter()
                .map(|&line| {
                    let nums: Vec<_> = line
                        .split_ascii_whitespace()
                        .map(str::parse::<i64>)
                        .map(Result::unwrap)
                        .collect();
                    (
                        SeedRange::new(nums[1] as usize, nums[2] as usize),
                        nums[0] - nums[1],
                    )
                })
                .collect()
        })
        .collect();

    // Action!
    ranges
        .into_iter()
        .map(|range| {
            let mut origin_ranges = vec![range];
            for map in &maps {
                let mut left_in_origin = origin_ranges.clone();
                let mut new_ranges = vec![];
                for (split_range, shift) in map {
                    let (new_left_in_origin, new_new_ranges) = left_in_origin
                        .into_iter()
                        .map(|origin_range| origin_range.split_with_shift(split_range, *shift))
                        .fold(
                            (vec![], vec![]),
                            |(mut acc_left_in_origin, mut acc_new_ranges), (el_left, el_new)| {
                                acc_left_in_origin.extend(el_left.into_iter());
                                acc_new_ranges.extend(el_new.into_iter());
                                (acc_left_in_origin, acc_new_ranges)
                            },
                        );
                    left_in_origin = new_left_in_origin;
                    new_ranges.extend(new_new_ranges.into_iter());
                }
                origin_ranges = left_in_origin
                    .into_iter()
                    .chain(new_ranges.into_iter())
                    .collect();
            }
            origin_ranges
                .into_iter()
                .map(|range| range.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

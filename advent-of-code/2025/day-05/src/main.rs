fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            let (min, max) = line.split_once("-").unwrap();
            (min.parse::<usize>().unwrap(), max.parse::<usize>().unwrap())
        })
        .collect();
    let ingredients = ingredients
        .lines()
        .map(|ingredient| ingredient.parse::<usize>().unwrap())
        .collect();
    (ranges, ingredients)
}

fn part1(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    let mut count = 0;
    for ingredient in ingredients {
        for &(min, max) in &ranges {
            if ingredient >= min && ingredient <= max {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_unstable();
    let mut last_range_end = 0usize;
    let mut count = 0;
    for (min, max) in ranges {
        let this_range_unique_begin = last_range_end.max(min);
        if this_range_unique_begin > max {
            continue;
        }
        count += max - this_range_unique_begin + 1;
        last_range_end = last_range_end.max(max + 1);
    }
    count
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {}  expected: 3", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {}  expected 14", part2(test));
    let my_test = include_str!("../my_test");
    println!("Part 2 (my_test): {}  expected 14", part2(my_test));
    println!("Part 2: {}", part2(input));
}

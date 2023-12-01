fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let digits: Vec<_> = line
                .chars()
                .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
                .collect();
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            let mut replaced_line = String::from(line);
            if let Some((pos, digit_less_one)) = digit_words
                .iter()
                .enumerate()
                .filter_map(|(pos, digit_word)| {
                    replaced_line
                        .find(digit_word)
                        .and_then(|found| Some((found, pos)))
                })
                .min()
            {
                replaced_line.insert_str(pos, &(digit_less_one + 1).to_string());
            }
            if let Some((pos, digit_less_one)) = digit_words
                .iter()
                .enumerate()
                .filter_map(|(pos, digit_word)| {
                    replaced_line
                        .rfind(digit_word)
                        .and_then(|found| Some((found, pos)))
                })
                .max()
            {
                replaced_line.insert_str(pos, &(digit_less_one + 1).to_string());
            }
            let digits: Vec<_> = replaced_line
                .chars()
                .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
                .collect();
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}
fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    let test = include_str!("../test2");
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

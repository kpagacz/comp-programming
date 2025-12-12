use regex::Regex;
fn parse_input(input: &str) -> Vec<(usize, usize, Vec<usize>)> {
    let re = Regex::new("([0-9]+)x([0-9]+): (.*)").unwrap();
    input
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|captures| {
                (
                    captures[1].parse::<usize>().unwrap(),
                    captures[2].parse::<usize>().unwrap(),
                    captures[3]
                        .split_whitespace()
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>(),
                )
            })
        })
        .collect()
}

const DOTS_IN_PRESENTS: [usize; 6] = [5, 7, 7, 6, 7, 7];

fn does_region_fit_presents(region_size: (usize, usize), presents: &[usize]) -> bool {
    let min_required_space = DOTS_IN_PRESENTS
        .iter()
        .zip(presents)
        .map(|(dots, count)| *dots * *count)
        .sum::<usize>();
    let available_area = region_size.0 * region_size.1;
    println!("min_required_space: {min_required_space} available area: {available_area}");
    if available_area < min_required_space {
        false
    } else {
        let total_presents = presents.iter().sum::<usize>();
        let available_3x3_squares = region_size.0 / 3 * region_size.1 / 3;
        println!("total_presents: {total_presents} available_squares: {available_3x3_squares}");
        total_presents <= available_3x3_squares
    }
}

fn part1(input: &str) -> usize {
    let cases = parse_input(input);

    cases
        .into_iter()
        .filter(|(region_x, region_y, presents_required)| {
            does_region_fit_presents((*region_x, *region_y), presents_required)
        })
        .count()
}
fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
}

use regex::Regex;

fn parse_input(input: &str) -> Vec<(char, usize, String)> {
    let re = Regex::new(r"^(.) ([0-9]+) \(#(.+)\)$").unwrap();
    input
        .lines()
        .map(|line| {
            let Some(caps) = re.captures(line) else {
                panic!("Failed to regex: {line}");
            };
            (
                caps[1].parse::<char>().unwrap(),
                caps[2].parse::<usize>().unwrap(),
                caps[3].to_string(),
            )
        })
        .collect()
}

fn construct_polygon(directions: &[(char, usize, String)]) -> Vec<(isize, isize)> {
    let mut polygon = vec![];
    let mut initial_point = (0, 0);
    directions.iter().for_each(|(dir, steps, _)| {
        match dir {
            'R' => initial_point.1 += *steps as isize,
            'L' => initial_point.1 -= *steps as isize,
            'D' => initial_point.0 += *steps as isize,
            'U' => initial_point.0 -= *steps as isize,
            _ => unreachable!(),
        }
        polygon.push(initial_point);
    });

    polygon
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Orientation {
    Positive, // counter clockwise
    Negative, // clockwise
}

fn area(polygon: &[(isize, isize)], directions: &[(char, usize, String)]) -> f64 {
    let from_circumference = directions
        .iter()
        .map(|(_, length, _)| *length)
        .sum::<usize>() as f64
        / 2.0;
    let cycled: Vec<_> = polygon.iter().chain(std::iter::once(&polygon[0])).collect();
    let shoelace = cycled
        .windows(2)
        .map(|window| {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];
            *x1 * *y2 - *x2 * *y1
        })
        .sum::<isize>()
        / 2;
    let polygon_orientation = if shoelace > 0 {
        Orientation::Positive
    } else {
        Orientation::Negative
    };
    let cycled_directions: Vec<_> = directions
        .iter()
        .chain(std::iter::once(&directions[0]))
        .collect();
    let (add_quarter_corner, subtract_quarer_corner) =
        cycled_directions
            .windows(2)
            .fold((0, 0), |corners, window| {
                let corner_orientation = match (window[0].0, window[1].0) {
                    ('R', 'D') | ('D', 'L') | ('L', 'U') | ('U', 'R') => Orientation::Negative,
                    _ => Orientation::Positive,
                };
                if polygon_orientation == corner_orientation {
                    (corners.0 + 1, corners.1)
                } else {
                    (corners.0, corners.1 + 1)
                }
            });
    shoelace.unsigned_abs() as f64 + from_circumference + (add_quarter_corner as f64 / 4.0)
        - (subtract_quarer_corner as f64 / 4.0)
}

fn update_directions_for_part2(
    original_directions: Vec<(char, usize, String)>,
) -> Vec<(char, usize, String)> {
    original_directions
        .into_iter()
        .map(|(_, _, hex)| {
            let num = usize::from_str_radix(&hex[..5], 16).expect("Hex should be parsable");
            let dir = match hex.chars().nth(5).unwrap() {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => unreachable!(),
            };
            (dir, num, hex)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let directions = parse_input(input);
    let polygon = construct_polygon(&directions);

    area(&polygon, &directions) as usize
}

fn part2(input: &str) -> usize {
    let directions = parse_input(input);
    let directions = update_directions_for_part2(directions);
    let polygon = construct_polygon(&directions);

    area(&polygon, &directions) as usize
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

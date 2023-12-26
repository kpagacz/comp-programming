type Position = (usize, usize, usize);
type Velocity = (isize, isize, isize);
type Hailstone = (Position, Velocity);

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" @ ").unwrap();
            let position = position
                .split(", ")
                .map(str::parse::<usize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            let position = (position[0], position[1], position[2]);
            let velocity = velocity
                .split(", ")
                .map(str::parse::<isize>)
                .map(Result::unwrap)
                .collect::<Vec<_>>();
            let velocity = (velocity[0], velocity[1], velocity[2]);
            (position, velocity)
        })
        .collect()
}

const PRECISION: f64 = 0.001;
fn calculate_intersection(first: &Hailstone, second: &Hailstone) -> Option<(f64, f64)> {
    let first_slope = first.1 .1 as f64 / first.1 .0 as f64;
    let second_slope = second.1 .1 as f64 / second.1 .0 as f64;
    // y = ax + b
    // b = y - ax
    let first_b = first.0 .1 as f64 - first.0 .0 as f64 * first_slope;
    let second_b = second.0 .1 as f64 - second.0 .0 as f64 * second_slope;
    // x = (b2 - b1) / (a1 - a2)
    if (first_slope - second_slope).abs() < PRECISION {
        return None;
    }
    let intersection_x = (second_b - first_b) / (first_slope - second_slope);
    let intersection_y = first_slope * intersection_x + first_b;

    Some((intersection_x, intersection_y))
}

fn did_cross_in_the_past(first: &Hailstone, second: &Hailstone, intersection: &(f64, f64)) -> bool {
    for ((x, _, _), (delta_x, _, _)) in [first, second] {
        if ((intersection.0 - *x as f64) / *delta_x as f64) < 0.0 {
            return true;
        }
    }
    false
}

fn do_paths_cross_in_test_zone(
    first: &Hailstone,
    second: &Hailstone,
    zone_start: f64,
    zone_end: f64,
) -> bool {
    if let Some((intersection_x, intersection_y)) = calculate_intersection(first, second) {
        intersection_x >= zone_start
            && intersection_x <= zone_end
            && intersection_y >= zone_start
            && intersection_y <= zone_end
            && !did_cross_in_the_past(first, second, &(intersection_x, intersection_y))
    } else {
        false
    }
}

fn part1(input: &str, zone_start: usize, zone_end: usize) -> usize {
    let hailstones = parse_input(input);
    let (zone_start, zone_end) = (zone_start as f64, zone_end as f64);
    let mut answer = 0;
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            if do_paths_cross_in_test_zone(&hailstones[i], &hailstones[j], zone_start, zone_end) {
                answer += 1;
            }
        }
    }
    answer
}

fn part2(input: &str) -> usize {
    let hailstones = parse_input(input);
    let mut answer = 0;
    let mut cache = vec![];
    for i in 0..hailstones.len() {
        for j in (i + 1)..hailstones.len() {
            if do_paths_cross_in_test_zone(&hailstones[i], &hailstones[j], f64::MIN, f64::MAX) {
                answer += 1;
                let intersection = calculate_intersection(&hailstones[i], &hailstones[j]).unwrap();
                cache.push(intersection);
            }
        }
    }
    cache.sort_by(|a, b| a.partial_cmp(b).unwrap());
    cache
        .windows(2)
        .map(|window| (window[1].0 - window[0].0, window[1].1 - window[0].1))
        .for_each(|diff| println!("{diff:?}"));
    answer
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test, 7, 27));
    // println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input, 200000000000000, 400000000000000));
    println!("Part 2: {}", part2(input));
}

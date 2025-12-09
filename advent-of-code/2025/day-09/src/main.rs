use std::ops::Add;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| line.split_once(",").unwrap())
        .map(|(first, second)| {
            (
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn area_from_corners(first: (usize, usize), second: (usize, usize)) -> usize {
    first.0.abs_diff(second.0).add(1) * first.1.abs_diff(second.1).add(1)
}

fn part1(input: &str) -> usize {
    let red_tiles = parse_input(input);

    let mut max_area = 0;
    for i in 0..red_tiles.len() {
        for j in i + 1..red_tiles.len() {
            max_area = max_area.max(area_from_corners(red_tiles[i], red_tiles[j]) as _);
        }
    }
    max_area
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Point(usize, usize);
#[derive(Debug, Copy, Clone)]
struct Line(Point, Point);

fn get_lines(points: &[(usize, usize)]) -> Vec<Line> {
    let last_point = *points.last().unwrap();
    let last = Line(
        Point(last_point.0, last_point.1),
        Point(points[0].0, points[0].1),
    );
    points
        .windows(2)
        .take(points.len())
        .map(|window| {
            let first = Point(window[0].0, window[0].1);
            let second = Point(window[1].0, window[1].1);
            Line(first, second)
        })
        .chain(std::iter::once(last))
        .collect()
}

fn get_corners(first_corner: (usize, usize), second_corner: (usize, usize)) -> Vec<(usize, usize)> {
    let third_corner = (first_corner.0, second_corner.1);
    let fourth_corner = (second_corner.0, first_corner.1);
    vec![first_corner, second_corner, third_corner, fourth_corner]
}

// This is grossly not sufficient when there is a large concave area
// but thankfully for the inputs, this is not the case
fn is_edge_outside(edge: Line, rectangle: &[(usize, usize)]) -> bool {
    let min_x = rectangle.iter().map(|point| point.0).min().unwrap();
    let max_x = rectangle.iter().map(|point| point.0).max().unwrap();
    let min_y = rectangle.iter().map(|point| point.1).min().unwrap();
    let max_y = rectangle.iter().map(|point| point.1).max().unwrap();
    let edge_min_x = edge.0.0.min(edge.1.0);
    let edge_max_x = edge.0.0.max(edge.1.0);
    let edge_min_y = edge.0.1.min(edge.1.1);
    let edge_max_y = edge.0.1.max(edge.1.1);

    let left = edge_max_x <= min_x;
    let right = edge_min_x >= max_x;
    let above = edge_min_y >= max_y;
    let below = edge_max_y <= min_y;

    left || right || above || below
}

fn part2(input: &str) -> usize {
    let points = parse_input(input);
    let edges = get_lines(&points);

    let mut max_area = 0;
    for first_corner in 0..points.len() {
        'outer: for second_corner in first_corner + 1..points.len() {
            let first_corner = points[first_corner];
            let second_corner = points[second_corner];
            let corners = get_corners(first_corner, second_corner);
            for edge in &edges {
                if !is_edge_outside(*edge, &corners) {
                    continue 'outer;
                }
            }
            max_area = max_area.max(area_from_corners(first_corner, second_corner));
        }
    }

    max_area
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {}, expected: 50", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {}, expected: 24", part2(test));
    println!("Part 2: {}", part2(input));
}

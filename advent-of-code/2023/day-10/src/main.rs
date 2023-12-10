// PART 1
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let mut lines: Vec<_> = input
        .lines()
        .map(|line| {
            std::iter::once(b'.')
                .chain(line.as_bytes().to_owned())
                .chain(std::iter::once(b'.'))
                .collect::<Vec<u8>>()
        })
        .collect();
    lines.insert(0, vec![b'.'; lines[0].len()]);
    lines.push(vec![b'.'; lines[0].len()]);
    lines
}

type Point = (usize, usize);
fn find_start(grid: &Vec<Vec<u8>>) -> Point {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

const NEIGHBOURS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
fn get_neighbours(point: &Point, pipe: &u8) -> Vec<Point> {
    match pipe {
        b'|' => vec![(point.0 - 1, point.1), (point.0 + 1, point.1)],
        b'-' => vec![(point.0, point.1 - 1), (point.0, point.1 + 1)],
        b'L' => vec![(point.0 - 1, point.1), (point.0, point.1 + 1)],
        b'J' => vec![(point.0 - 1, point.1), (point.0, point.1 - 1)],
        b'7' => vec![(point.0, point.1 - 1), (point.0 + 1, point.1)],
        b'F' => vec![(point.0 + 1, point.1), (point.0, point.1 + 1)],
        b'S' => NEIGHBOURS
            .iter()
            .map(|(delta_x, delta_y)| (point.0 as i32 + delta_x, point.1 as i32 + delta_y))
            .map(|(x, y)| (x as usize, y as usize))
            .collect(),
        _ => vec![],
    }
}

fn part1(input: &str) -> (usize, Point) {
    let grid = parse_input(input);
    let start = find_start(&grid);
    let mut queue = vec![(start, start, 0)]; // (point, source, steps)
    while let Some((top_point, source, steps)) = queue.pop() {
        let targets: Vec<_> = get_neighbours(&top_point, &grid[top_point.0][top_point.1])
            .into_iter()
            .filter(|&point| point != source)
            .collect();
        if targets.iter().any(|&(x, y)| grid[x][y] == b'S') {
            return ((steps + 1) / 2, top_point);
        }
        targets
            .into_iter()
            .for_each(|target| queue.push((target, top_point, steps + 1)));
    }
    unreachable!();
}

// PART 2
use std::collections::BTreeSet;
fn flood_fill(from: &Point, with: u8, grid: &mut [Vec<u8>], restricted: &BTreeSet<Point>) {
    if grid[from.0][from.1] == with || restricted.contains(from) {
        return;
    }
    grid[from.0][from.1] = with;
    let n = grid.len();
    let row_n = grid[0].len();
    NEIGHBOURS
        .iter()
        .map(|(delta_x, delta_y)| (from.0 as i32 + delta_x, from.1 as i32 + delta_y))
        .filter(|&(x, y)| x >= 0 && x < n as i32 && y >= 0 && y < row_n as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|point| !restricted.contains(point))
        .for_each(|point| flood_fill(&point, with, grid, restricted))
}

use std::collections::BTreeMap;
fn reconstruct_loop(
    mut from: Point,
    mut previous: Point,
    grid: &[Vec<u8>],
) -> BTreeMap<usize, Point> {
    let mut loop_map = BTreeMap::new();
    loop_map.insert(0, from);

    while grid[from.0][from.1] != b'S' {
        let next = get_neighbours(&from, &grid[from.0][from.1])
            .into_iter()
            .find(|&point| point != previous)
            .unwrap();
        previous = from;
        from = next;
        loop_map.insert(loop_map.len(), from);
    }

    loop_map
}

fn orientation(first: Point, second: Point, to_orient: Point) -> u8 {
    let first = (first.0 as i32, first.1 as i32);
    let second = (second.0 as i32, second.1 as i32);
    let to_orient = (to_orient.0 as i32, to_orient.1 as i32);
    match (first.0 - to_orient.0) * (second.1 - to_orient.1)
        - (second.0 - to_orient.0) * (first.1 - to_orient.1)
        > 0
    {
        true => b'X',
        false => b'Y',
    }
}

const EXTENDED_NEIGHBOURS: [(i32, i32); 8] = [
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn get_extended_neighbours(point: &Point, grid: &[Vec<u8>]) -> BTreeSet<Point> {
    EXTENDED_NEIGHBOURS
        .iter()
        .map(|(delta_x, delta_y)| (point.0 as i32 + delta_x, point.1 as i32 + delta_y))
        .filter(|&(x, y)| x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}

fn mark_orientation(grid: &mut [Vec<u8>], pipe_loop: &BTreeMap<usize, Point>) {
    let idx: Vec<_> = (0..pipe_loop.len()).chain(std::iter::once(0)).collect();
    let pipe_set: BTreeSet<_> = pipe_loop.values().copied().collect();
    idx.windows(2)
        .map(|window| {
            let (first, second) = (window[0], window[1]);
            (
                pipe_loop.get(&first).unwrap(),
                pipe_loop.get(&second).unwrap(),
            )
        })
        .for_each(|(first, second)| {
            let first_neighbours = get_extended_neighbours(first, grid);
            let second_neighbours = get_extended_neighbours(second, grid);
            let common = first_neighbours
                .intersection(&second_neighbours)
                .filter(|&&(x, y)| !pipe_set.contains(&(x, y)))
                .copied()
                .collect::<Vec<_>>();
            common.into_iter().for_each(|common_neighbour| {
                let neighbour_orientation = orientation(*first, *second, common_neighbour);
                flood_fill(&common_neighbour, neighbour_orientation, grid, &pipe_set);
            });
        })
}

fn count_inside(grid: &[Vec<u8>]) -> usize {
    let inside_char = if grid[0][0] == b'X' { b'Y' } else { b'X' };
    grid.iter()
        .map(|line| line.iter().filter(|&&c| c == inside_char).count())
        .sum()
}

// Get the piece of the pipe that is part of the loop from part 1.
// This lets us trace back all the points in the loop.
// Having just the start does not let us do that because start can go any way.
fn part2(input: &str, point_before_start: Point) -> usize {
    let mut grid = parse_input(input);
    let start = find_start(&grid);
    let loop_points = reconstruct_loop(point_before_start, start, &grid);
    mark_orientation(&mut grid, &loop_points);
    count_inside(&grid)
}

// Using Pick's theorem and the shoelace formula
fn part2_smarter(input: &str, point_before_start: Point) -> i32 {
    let grid = parse_input(input);
    let start = find_start(&grid);
    let loop_points = reconstruct_loop(point_before_start, start, &grid);
    let loop_points: Vec<_> = loop_points.values().copied().collect();

    let shoelace_array: Vec<_> = loop_points
        .iter()
        .chain(std::iter::once(&loop_points[0]))
        .collect();

    let shoelace_area: i32 = shoelace_array
        .windows(2)
        .map(|window| {
            let (x1, y1) = *window[0];
            let (x2, y2) = *window[1];
            x1 as i32 * y2 as i32 - x2 as i32 * y1 as i32
        })
        .sum::<i32>()
        .abs()
        / 2;
    // Pick theorem in action
    // A = i + b / 2 - 1.
    // where A = area, i = number of interior points, b = number of boundary points
    // i = A - b / 2 + 1
    shoelace_area - loop_points.len() as i32 / 2 + 1
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    let (farthest, _) = part1(test);
    println!("Part 1: {}", farthest);
    let test = include_str!("../test2");
    let (_, before_start) = part1(test);
    println!("Part 2: {}", part2(test, before_start));
    println!("Part 2 smarter: {}", part2_smarter(test, before_start));
    println!("ANOTHER TEST FOR PART 2");
    let test = include_str!("../test3");
    let (_, before_start) = part1(test);
    println!("Part 2: {}", part2(test, before_start));
    println!("Part 2 smarter: {}", part2_smarter(test, before_start));
    println!("ANOTHER TEST FOR PART 2");
    let test = include_str!("../test4");
    let (_, before_start) = part1(test);
    println!("Part 2: {}", part2(test, before_start));
    println!("Part 2 smarter: {}", part2_smarter(test, before_start));

    println!("INPUT");
    let input = include_str!("../input");
    let (farthest, before_start) = part1(input);
    println!("Part 1: {}", farthest);
    println!("Part 2: {}", part2(input, before_start));
    println!("Part 2 smarter: {}", part2_smarter(input, before_start));
}

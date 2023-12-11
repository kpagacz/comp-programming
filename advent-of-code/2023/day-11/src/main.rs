fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

use std::collections::BTreeSet;
fn find_empty_rows(image: &[Vec<u8>]) -> BTreeSet<usize> {
    image
        .iter()
        .enumerate()
        .filter_map(|(pos, row)| {
            if row.iter().all(|c| c == &b'.') {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
}

fn find_empty_columns(image: &[Vec<u8>]) -> BTreeSet<usize> {
    (0..image[0].len())
        .filter(|&col| {
            for row in image {
                if row[col] != b'.' {
                    return false;
                }
            }
            true
        })
        .collect()
}

type Point = (i64, i64);
fn find_galaxies(image: &[Vec<u8>]) -> Vec<Point> {
    image
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, c)| {
                    if *c == b'#' {
                        Some((i as i64, j as i64))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .filter(|arr| !arr.is_empty())
        .flatten()
        .collect()
}

fn sum_distances(
    galaxies: &[Point],
    empty_rows: &BTreeSet<usize>,
    empty_cols: &BTreeSet<usize>,
    expansion: i64,
) -> i64 {
    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (first, second) = (galaxies[i], galaxies[j]);
            total += (first.0 - second.0).abs() + (first.1 - second.1).abs();
            let (min_x, max_x) = (first.0.min(second.0), first.0.max(second.0));
            let (min_y, max_y) = (first.1.min(second.1), first.1.max(second.1));
            for row in min_x..max_x {
                if empty_rows.contains(&(row as usize)) {
                    total += expansion;
                }
            }
            for col in min_y..max_y {
                if empty_cols.contains(&(col as usize)) {
                    total += expansion;
                }
            }
        }
    }
    total
}

fn part1(input: &str) -> i64 {
    let image = parse_input(input);

    let empty_rows = find_empty_rows(&image);
    let empty_cols = find_empty_columns(&image);

    let galaxies = find_galaxies(&image);

    sum_distances(&galaxies, &empty_rows, &empty_cols, 1)
}

fn part2(input: &str) -> i64 {
    let image = parse_input(input);

    let empty_rows = find_empty_rows(&image);
    let empty_cols = find_empty_columns(&image);

    let galaxies = find_galaxies(&image);

    sum_distances(&galaxies, &empty_rows, &empty_cols, 1000000 - 1)
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

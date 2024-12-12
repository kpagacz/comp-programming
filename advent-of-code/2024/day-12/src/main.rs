// https://adventofcode.com/2024/day/12
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const NEIGHBOURS: [(usize, usize); 4] = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];
fn flood_fill(
    x: usize,
    y: usize,
    garden: &mut [Vec<char>],
    area: &mut usize,
    fences: &mut usize,
    plot_type: char,
) {
    if garden[x][y] != plot_type {
        return;
    }
    *area += 1;
    *fences += 4;
    let rows = garden.len();
    let cols = garden[0].len();
    garden[x][y] = plot_type.to_lowercase().next().unwrap();

    let ns: Vec<_> = NEIGHBOURS
        .into_iter()
        .map(|(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
        .filter(|(x, y)| *x < rows && *y < cols)
        .collect();

    for &(nx, ny) in &ns {
        if garden[nx][ny] == garden[x][y] || garden[nx][ny] == plot_type {
            *fences -= 1;
        }
    }

    let unvisited_ns: Vec<_> = ns
        .into_iter()
        .filter(|&(nx, ny)| garden[nx][ny] == plot_type)
        .collect();
    for (x, y) in unvisited_ns {
        flood_fill(x, y, garden, area, fences, plot_type);
    }
}

fn part1(input: &str) -> usize {
    let mut garden = parse_input(input);
    let mut total_cost = 0;

    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            if garden[row][col].is_uppercase() {
                let mut area = 0;
                let mut fences = 0;
                let plot_type = garden[row][col];
                flood_fill(row, col, &mut garden, &mut area, &mut fences, plot_type);
                total_cost += area * fences;
            }
        }
    }

    total_cost
}

use std::collections::HashSet;
fn find_points_in_plot(
    x: usize,
    y: usize,
    garden: &[Vec<char>],
    points: &mut HashSet<(usize, usize)>,
    plot_type: char,
) {
    if points.contains(&(x, y)) {
        return;
    }
    points.insert((x, y));

    let rows = garden.len();
    let cols = garden[0].len();

    let ns: Vec<_> = NEIGHBOURS
        .into_iter()
        .map(|(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
        .filter(|&(x, y)| x < rows && y < cols && garden[x][y] == plot_type)
        .collect();

    for (x, y) in ns {
        find_points_in_plot(x, y, garden, points, plot_type);
    }
}

const SPECIAL: char = '$';
fn paint_plot(garden: &mut [Vec<char>], points: &HashSet<(usize, usize)>, with: char) {
    for &(x, y) in points {
        garden[x][y] = with;
    }
}

// A line algorithm sort of
fn count_sides(plot: HashSet<(usize, usize)>, garden: &mut [Vec<char>]) -> usize {
    let &(x, y) = plot.iter().next().unwrap();
    let plot_type = garden[x][y];
    // Paint the plot to avoid counting separate plots with the same char
    paint_plot(garden, &plot, SPECIAL);

    // LINE!
    let mut sides = 0;
    let (minx, maxx, miny, maxy) = plot.iter().fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |acc, &point| {
            (
                acc.0.min(point.0),
                acc.1.max(point.0),
                acc.2.min(point.1),
                acc.3.max(point.1),
            )
        },
    );

    #[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy)]
    enum Side {
        InOut,
        OutIn,
    }

    //// Rows
    let mut last_char = 'x';
    let mut last_row = vec![];
    for row in minx..=maxx {
        let mut new_row = vec![];
        for col in miny..=maxy + 1 {
            if last_char != SPECIAL && col < garden[0].len() && garden[row][col] == SPECIAL {
                new_row.push((Side::OutIn, col));
                if !last_row.contains(&(Side::OutIn, col)) {
                    sides += 1;
                }
            }
            if last_char == SPECIAL && (col >= garden[0].len() || garden[row][col] != SPECIAL) {
                new_row.push((Side::InOut, col));
                if !last_row.contains(&(Side::InOut, col)) {
                    sides += 1;
                }
            }
            if col < garden[0].len() {
                last_char = garden[row][col];
            }
        }
        last_row = new_row;
        last_char = 'x';
    }

    //// Columns
    let mut last_char = 'x';
    let mut last_col = vec![];
    for col in miny..=maxy {
        let mut new_col = vec![];
        for row in minx..=maxx + 1 {
            if last_char != SPECIAL && row < garden.len() && garden[row][col] == SPECIAL {
                new_col.push((Side::OutIn, row));
                if !last_col.contains(&(Side::OutIn, row)) {
                    sides += 1;
                }
            }
            if last_char == SPECIAL && (row >= garden.len() || garden[row][col] != SPECIAL) {
                new_col.push((Side::InOut, row));
                if !last_col.contains(&(Side::InOut, row)) {
                    sides += 1;
                }
            }
            if row < garden.len() {
                last_char = garden[row][col];
            }
        }
        last_col = new_col;
        last_char = 'x';
    }

    paint_plot(garden, &plot, plot_type);
    sides
}

fn part2(input: &str) -> usize {
    let mut garden = parse_input(input);
    let mut total_cost = 0;

    for row in 0..garden.len() {
        for col in 0..garden[0].len() {
            if garden[row][col].is_uppercase() {
                let mut plot = HashSet::default();
                let plot_type = garden[row][col];
                find_points_in_plot(row, col, &garden, &mut plot, plot_type);
                let sides = count_sides(plot, &mut garden);
                let mut area = 0;
                let mut fences = 0;
                flood_fill(row, col, &mut garden, &mut area, &mut fences, plot_type);
                total_cost += sides * area;
            }
        }
    }

    total_cost
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    let test3 = include_str!("../test3");
    let test4 = include_str!("../test4");
    let test5 = include_str!("../test5");

    println!("Part 1 (test): {}", part1(test));
    println!("Part 1 (test): {}", part1(test2));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {} expected: 80", part2(test));
    println!("Part 2 (test): {} expected 436", part2(test2));
    println!("Part 2 (test): {} expected 236", part2(test3));
    println!("Part 2 (test): {} expected 368", part2(test4));
    println!("Part 2 (test): {} expected 1206", part2(test5));
    println!("Part 2: {}", part2(input));
}

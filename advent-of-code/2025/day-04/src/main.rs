#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Paper,
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|c| match c {
                    b'.' => Tile::Empty,
                    b'@' => Tile::Paper,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

const DIRECTIONS: [(usize, usize); 8] = [
    (usize::MAX, usize::MAX),
    (usize::MAX, 0),
    (usize::MAX, 1),
    (0, usize::MAX),
    (0, 1),
    (1, usize::MAX),
    (1, 0),
    (1, 1),
];

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == Tile::Paper {
                let adjacent_papers = DIRECTIONS
                    .into_iter()
                    .filter_map(|(dx, dy)| {
                        let new_x = row.wrapping_add(dx);
                        let new_y = col.wrapping_add(dy);
                        if new_x < rows && new_y < cols {
                            Some((new_x, new_y))
                        } else {
                            None
                        }
                    })
                    .filter(|(x, y)| grid[*x][*y] == Tile::Paper)
                    .count();
                if adjacent_papers < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let mut count = 0;
    use std::collections::HashSet;

    loop {
        let mut removed_paper = HashSet::new();
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == Tile::Paper {
                    let adjacent_papers = DIRECTIONS
                        .into_iter()
                        .filter_map(|(dx, dy)| {
                            let new_x = row.wrapping_add(dx);
                            let new_y = col.wrapping_add(dy);
                            if new_x < rows && new_y < cols {
                                Some((new_x, new_y))
                            } else {
                                None
                            }
                        })
                        .filter(|(x, y)| grid[*x][*y] == Tile::Paper)
                        .count();
                    if adjacent_papers < 4 {
                        removed_paper.insert((row, col));
                        count += 1;
                    }
                }
            }
        }

        if removed_paper.is_empty() {
            break;
        }

        for (x, y) in removed_paper {
            grid[x][y] = Tile::Empty;
        }
    }
    count
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {}  expected 13", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {}  expected 43", part2(test));
    println!("Part 2: {}", part2(input));
}

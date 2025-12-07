const VISITED: char = 'x';
const FREE: char = '.';
const SPLITTER: char = '^';
const START: char = 'S';

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> (usize, usize) {
    (
        0,
        grid[0]
            .iter()
            .position(|c| *c == START)
            .expect("Start exists"),
    )
}

fn walk_beam(start: (usize, usize), splitting_count: &mut usize, grid: &mut [Vec<char>]) {
    let (mut row, col) = start;
    while row < grid.len() {
        match grid[row][col] {
            VISITED => {
                break;
            }
            FREE | START => {
                grid[row][col] = VISITED;
                row += 1;
            }
            SPLITTER => {
                *splitting_count += 1;
                if col > 0 && grid[row][col - 1] == FREE {
                    walk_beam((row, col - 1), splitting_count, grid);
                }
                if col < grid[0].len() - 1 && grid[row][col + 1] == FREE {
                    walk_beam((row, col + 1), splitting_count, grid);
                }
                break;
            }
            _ => unreachable!(
                "No other chars expected on the grid, found {}",
                grid[row][col]
            ),
        }
    }
}

use std::collections::HashMap;

fn walk_beam_quantum(
    start: (usize, usize),
    grid: &[Vec<char>],
    mem: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(cached) = mem.get(&start) {
        return *cached;
    }
    let (mut row, col) = start;
    let mut count = 0usize;
    while row < grid.len() {
        match grid[row][col] {
            FREE | START => {
                row += 1;
            }
            SPLITTER => {
                count += 1;
                if col > 0 && grid[row][col - 1] == FREE {
                    count += walk_beam_quantum((row, col - 1), grid, mem);
                }
                if col < grid[0].len() - 1 && grid[row][col + 1] == FREE {
                    count += walk_beam_quantum((row, col + 1), grid, mem);
                }
                break;
            }
            _ => unreachable!(
                "No other chars expected on the grid, found {}",
                grid[row][col]
            ),
        }
    }

    mem.insert(start, count);
    count
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    let mut split = 0;
    let start = find_start(&grid);
    walk_beam(start, &mut split, &mut grid);
    split
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let start = find_start(&grid);
    let mut mem = HashMap::new();
    let res = walk_beam_quantum(start, &grid, &mut mem);
    res + 1
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {} expected 21", part1(test));
    println!("Part 1: {}", part1(input));
    println!("Part 2 (test): {} expected 40", part2(test));
    println!("Part 2: {}", part2(input));
}

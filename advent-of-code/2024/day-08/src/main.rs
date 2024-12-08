// https://adventofcode.com/2024/day/8
use std::collections::HashMap;
use std::collections::HashSet;
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn in_bounds(row: isize, col: isize, map: &[Vec<char>]) -> bool {
    row >= 0 && col >= 0 && row < map.len() as isize && col < map[0].len() as isize
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    let (rows, cols) = (map.len(), map[0].len());
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::default();

    (0..rows).for_each(|row| {
        (0..cols).for_each(|col| {
            if map[row][col] != '.' {
                antennas.entry(map[row][col]).or_default().push((row, col));
            }
        })
    });

    let mut antinodes: HashSet<(isize, isize)> = HashSet::default();
    for same_freq_antennas in antennas.values() {
        for (pos, this) in same_freq_antennas.iter().enumerate() {
            for other in &same_freq_antennas[pos + 1..] {
                let row_diff = other.0 as isize - this.0 as isize;
                let col_diff = other.1 as isize - this.1 as isize;

                let first_antinode = (this.0 as isize - row_diff, this.1 as isize - col_diff);
                let second_antinode = (other.0 as isize + row_diff, other.1 as isize + col_diff);
                if in_bounds(first_antinode.0, first_antinode.1, &map) {
                    antinodes.insert(first_antinode);
                }
                if in_bounds(second_antinode.0, second_antinode.1, &map) {
                    antinodes.insert(second_antinode);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let (rows, cols) = (map.len(), map[0].len());
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::default();

    (0..rows).for_each(|row| {
        (0..cols).for_each(|col| {
            if map[row][col] != '.' {
                antennas.entry(map[row][col]).or_default().push((row, col));
            }
        })
    });

    let mut antinodes: HashSet<(isize, isize)> = HashSet::default();
    for same_freq_antennas in antennas.values() {
        for (pos, this) in same_freq_antennas.iter().enumerate() {
            for other in &same_freq_antennas[pos + 1..] {
                let row_diff = other.0 as isize - this.0 as isize;
                let col_diff = other.1 as isize - this.1 as isize;

                let mut first_antinode = (this.0 as isize, this.1 as isize);
                while in_bounds(first_antinode.0, first_antinode.1, &map) {
                    antinodes.insert(first_antinode);
                    first_antinode = (first_antinode.0 - row_diff, first_antinode.1 - col_diff);
                }
                let mut second_antinode = (other.0 as isize, other.1 as isize);
                while in_bounds(second_antinode.0, second_antinode.1, &map) {
                    antinodes.insert(second_antinode);
                    second_antinode = (second_antinode.0 + row_diff, second_antinode.1 + col_diff);
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");

    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}

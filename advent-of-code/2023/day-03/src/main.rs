const NEIGHBOURS: [(i32, i32); 8] = [ (1, -1), (1, 0), (1, 1), (0, -1), (0, 1), (-1, -1), (-1, 0), (-1, 1), ];

// Read input as a grid
fn lines_to_vec(input: &str) -> Vec<Vec<u8>> {
    let lines: Vec<_> = input.lines().collect();
    let mut out = vec![vec![b'.'; lines[0].len() + 2]; lines.len() + 2];
    let cols_n = out[0].len();
    for (row, &line) in lines.iter().enumerate() {
        out[row + 1][1..cols_n - 1].copy_from_slice(line.as_bytes());
    }

    out
}

// Vector of positions in the grid
type Number = Vec<(usize, usize)>;

// Get all the numbers in the grid
fn scan_for_numbers(grid: &[Vec<u8>]) -> Vec<Number> {
    let mut numbers = vec![];
    for (row, line) in grid.iter().enumerate() {
        let mut number = vec![];
        let mut it = 0;
        while it < line.len() {
            if line[it].is_ascii_digit() == false {
                if !number.is_empty() {
                    numbers.push(number);
                    number = vec![];
                }
            } else {
                number.push((row, it));
            }
            it += 1;
        }
    }
    numbers
}

// Transform Number to an actual usize
fn number_to_usize(number: &Number, grid: &[Vec<u8>]) -> usize {
    String::from_utf8(number.iter().map(|&(row, col)| grid[row][col]).collect())
        .unwrap()
        .parse::<usize>()
        .expect("Number should always parse because it consists of digits")
}

fn part1(input: &str) -> usize {
    let grid = lines_to_vec(input);
    // for line in &grid {
    //     println!("{line:?}");
    // }
    let numbers = scan_for_numbers(&grid);
    // for number in &numbers {
    //     println!("{number:?}");
    // }
    fn has_neighbouring_symbol(number: &Number, grid: &[Vec<u8>]) -> bool {
        number.iter().any(|&(digit_row, digit_col)| {
            NEIGHBOURS
                .iter()
                .map(|(delta_row, delta_col)| {
                    (
                        (digit_row as i32 + delta_row) as usize,
                        (digit_col as i32 + delta_col) as usize,
                    )
                })
                .any(|(neighbour_row, neighbour_col)| {
                    grid[neighbour_row][neighbour_col] != b'.'
                        && !grid[neighbour_row][neighbour_col].is_ascii_digit()
                })
        })
    }

    let numbers_with_neighbour_symbols: Vec<_> = numbers
        .into_iter()
        .filter(|number| has_neighbouring_symbol(number, &grid))
        .collect();
    // println!("Number with neighbouring symbols");
    // for number in &numbers_with_neighbour_symbols {
    //     println!("{number:?}");
    // }
    numbers_with_neighbour_symbols
        .into_iter()
        .map(|number| number_to_usize(&number, &grid))
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = lines_to_vec(input);
    let numbers = scan_for_numbers(&grid);
    use std::collections::{BTreeMap, BTreeSet};
    let mut gears = BTreeMap::new();

    fn neighbouring_stars(number: &Number, grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
        number
            .iter()
            .map(|&(digit_row, digit_col)| {
                NEIGHBOURS
                    .iter()
                    .map(|(delta_row, delta_col)| {
                        (
                            (digit_row as i32 + delta_row) as usize,
                            (digit_col as i32 + delta_col) as usize,
                        )
                    })
                    .filter(|&(neighbour_row, neighbour_col)| {
                        grid[neighbour_row][neighbour_col] == b'*'
                    })
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect()
    }
    for number in &numbers {
        neighbouring_stars(number, &grid)
            .into_iter()
            .for_each(|star| {
                gears
                    .entry(star)
                    .or_insert(BTreeSet::new())
                    .insert(number.clone());
            });
    }
    gears
        .into_iter()
        .filter_map(|(_, numbers)| {
            if numbers.len() != 2 {
                None
            } else {
                Some(
                    numbers
                        .into_iter()
                        .map(|number| number_to_usize(&number, &grid))
                        .reduce(|acc, num| acc * num)
                        .expect("There are 2 numbers"),
                )
            }
        })
        .sum()
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

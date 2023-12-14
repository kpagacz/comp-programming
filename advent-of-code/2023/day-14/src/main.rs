type Grid = Vec<Vec<u8>>;
fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn transpose(grid: &[Vec<u8>]) -> Grid {
    let mut answer = vec![vec![0; grid.len()]; grid[0].len()];
    for i in 0..answer.len() {
        for j in 0..answer[0].len() {
            answer[i][j] = grid[j][i];
        }
    }
    answer
}

fn calculate_load(grid: &[Vec<u8>]) -> usize {
    let grid = transpose(grid);
    grid.into_iter()
        .map(|row| {
            let row_len = row.len();
            row.into_iter()
                .enumerate()
                .filter_map(|(pos, c)| if c == b'O' { Some(row_len - pos) } else { None })
                .sum::<usize>()
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let mut grid = parse_input(input);
    tilt_to_north(&mut grid);
    calculate_load(&grid)
}

use std::collections::VecDeque;
fn tilt_to_west(grid: &mut [Vec<u8>]) {
    grid.iter_mut().for_each(|row| {
        let mut stones = VecDeque::new();
        let mut current_stones = 0;
        row.iter().for_each(|c| match c {
            b'O' => current_stones += 1,
            b'#' => {
                stones.push_back(current_stones);
                current_stones = 0;
            }
            _ => {}
        });
        stones.push_back(current_stones);

        for c in row.iter_mut() {
            if c == &b'.' && stones[0] > 0 {
                *c = b'O';
                stones[0] -= 1;
            } else if c == &b'#' {
                stones.pop_front();
            } else if c == &b'O' && stones[0] == 0 {
                *c = b'.';
            } else if c == &b'O' && stones[0] > 0 {
                stones[0] -= 1;
            }
        }
    });
}

fn transpose_mut(grid: &mut Grid) {
    let mut answer = vec![vec![0; grid.len()]; grid[0].len()];
    for i in 0..answer.len() {
        for j in 0..answer[0].len() {
            answer[i][j] = grid[j][i];
        }
    }
    *grid = answer;
}

fn tilt_to_north(grid: &mut Grid) {
    transpose_mut(grid);
    tilt_to_west(grid);
    transpose_mut(grid);
}

fn tilt_to_south(grid: &mut Grid) {
    grid.reverse();
    tilt_to_north(grid);
    grid.reverse();
}

fn tilt_to_east(grid: &mut Grid) {
    transpose_mut(grid);
    tilt_to_south(grid);
    transpose_mut(grid);
}

fn cycle(grid: &mut Grid) -> (usize, usize, usize, usize) {
    tilt_to_north(grid);
    let first = calculate_load(grid);
    tilt_to_west(grid);
    let second = calculate_load(grid);
    tilt_to_south(grid);
    let third = calculate_load(grid);
    tilt_to_east(grid);
    let fourth = calculate_load(grid);
    (first, second, third, fourth)
}

fn part2(input: &str) -> usize {
    let mut grid = parse_input(input);

    use std::collections::BTreeMap;
    type LoadCache = BTreeMap<(usize, usize, usize, usize), usize>;

    let mut load_cache = LoadCache::new();

    for i in 0..200 {
        let loads = cycle(&mut grid);
        println!("Loads: {loads:?} at {i}");
        if let Some(&cached) = load_cache.get(&loads) {
            println!("Found cycle starting after: {cached} cycles that loops after {i} cycles");
            let loop_length = i - cached;
            let remaining = (1_000_000_000 - i - 1) % loop_length;
            for _ in 0..remaining {
                cycle(&mut grid);
            }
            return calculate_load(&grid);
        } else {
            load_cache.insert(loads, i);
        }
    }

    unreachable!()
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

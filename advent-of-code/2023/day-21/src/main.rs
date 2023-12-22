const BORDER: u8 = b'$';
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    let grid = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let mut answer = vec![vec![BORDER; grid[0].len() + 2]; grid.len() + 2];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            answer[i + 1][j + 1] = grid[i][j];
        }
    }
    answer
}

fn find_start(grid: &[Vec<u8>]) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'S' {
                return (i, j);
            }
        }
    }
    unreachable!()
}

const NEIGHBOURS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
fn calculate_distances(start: (usize, usize), grid: &[Vec<u8>]) -> Vec<Vec<isize>> {
    let mut answer = vec![vec![-1; grid[0].len()]; grid.len()];
    use std::collections::VecDeque;
    let mut queue = VecDeque::from([(start, 0)]);
    use std::collections::BTreeSet;
    let mut seen = BTreeSet::new();

    while let Some((pos, steps)) = queue.pop_front() {
        if seen.contains(&pos) {
            continue;
        }
        let (x, y) = (pos.0, pos.1);
        if grid[x][y] == BORDER || grid[x][y] == b'#' {
            continue;
        }
        answer[x][y] = steps;
        NEIGHBOURS
            .iter()
            .map(|(delta_x, delta_y)| (x as isize + delta_x, y as isize + delta_y))
            .map(|(x, y)| (x as usize, y as usize))
            .for_each(|pos| queue.push_back((pos, steps + 1)));

        seen.insert(pos);
    }
    answer
}

use std::collections::BTreeMap;
fn calculate_plots_from(
    start: (usize, usize),
    range: isize,
    grid: &[Vec<u8>],
    cache: &mut BTreeMap<((usize, usize), isize), usize>,
) -> usize {
    if let Some(&cached) = cache.get(&(start, range)) {
        return cached;
    }
    let distances = calculate_distances(start, grid);

    let mut answer = 0;
    for i in 0..distances.len() {
        for j in 0..distances[0].len() {
            if distances[i][j] != -1 && distances[i][j] <= range && distances[i][j] % 2 == range % 2
            {
                answer += 1;
            }
        }
    }

    cache.insert((start, range), answer);
    answer
}

fn part1(input: &str, range: isize) -> usize {
    let grid = parse_input(input);
    let start = find_start(&grid);
    let mut cache = BTreeMap::new();
    calculate_plots_from(start, range, &grid, &mut cache)
}
fn maps_with_diameter(n: isize) -> usize {
    (((n - 1) * n) * 2 + 1) as _
}

fn number_of_plots_from_middle(even_range: bool, grid: &[Vec<u8>]) -> usize {
    let start = find_start(grid);
    let mut cache = BTreeMap::new();
    if even_range {
        calculate_plots_from(start, 300, grid, &mut cache)
    } else {
        calculate_plots_from(start, 301, grid, &mut cache)
    }
}

fn plots_from_border_maps(map_range: usize, grid: &[Vec<u8>]) -> usize {
    let mut cache = BTreeMap::new();
    let mut answer = 0;

    for start in [
        (grid.len() - 2, grid[0].len() - 2),
        (grid.len() - 2, 1),
        (1, 1),
        (1, grid[0].len() - 2),
    ] {
        answer += map_range * calculate_plots_from(start, 64, grid, &mut cache);
        answer += (map_range - 1) * calculate_plots_from(start, 195, grid, &mut cache);
    }

    for start in [
        (grid.len() / 2, 1),
        (grid.len() / 2, grid[0].len() - 2),
        (1, grid[0].len() / 2),
        (grid.len() - 2, grid[0].len() / 2),
    ] {
        answer += calculate_plots_from(start, 130, grid, &mut cache);
    }

    answer
}

const STEPS: usize = 26501365;
fn part2(input: &str) -> usize {
    let grid = parse_input(input);

    let middle_plots_even = number_of_plots_from_middle(true, &grid);
    let middle_plots_odd = number_of_plots_from_middle(false, &grid);
    println!("Even plots: {middle_plots_even} Odd plots: {middle_plots_odd}");
    println!("Max sum of map's coordinates: {}", STEPS / 131);
    let max_map_range = STEPS / 131;

    let mut plots = middle_plots_odd;
    for range in 2..=max_map_range {
        let border_maps =
            maps_with_diameter(range as isize) - maps_with_diameter(range as isize - 1);
        if range % 2 == 0 {
            plots += border_maps * middle_plots_even;
        } else {
            plots += border_maps * middle_plots_odd;
        }
    }
    println!("Plots from maps that are not a border: {}", plots);

    let plots_from_borders = plots_from_border_maps(max_map_range, &grid);
    println!(
        "Plots from maps that are on a border: {}",
        plots_from_borders
    );
    plots + plots_from_borders
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test, 6));
    // println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input, 64));
    println!("Part 2: {}", part2(input))
}

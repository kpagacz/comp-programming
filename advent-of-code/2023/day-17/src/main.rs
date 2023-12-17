fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.as_bytes()
                .into_iter()
                .map(|&c| c as usize - 48)
                .collect()
        })
        .collect()
}

type State = (usize, (isize, isize), usize, usize); // (cost, (row, col), direction, moves_so_far

use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;

const NEXT_MOVES: [[(isize, isize); 3]; 4] = [
    [(0, -1), (0, 1), (-1, 0)], // NORTH
    [(-1, 0), (1, 0), (0, 1)],  // EAST
    [(0, 1), (0, -1), (1, 0)],  // SOUTH
    [(-1, 0), (1, 0), (0, -1)], // WEST
];

fn map_delta_to_direction(delta_x: isize, delta_y: isize) -> usize {
    match (delta_x, delta_y) {
        (-1, 0) => 0,
        (0, 1) => 1,
        (1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    }
}

// Modified Djikstra
fn move_cauldrons(
    mut states: BinaryHeap<Reverse<State>>,
    min_move: usize,
    max_move: usize,
    grid: &[Vec<usize>],
) -> usize {
    let mut seen = BTreeSet::new();
    let mut lowest_costs = vec![vec![usize::MAX; grid[0].len()]; grid.len()];

    while let Some(top) = states.pop() {
        let (cost, (row, col), direction, moves_so_far) = top.0;

        // End conditions
        if seen.contains(&(row, col, direction, moves_so_far)) {
            continue;
        }
        if row == grid.len() as isize - 1 && col == grid[0].len() as isize - 1 {
            return cost;
        }

        let is_in_grid = |point: (isize, isize)| {
            let (x, y) = point;
            x >= 0 && y >= 0 && x < grid.len() as isize && y < grid[0].len() as isize
        };

        // Update the path cost for the current path
        seen.insert((row, col, direction, moves_so_far));
        lowest_costs[row as usize][col as usize] = cost;

        // Branch out!
        let next_moves = NEXT_MOVES[direction];
        if moves_so_far >= min_move {
            for &(delta_x, delta_y) in &next_moves[..2] {
                let new_point = (row + delta_x, col + delta_y);
                if is_in_grid(new_point) {
                    let new_cost = cost + grid[new_point.0 as usize][new_point.1 as usize];
                    states.push(Reverse((
                        new_cost,
                        new_point,
                        map_delta_to_direction(delta_x, delta_y),
                        1,
                    )));
                }
            }
        }
        if moves_so_far < max_move {
            let (delta_x, delta_y) = next_moves[2];
            let new_point = (row + delta_x, col + delta_y);
            if is_in_grid(new_point) {
                let new_cost = cost + grid[new_point.0 as usize][new_point.1 as usize];
                states.push(Reverse((new_cost, new_point, direction, moves_so_far + 1)));
            }
        }
    }

    lowest_costs[grid.len() - 1][grid[0].len() - 1]
}

fn part1(input: &str) -> usize {
    let grid = parse_input(input);
    let initial_state = BinaryHeap::from([Reverse((0, (0, 0), 1, 0))]);
    move_cauldrons(initial_state, 0, 3, &grid)
}

fn part2(input: &str) -> usize {
    let grid = parse_input(input);
    let initial_state = BinaryHeap::from([Reverse((0, (0, 0), 1, 0))]);
    move_cauldrons(initial_state, 4, 10, &grid)
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    let end = start.elapsed();
    println!("Took: {:?}", end);
    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    let end = start.elapsed();
    println!("Took: {:?}", end);
}

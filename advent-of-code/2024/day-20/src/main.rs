use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const END: char = 'E';
const START: char = 'S';
const WALL: char = '#';
const NEIGHBOURS: [(usize, usize); 4] = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];
fn djikstra(map: &[Vec<char>], start: (usize, usize)) -> usize {
    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Reverse((0, start.0, start.1))); // cost, x, y

    while let Some(Reverse((cost, x, y))) = pq.pop() {
        if map[x][y] == END {
            return cost;
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        NEIGHBOURS
            .into_iter()
            .map(|(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
            .filter(|&(x, y)| x < map.len() && y < map[0].len() && map[x][y] != WALL)
            .for_each(|(x, y)| pq.push(Reverse((cost + 1, x, y))));
    }
    unreachable!()
}

fn calculate_distance_without_cheats(map: &[Vec<char>]) -> HashMap<(usize, usize), usize> {
    let mut costs = HashMap::new();

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] != WALL {
                costs.insert((x, y), djikstra(map, (x, y)));
            }
        }
    }

    costs
}

fn find_start(map: &[Vec<char>]) -> (usize, usize) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == START {
                return (x, y);
            }
        }
    }
    unreachable!()
}

fn cheated_positions(
    map: &[Vec<char>],
    allowed_cheats: usize,
    start: (usize, usize),
) -> HashSet<(usize, usize, usize)> {
    let mut cheated_positions = HashSet::new();
    cheated_positions.insert((start.0, start.1, 0));
    let mut visited = HashSet::new();
    let mut moves = 0;

    while moves < allowed_cheats {
        let mut new_ns = vec![];
        for &position in &cheated_positions {
            let (x, y, cheat_time) = position;
            NEIGHBOURS
                .into_iter()
                .map(|(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
                .filter(|&(x, y)| x < map.len() && y < map[0].len())
                .for_each(|pos| {
                    if !visited.contains(&pos) {
                        visited.insert(pos);
                        new_ns.push((pos.0, pos.1, cheat_time + 1));
                    }
                });
        }
        for n in new_ns {
            cheated_positions.insert(n);
        }

        moves += 1;
    }

    cheated_positions
        .into_iter()
        .filter(|&(x, y, _)| map[x][y] != WALL)
        .collect()
}

fn find_cheats_saving_time(
    map: &[Vec<char>],
    start: (usize, usize),
    limit: usize,
    times: &HashMap<(usize, usize), usize>,
    allowed_cheats: usize,
) -> usize {
    let mut time = 0;
    let mut positions = vec![start];
    let mut effective_cheats = 0;
    let mut visited = HashSet::new();

    while !positions.is_empty() {
        let mut new_positions = Vec::new();

        for position in positions {
            visited.insert(position);
            let normal_time = time + *times.get(&position).unwrap();

            cheated_positions(map, allowed_cheats, position)
                .into_iter()
                .for_each(|pos| {
                    let cheated_time = time + pos.2 + *times.get(&(pos.0, pos.1)).unwrap();
                    if cheated_time + limit <= normal_time {
                        // println!("Effective cheat found. Normal time {normal_time} cheated_time: {cheated_time} original pos {position:?} cheated pos: {pos:?}");
                        effective_cheats += 1;
                    }
                });

            NEIGHBOURS
                .into_iter()
                .map(|(dx, dy)| (position.0 + dx, position.1 + dy))
                .filter(|&(x, y)| {
                    x < map.len()
                        && y < map[0].len()
                        && map[x][y] != WALL
                        && !visited.contains(&(x, y))
                })
                .for_each(|pos| new_positions.push(pos));
        }

        time += 1;
        positions = new_positions;
    }

    effective_cheats
}

fn part1(input: &str, limit: usize, allowed_cheats: usize) -> usize {
    let map = parse_input(input);
    let times = calculate_distance_without_cheats(&map);
    let start = find_start(&map);

    find_cheats_saving_time(&map, start, limit, &times, allowed_cheats)
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");

    println!("Part 1 (test): {}, expected 44", part1(test, 1, 2));
    println!("Part 1: {}", part1(input, 100, 2));

    println!("Part 2 (test): {}, expected 41", part1(test, 70, 20));
    println!("Part 2: {}", part1(input, 100, 20));
}

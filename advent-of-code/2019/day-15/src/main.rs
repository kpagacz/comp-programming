use intcode::{IntCode, IntUnit};

mod intcode;

use std::collections::{HashMap, HashSet};
const NORTH: i32 = 1;
const SOUTH: i32 = 2;
const WEST: i32 = 3;
const EAST: i32 = 4;
const WALL: i32 = 0;
const MOVED: i32 = 1;
const OXYGEN: i32 = 2;
const DIRECTIONS: [(i32, i32); 5] = [(0, 0), (-1, 0), (1, 0), (0, -1), (0, 1)];
const EMPTY: char = '.';
const BORDER: char = '#';
const STATION: char = 'E';
const UNKNOWN: char = ' ';

fn map_area(intcode: &mut IntCode) -> HashMap<(i32, i32), char> {
    let mut map = HashMap::default();
    let (mut x, mut y) = (0, 0);
    map.insert((x, y), EMPTY);
    let mut walk_queue = vec![1, 2, 3, 4];

    while let Some(next_move) = walk_queue.pop() {
        let (dx, dy) = DIRECTIONS[next_move as usize];
        let next_x = x + dx;
        let next_y = y + dy;
        let res = intcode.run(vec![next_move]);
        match res[0] as i32 {
            WALL => {
                map.insert((next_x, next_y), BORDER);
            }
            MOVED => {
                x = next_x;
                y = next_y;
                if !map.contains_key(&(x, y)) {
                    map.insert((x, y), EMPTY);
                    let backtrack = {
                        let mut n = next_move - 1;
                        n ^= 1;
                        n += 1;
                        n
                    };
                    walk_queue.push(backtrack);
                    [1, 2, 3, 4].into_iter().for_each(|m| {
                        let (dx, dy) = DIRECTIONS[m as usize];
                        let next_x = x + dx;
                        let next_y = y + dy;
                        if !map.contains_key(&(next_x, next_y)) {
                            walk_queue.push(m);
                        }
                    });
                }
            }
            OXYGEN => {
                x = next_x;
                y = next_y;
                if !map.contains_key(&(x, y)) {
                    map.insert((x, y), STATION);
                    let backtrack = {
                        let mut n = next_move - 1;
                        n ^= 1;
                        n += 1;
                        n
                    };
                    walk_queue.push(backtrack);
                    [1, 2, 3, 4].into_iter().for_each(|m| {
                        let (dx, dy) = DIRECTIONS[m as usize];
                        let next_x = x + dx;
                        let next_y = y + dy;
                        if !map.contains_key(&(next_x, next_y)) {
                            walk_queue.push(m);
                        }
                    });
                }
            }
            _ => unreachable!(),
        }
    }

    map
}

fn part1(input: &str) -> usize {
    let mut intcode = IntCode::from(input);
    let map = map_area(&mut intcode);
    println!(
        "Station: {:?}",
        map.iter()
            .filter_map(|(key, value)| {
                if *value == 'E' {
                    Some(key)
                } else {
                    None
                }
            })
            .next()
            .unwrap()
    );

    for i in (-21..=21) {
        for j in (-25..=25) {
            if i == 0 && j == 0 {
                print!("S");
            } else {
                print!("{}", map.get(&(i, j)).unwrap_or(&' '));
            }
        }
        println!()
    }

    fn bfs(
        pos: Vec<(i32, i32)>,
        visited: &mut HashSet<(i32, i32)>,
        it: usize,
        map: &HashMap<(i32, i32), char>,
    ) -> usize {
        if pos.iter().any(|(x, y)| *map.get(&(*x, *y)).unwrap() == 'E') {
            it
        } else {
            pos.iter().for_each(|(x, y)| {
                visited.insert((*x, *y));
            });
            bfs(
                pos.into_iter()
                    .flat_map(|(x, y)| {
                        DIRECTIONS
                            .into_iter()
                            .map(|(dx, dy)| (x + dx, y + dy))
                            .collect::<Vec<_>>()
                    })
                    .filter(|(x, y)| {
                        !visited.contains(&(*x, *y))
                            && map.get(&(*x, *y)).map(|c| *c == '.' || *c == 'E') == Some(true)
                    })
                    .collect(),
                visited,
                it + 1,
                map,
            )
        }
    }

    bfs(vec![(0, 0)], &mut HashSet::default(), 0, &map)
}

fn part2(input: &str) -> usize {
    let mut intcode = IntCode::from(input);
    let map = map_area(&mut intcode);
    fn bfs(
        pos: Vec<(i32, i32)>,
        visited: &mut HashSet<(i32, i32)>,
        it: usize,
        map: &HashMap<(i32, i32), char>,
    ) -> usize {
        if pos.is_empty() {
            it
        } else {
            pos.iter().for_each(|(x, y)| {
                visited.insert((*x, *y));
            });
            bfs(
                pos.into_iter()
                    .flat_map(|(x, y)| {
                        DIRECTIONS
                            .into_iter()
                            .map(|(dx, dy)| (x + dx, y + dy))
                            .collect::<Vec<_>>()
                    })
                    .filter(|(x, y)| {
                        !visited.contains(&(*x, *y))
                            && map.get(&(*x, *y)).map(|c| *c == '.' || *c == 'E') == Some(true)
                    })
                    .collect(),
                visited,
                it + 1,
                map,
            )
        }
    }
    bfs(
        vec![*map
            .iter()
            .filter_map(|(key, value)| if *value == 'E' { Some(key) } else { None })
            .next()
            .unwrap()],
        &mut HashSet::default(),
        0,
        &map,
    ) - 1
}

fn main() {
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

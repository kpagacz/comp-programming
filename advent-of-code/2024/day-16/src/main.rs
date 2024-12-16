fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

const WALL: char = '#';
const START: char = 'S';
const END: char = 'E';
fn part1(input: &str) -> usize {
    let map = parse_input(input);
    djikstra(&map, find_start(&map))
}

fn find_start(map: &[Vec<char>]) -> (usize, usize) {
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == START {
                return (row, col);
            }
        }
    }
    unreachable!()
}

fn get_next(map: &[Vec<char>], x: usize, y: usize, step: (usize, usize)) -> char {
    map[x.wrapping_add(step.0)][y.wrapping_add(step.1)]
}

const DIRECTIONS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)];
fn djikstra(map: &[Vec<char>], start: (usize, usize)) -> usize {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    let mut heap = BinaryHeap::default(); // const, x, y, direction
    let mut visited: HashSet<(usize, usize, usize)> = HashSet::default();

    heap.push(Reverse((0, start.0, start.1, 0)));

    while !heap.is_empty() {
        let Reverse((cost, x, y, direction)) = heap.pop().unwrap();

        if map[x][y] == END {
            return cost;
        }

        if visited.contains(&(x, y, direction)) {
            continue;
        } else {
            visited.insert((x, y, direction));
        }

        // println!("{cost} {x} {y} {direction} {}", map[x][y]);
        for dir_change in [0, 1, 3] {
            // no point reversing, right?
            let step = DIRECTIONS[(direction + dir_change) % 4];

            if get_next(map, x, y, step) != WALL {
                heap.push(Reverse((
                    cost + 1 + 1000 * (dir_change % 2),
                    x.wrapping_add(step.0),
                    y.wrapping_add(step.1),
                    (direction + dir_change) % 4,
                )));
            }
        }
    }
    unreachable!()
}

fn djikstra_with_fucking_benches(map: &[Vec<char>], start: (usize, usize)) -> usize {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut heap = BinaryHeap::default(); // const, x, y, direction
    let mut visited: HashMap<(usize, usize, usize), usize> = HashMap::default();

    heap.push(Reverse((0, start.0, start.1, 0, vec![])));

    let mut good_for_sitting = HashSet::new();
    let mut min_cost = usize::MAX;
    while !heap.is_empty() {
        let Reverse((cost, x, y, direction, path)) = heap.pop().unwrap();

        if map[x][y] == END {
            if cost > min_cost {
                return good_for_sitting.len();
            } else {
                min_cost = cost;
                good_for_sitting.extend(path);
                continue;
            }
        }
        if cost > min_cost {
            return good_for_sitting.len();
        }

        if let Some(&seen_cost) = visited.get(&(x, y, direction)) {
            if cost > seen_cost {
                continue;
            }
        } else {
            visited.insert((x, y, direction), cost);
        }

        // println!("{cost} {x} {y} {direction} {}", map[x][y]);
        for dir_change in [0, 1, 3] {
            // no point reversing, right?
            let step = DIRECTIONS[(direction + dir_change) % 4];

            if get_next(map, x, y, step) != WALL {
                let mut copy = path.clone();
                copy.push((x.wrapping_add(step.0), y.wrapping_add(step.1)));
                heap.push(Reverse((
                    cost + 1 + 1000 * (dir_change % 2),
                    x.wrapping_add(step.0),
                    y.wrapping_add(step.1),
                    (direction + dir_change) % 4,
                    copy,
                )));
            }
        }
    }
    unreachable!()
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    djikstra_with_fucking_benches(&map, find_start(&map)) + 1
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");

    println!("Part 1 (test): {}, expected 7036", part1(test));
    println!("Part 1 (test): {}, expected 11048", part1(test2));
    println!("Partd 1: {}", part1(input));

    println!("Part 2 (test): {}, expected 45", part2(test));
    println!("Part 2 (test): {}, expected 64", part2(test2));
    println!("Part 2: {}", part2(input));
}

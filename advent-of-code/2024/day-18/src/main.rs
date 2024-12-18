fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(',').unwrap();
            (
                first.parse::<usize>().unwrap(),
                second.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn drop_stones(map: &mut [Vec<char>], count: usize, stones: &[(usize, usize)]) {
    stones.iter().take(count).for_each(|stone| {
        map[stone.0][stone.1] = '#';
    });
}

const NEIGHBOURS: [(usize, usize); 4] = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];
fn djikstra(map: &[Vec<char>], start: (usize, usize)) -> Option<usize> {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    let mut pq = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    pq.push(Reverse((0, start.0, start.1)));

    while let Some(Reverse((cost, x, y))) = pq.pop() {
        if x == map.len() - 1 && y == map[0].len() - 1 {
            return Some(cost);
        }
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        NEIGHBOURS
            .into_iter()
            .map(|(dx, dy)| (x.wrapping_add(dx), y.wrapping_add(dy)))
            .filter(|&(new_x, new_y)| {
                new_x < map.len() && new_y < map[0].len() && map[new_x][new_y] != '#'
            })
            .for_each(|(x, y)| pq.push(Reverse((cost + 1, x, y))));
    }
    None
}

fn part1(input: &str) -> usize {
    let mut map = vec![vec!['.'; 71]; 71];
    let stones = parse_input(input);

    drop_stones(&mut map, 1024, &stones);
    djikstra(&map, (0, 0)).unwrap()
}

fn part2(input: &str) -> (usize, usize) {
    let stones = parse_input(input);
    let map = vec![vec!['.'; 71]; 71];

    let possible: Vec<_> = (0..stones.len()).collect();
    let pos = possible.partition_point(|&dropped_stones| {
        let mut map_copy = map.clone();
        drop_stones(&mut map_copy, dropped_stones, &stones);
        djikstra(&map_copy, (0, 0)).is_some()
    });

    stones[pos - 1]
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

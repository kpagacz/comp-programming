fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

const NEIGHBOURS: [(usize, usize); 4] = [(usize::MAX, 0), (1, 0), (0, usize::MAX), (0, 1)];

fn part1(input: &str) -> usize {
    let map = parse_input(input);

    fn flood_fill(
        x: usize,
        y: usize,
        visited: &mut [Vec<bool>],
        map: &[Vec<u8>],
        peaks: &mut usize,
        previous: u8,
    ) {
        if map[x][y] != previous + 1 {
            return;
        }
        if visited[x][y] {
            return;
        }
        visited[x][y] = true;
        if map[x][y] == b'9' {
            *peaks += 1;
            return;
        }

        NEIGHBOURS.into_iter().for_each(|(dx, dy)| {
            if x.wrapping_add(dx) < map.len() && y.wrapping_add(dy) < map[0].len() {
                flood_fill(
                    x.wrapping_add(dx),
                    y.wrapping_add(dy),
                    visited,
                    map,
                    peaks,
                    map[x][y],
                );
            }
        });
    }

    let mut score = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == b'0' {
                let mut trail_score = 0;
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                flood_fill(x, y, &mut visited, &map, &mut trail_score, b'0' - 1);
                score += trail_score;
            }
        }
    }

    score
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);

    fn backtrack(x: usize, y: usize, map: &[Vec<u8>], trails: &mut usize, previous: u8) {
        if map[x][y] != previous + 1 {
            return;
        }
        if map[x][y] == b'9' {
            *trails += 1;
            return;
        }

        NEIGHBOURS.into_iter().for_each(|(dx, dy)| {
            let nx = x.wrapping_add(dx);
            let ny = y.wrapping_add(dy);

            if nx < map.len() && ny < map[0].len() {
                backtrack(nx, ny, map, trails, map[x][y]);
            }
        });
    }

    let mut rating = 0;

    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == b'0' {
                let mut trailhead_rating = 0;
                backtrack(x, y, &map, &mut trailhead_rating, b'0' - 1);
                rating += trailhead_rating;
            }
        }
    }

    rating
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}", part2(test));
    let test2 = include_str!("../test2");
    println!("Part 2 (test2): {}", part2(test2));
    println!("Part 2: {}", part2(input));
}

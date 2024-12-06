fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_guard_pos(map: &[Vec<char>]) -> (usize, usize) {
    let mut guard_pos: (usize, usize) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                guard_pos = (i, j);
            }
        }
    }
    guard_pos
}

fn part1(input: &str) -> usize {
    let mut map = parse_input(input);
    const VISITED: char = 'x';

    let mut guard_pos: (usize, usize) = get_guard_pos(&map);
    let mut current_direction = 0usize;
    let directions = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let rows = map.len();
    let cols = map[0].len();
    loop {
        let (mut x, mut y) = guard_pos;
        map[x][y] = VISITED;

        x = x.wrapping_add(directions[current_direction].0);
        y = y.wrapping_add(directions[current_direction].1);
        if x >= rows || y >= cols {
            break;
        } else {
            guard_pos = (x, y);
        }

        let next_x = x.wrapping_add(directions[current_direction].0);
        let next_y = y.wrapping_add(directions[current_direction].1);

        if map
            .get(next_x)
            .and_then(|row| row.get(next_y))
            .map(|c| *c == '#')
            == Some(true)
        {
            current_direction = (current_direction + 1) % directions.len();
        }
    }

    map.iter()
        .map(|row| row.iter().filter(|&c| *c == VISITED).count())
        .sum::<usize>()
}

fn get_positions_in_path(map: &[Vec<char>]) -> Vec<(usize, usize, usize)> {
    let mut visited = Vec::new();
    let mut guard_pos: (usize, usize) = get_guard_pos(map);
    let mut current_direction = 0usize;
    let directions = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let rows = map.len();
    let cols = map[0].len();

    loop {
        let (mut x, mut y) = guard_pos;
        visited.push((x, y, current_direction));

        x = x.wrapping_add(directions[current_direction].0);
        y = y.wrapping_add(directions[current_direction].1);
        if x >= rows || y >= cols {
            return visited;
        } else {
            guard_pos = (x, y);
        }

        let next_x = x.wrapping_add(directions[current_direction].0);
        let next_y = y.wrapping_add(directions[current_direction].1);

        if map
            .get(next_x)
            .and_then(|row| row.get(next_y))
            .map(|c| *c == '#')
            == Some(true)
        {
            current_direction = (current_direction + 1) % directions.len();
        }
    }
}

fn will_get_stuck(map: &[Vec<char>], guard_direction: usize) -> bool {
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    let mut guard_pos: (usize, usize) = get_guard_pos(map);
    let mut current_direction = guard_direction;
    let directions = [(usize::MAX, 0), (0, 1), (1, 0), (0, usize::MAX)];
    let rows = map.len();
    let cols = map[0].len();

    loop {
        let (mut x, mut y) = guard_pos;
        if visited.contains(&(x, y, current_direction)) {
            return true;
        } else {
            visited.insert((x, y, current_direction));
        }

        x = x.wrapping_add(directions[current_direction].0);
        y = y.wrapping_add(directions[current_direction].1);
        if x >= rows || y >= cols {
            return false;
        } else {
            guard_pos = (x, y);
        }

        let mut next_x = x.wrapping_add(directions[current_direction].0);
        let mut next_y = y.wrapping_add(directions[current_direction].1);

        while map
            .get(next_x)
            .and_then(|row| row.get(next_y))
            .map(|c| *c == '#')
            == Some(true)
        {
            current_direction = (current_direction + 1) % directions.len();
            next_x = x.wrapping_add(directions[current_direction].0);
            next_y = y.wrapping_add(directions[current_direction].1);
        }
    }
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    let mut answer = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            let mut copy = map.clone();
            copy[row][col] = '#';
            if will_get_stuck(&copy, 0) {
                answer += 1;
            }
        }
    }

    answer
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}

// https://adventofcode.com/2019/day/10

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn count_visible_asteroids(
    source: (usize, usize), // row, col
    grid: &[Vec<u8>],
) -> (usize, Vec<Vec<bool>>) {
    let mut visible_grid = vec![vec![true; grid[0].len()]; grid.len()];

    let (source_row, source_col) = source;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if (row != source_row || col != source_col) && (grid[row][col] == b'#') {
                let row_diff = row as i32 - source_row as i32;
                let col_diff = col as i32 - source_col as i32;

                let gcd = gcd(row_diff.unsigned_abs(), col_diff.unsigned_abs()) as i32;
                let row_delta = row_diff / gcd;
                let col_delta = col_diff / gcd;

                let mut row_it = row as i32 + row_delta;
                let mut col_it = col as i32 + col_delta;
                while row_it >= 0
                    && col_it >= 0
                    && row_it < grid.len() as i32
                    && col_it < grid[0].len() as i32
                {
                    visible_grid[row_it as usize][col_it as usize] = false;
                    row_it += row_delta;
                    col_it += col_delta;
                }
            }
        }
    }

    let mut visible_asteroids = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == b'#' && visible_grid[row][col] {
                visible_asteroids += 1;
            }
        }
    }
    (visible_asteroids - 1, visible_grid)
}

fn part1(input: &str) -> (usize, (usize, usize)) {
    let grid = parse_input(input);

    let mut asteroids = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == b'#' {
                asteroids.push((row, col));
            }
        }
    }

    asteroids.sort_unstable_by_key(|&asteroid| count_visible_asteroids(asteroid, &grid).0);

    (
        count_visible_asteroids(asteroids[asteroids.len() - 1], &grid).0,
        asteroids[asteroids.len() - 1],
    )
}

fn part2(input: &str) -> (usize, (usize, usize)) {
    let (_, (station_row, station_col)) = part1(input);
    let grid = parse_input(input);
    let (_, mut visible_grid) = count_visible_asteroids((station_row, station_col), &grid);

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Angle(f64);
    impl Eq for Angle {}
    impl PartialOrd for Angle {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Angle {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.partial_cmp(&other.0).unwrap()
        }
    }

    let mut right_side_asteroids = vec![];
    let mut left_side_asteroids = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == b'#' && (row != station_row || col != station_col) {
                if col < station_col {
                    left_side_asteroids.push((row, col));
                } else {
                    right_side_asteroids.push((row, col));
                }
            }
        }
    }
    fn tangent(row_delta: i32, col_delta: i32) -> Angle {
        Angle(row_delta as f64 / col_delta as f64)
    }
    // Clockwise so top to bottom
    right_side_asteroids.sort_by_key(|&(row, col)| {
        tangent(
            row as i32 - station_row as i32,
            col as i32 - station_col as i32,
        )
    });
    // Clockwise so bottom to top
    left_side_asteroids.sort_by_key(|&(row, col)| {
        tangent(
            row as i32 - station_row as i32,
            col as i32 - station_col as i32,
        )
    });

    fn destroy_asteroid(
        station_row: usize,
        station_col: usize,
        row: usize,
        col: usize,
        visible_grid: &mut [Vec<bool>],
    ) {
        let row_diff = row as i32 - station_row as i32;
        let col_diff = col as i32 - station_col as i32;

        let gcd = gcd(row_diff.unsigned_abs(), col_diff.unsigned_abs()) as i32;
        let row_delta = row_diff / gcd;
        let col_delta = col_diff / gcd;

        let mut row_it = row as i32 + row_delta;
        let mut col_it = col as i32 + col_delta;
        while row_it >= 0
            && col_it >= 0
            && row_it < visible_grid.len() as i32
            && col_it < visible_grid[0].len() as i32
        {
            visible_grid[row_it as usize][col_it as usize] = true;
            row_it += row_delta;
            col_it += col_delta;
        }
    }

    let mut destroyed_asteroids_count = 0;
    let mut last_destroyed: (usize, usize) = (0, 0);
    let total_asteroids_count = left_side_asteroids.len() + right_side_asteroids.len();
    let mut destroyed_asteroids = vec![];
    loop {
        for &(row, col) in &destroyed_asteroids {
            // Unblocks the visibility behind the destroyed asteroids
            destroy_asteroid(station_row, station_col, row, col, &mut visible_grid);
        }
        destroyed_asteroids.clear();
        for asteroid_arr in [&right_side_asteroids, &left_side_asteroids] {
            for &(row, col) in asteroid_arr {
                if destroyed_asteroids_count == 200 {
                    break;
                }
                if !visible_grid[row][col] {
                    continue;
                }
                // Set the visibility to false to avoid destroying this asteroid again
                visible_grid[row][col] = false;
                destroyed_asteroids_count += 1;
                last_destroyed = (row, col);
                destroyed_asteroids.push((row, col));
            }
        }
        if destroyed_asteroids_count == 200 || destroyed_asteroids_count == total_asteroids_count {
            break;
        }
    }

    (100 * last_destroyed.1 + last_destroyed.0, last_destroyed)
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");

    // Test
    println!("Part 1 (test): {:?}", part1(test));
    println!("Part 2 (test): {:?}", part2(test));

    // Input
    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

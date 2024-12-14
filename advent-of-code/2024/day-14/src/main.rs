use regex::Regex;
use std::sync::LazyLock;

static ROBOT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap());

#[derive(Debug)]
struct Robot {
    row: isize,
    col: isize,
    vrow: isize,
    vcol: isize,
}
impl Robot {
    fn new(row: isize, col: isize, vrow: isize, vcol: isize) -> Self {
        Self {
            row,
            col,
            vrow,
            vcol,
        }
    }
}
fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            if let Some(caps) = ROBOT.captures(line) {
                let (_, [col, row, vcol, vrow]) = caps.extract();
                Robot::new(
                    row.parse::<isize>().unwrap(),
                    col.parse::<isize>().unwrap(),
                    vrow.parse::<isize>().unwrap(),
                    vcol.parse::<isize>().unwrap(),
                )
            } else {
                panic!("HELP! at {line}");
            }
        })
        .collect()
}

fn move_robot(robot: &mut Robot, rows: isize, cols: isize, times: isize) {
    let mut new_row = robot.row + times * robot.vrow;
    while new_row < 0 {
        new_row += rows;
    }
    while new_row >= rows {
        new_row -= rows;
    }
    robot.row = new_row;
    let mut new_col = robot.col + times * robot.vcol;
    while new_col < 0 {
        new_col += cols;
    }
    while new_col >= cols {
        new_col -= cols;
    }
    robot.col = new_col;
}

fn move_robots(robots: &mut [Robot], rows: isize, cols: isize, times: isize) {
    robots
        .iter_mut()
        .for_each(|robot| move_robot(robot, rows, cols, times));
}

fn print_robots(robots: &[Robot], rows: isize, cols: isize) {
    let rows = rows as usize;
    let cols = cols as usize;

    for y in 0..rows {
        for x in 0..cols {
            if robots
                .iter()
                .any(|robot| robot.col == x as isize && robot.row == y as isize)
            {
                print!("R");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn count_robots_in_quadrants(
    robots: &[Robot],
    rows: isize,
    cols: isize,
) -> (usize, usize, usize, usize) {
    robots.iter().fold((0, 0, 0, 0), |mut res, robot| {
        match (robot.row < rows / 2, robot.col < cols / 2) {
            (true, true) => res.0 += 1,
            (true, false) if robot.col > cols / 2 => res.1 += 1,
            (false, true) if robot.row > rows / 2 => res.2 += 1,
            (false, false) if robot.row > rows / 2 && robot.col > cols / 2 => res.3 += 1,
            _ => {}
        }
        res
    })
}

fn part1(input: &str, rows: isize, cols: isize) -> usize {
    let mut robots = parse_input(input);

    move_robots(&mut robots, rows, cols, 100);

    let counts = count_robots_in_quadrants(&robots, rows, cols);
    counts.0 * counts.1 * counts.2 * counts.3
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: isize, b: isize) -> isize {
    a * b / gcd(a, b)
}

fn count_period(robot: &Robot, rows: isize, cols: isize) -> isize {
    let vrow = if robot.vrow > 0 {
        robot.vrow
    } else {
        robot.vrow + rows
    };
    let row_period = vrow * rows;
    let vcol = if robot.vcol > 0 {
        robot.vcol
    } else {
        robot.vcol + cols
    };
    let col_period = vcol * cols;

    lcm(row_period, col_period)
}

fn robots_period(robots: &[Robot], rows: isize, cols: isize) -> isize {
    robots
        .iter()
        .fold(1, |acc, robot| lcm(acc, count_period(robot, rows, cols)))
}

fn is_sort_of_easter_egg(robots: &[Robot]) -> bool {
    let outside_line = robots
        .iter()
        .filter(|&robot| robot.row < 30 || robot.row > 63)
        .count();
    outside_line < 300
}

fn part2(input: &str, rows: isize, cols: isize) {
    let mut robots = parse_input(input);
    for elapsed in 1..7000 {
        move_robots(&mut robots, rows, cols, 1);
        if is_sort_of_easter_egg(&robots) {
            println!("Elapsed {}", elapsed);
            print_robots(&robots, rows, cols);
        }
    }
}

fn print_period(input: &str, rows: isize, cols: isize) {
    let robots = parse_input(input);
    println!("Period: {}", robots_period(&robots, rows, cols));
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    let test2 = include_str!("../test2");

    println!("Part 1 (test): {}, expected 12", part1(test, 7, 11));
    // println!("Part 1 (test): {}", part1(test2, 7, 11));
    println!("Part 1: {}", part1(input, 103, 101));

    println!("Part 2:");
    print_period(input, 103, 101);
    part2(input, 103, 101);
}

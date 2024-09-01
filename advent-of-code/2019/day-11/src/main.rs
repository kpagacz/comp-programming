// https://adventofcode.com/2019/day/11
mod intcode;
use intcode::{IntCode, IntResult, IntUnit};

const BLACK: IntUnit = 0;
const WHITE: IntUnit = 1;
const LEFT: IntUnit = 0;
const RIGHT: IntUnit = 1;

fn part1(input: &str) -> usize {
    let mut robot = IntCode::from(input);
    use std::collections::HashMap;
    let mut grid = HashMap::new();
    let mut position = (0, 0);
    let mut direction = (-1, 0);
    let mut current_color;
    loop {
        match (robot.run_interruptible(), robot.run_interruptible()) {
            (IntResult::End, _) => break,
            (IntResult::Ok(paint), IntResult::Ok(turn)) => {
                grid.insert(position, paint);
                match turn {
                    // Because the direction of the rotation is counterclockwise
                    // so right is -90 degrees and left is +90 degrees
                    RIGHT => direction = (direction.1, -direction.0),
                    LEFT => direction = (-direction.1, direction.0),
                    _ => unreachable!(),
                }
                position.0 += direction.0;
                position.1 += direction.1;
            }
            (IntResult::AwaitInput, _) => {
                current_color = *grid.get(&position).unwrap_or(&BLACK);
                robot.input.push_back(current_color);
            }
            _ => unreachable!(),
        }
    }

    grid.keys().len()
}

fn part2(input: &str) {
    let mut robot = IntCode::from(input);
    robot.input.push_back(WHITE);
    use std::collections::HashMap;
    let mut grid = HashMap::new();
    let mut position = (0, 0);
    let mut direction = (-1, 0);
    let mut current_color;
    loop {
        match (robot.run_interruptible(), robot.run_interruptible()) {
            (IntResult::End, _) => break,
            (IntResult::Ok(paint), IntResult::Ok(turn)) => {
                grid.insert(position, paint);
                match turn {
                    // Because the direction of the rotation is counterclockwise
                    // so right is -90 degrees and left is +90 degrees
                    RIGHT => direction = (direction.1, -direction.0),
                    LEFT => direction = (-direction.1, direction.0),
                    _ => unreachable!(),
                }
                position.0 += direction.0;
                position.1 += direction.1;
            }
            (IntResult::AwaitInput, _) => {
                current_color = *grid.get(&position).unwrap_or(&BLACK);
                robot.input.push_back(current_color);
            }
            _ => unreachable!(),
        }
    }

    let min_x = *grid.keys().map(|(row, _)| row).min().unwrap();
    let max_x = *grid.keys().map(|(row, _)| row).max().unwrap();
    let min_y = *grid.keys().map(|(_, col)| col).min().unwrap();
    let max_y = *grid.keys().map(|(_, col)| col).max().unwrap();

    for row in min_x..=max_x {
        for col in min_y..=max_y {
            match *grid.get(&(row, col)).unwrap_or(&BLACK) {
                WHITE => print!("#"),
                BLACK => print!("."),
                _ => unreachable!(),
            }
        }
        println!()
    }
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1 (input): {}", part1(input));
    part2(input);
}

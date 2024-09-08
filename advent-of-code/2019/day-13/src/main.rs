use intcode::{IntCode, IntUnit};

mod intcode;

const BLOCK: IntUnit = 2;
const BALL: IntUnit = 4;
const PADDLE: IntUnit = 3;
fn part1(input: &str) -> usize {
    let mut vm = IntCode::from(input);
    let mut block_tiles = 0;

    loop {
        let slice: Vec<_> = vm.by_ref().take(3).collect();
        if slice.is_empty() {
            break;
        }
        if slice[2] == BLOCK {
            block_tiles += 1;
        }
    }

    block_tiles
}

fn part2(input: &str) -> usize {
    let mut vm = IntCode::from(input);
    vm.memory.insert(0, 2);

    let mut current_score = 0;
    let mut paddle_x = 0;

    loop {
        match vm.run_interruptible() {
            intcode::IntResult::End => break,
            intcode::IntResult::AwaitInput => vm.input.push_back(0),
            intcode::IntResult::Ok(first) => {
                match (vm.run_interruptible(), vm.run_interruptible()) {
                    (intcode::IntResult::Ok(second), intcode::IntResult::Ok(third)) => {
                        match (first, second, third) {
                            (-1, 0, score) => current_score = score,
                            (ball_x, _, BALL) => match ball_x.cmp(&paddle_x) {
                                std::cmp::Ordering::Less => vm.input.push_back(-1),
                                std::cmp::Ordering::Equal => vm.input.push_back(0),
                                std::cmp::Ordering::Greater => vm.input.push_back(1),
                            },
                            (x, _, PADDLE) => paddle_x = x,
                            _ => {}
                        }
                    }
                    _ => unreachable!("Didn't get a triple!!!"),
                }
            }
        }
    }

    current_score as _
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1 (input): {}", part1(input));
    println!("Part 2 (input): {}", part2(input));
}

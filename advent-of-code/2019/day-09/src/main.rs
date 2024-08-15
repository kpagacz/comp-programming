// https://adventofcode.com/2019/day/9
mod intcode;
use intcode::{IntCode, IntUnit};

fn run(input: &str, input_value: IntUnit) -> IntUnit {
    let mut intcode = IntCode::from(input);

    let res = intcode.run(vec![input_value]);

    if res.len() != 1 {
        println!("Got errors when running checks: {res:?}");
        -1
    } else {
        res[0]
    }
}

fn main() {
    let input = include_str!("../input");

    println!("Part 1: {}", run(input, 1));
    println!("Part 2: {}", run(input, 2));
}

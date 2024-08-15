// https://adventofcode.com/2019/day/5
mod intcode;
use intcode::IntCode;

fn main() {
    let input = include_str!("../input");
    let mut intcode = IntCode::from(input);
    let part1_output = intcode.run(vec![1]);
    println!("{:?}", part1_output);

    intcode.reset();
    let part2_output = intcode.run(vec![5]);
    println!("{:?}", part2_output);
}

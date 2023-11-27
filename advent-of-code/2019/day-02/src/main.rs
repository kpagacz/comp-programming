use intcode::IntCode;

mod intcode;

fn part1(input: &str) -> i32 {
    let mut program = IntCode::from(input);
    program.memory[1] = 12;
    program.memory[2] = 2;
    program.run();

    program.memory[0]
}

fn part2(input: &str) -> i32 {
    for i in 0..=99 {
        for j in 0..=99 {
            let mut program = IntCode::from(input);
            program.memory[1] = i;
            program.memory[2] = j;
            program.run();

            if program.memory[0] == 19690720 {
                return 100 * i + j;
            }
        }
    }
    unreachable!()
}

fn main() {
    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

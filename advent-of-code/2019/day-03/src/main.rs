struct Instruction(u8, u32);
fn parse_input(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let mut lines = input.lines();
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();

    fn parse_line(line: &str) -> Vec<Instruction> {
        line.split(',')
            .map(|instruction| {
                let bytes = instruction.as_bytes();
                let c = bytes[0];
                let steps = instruction[1..].parse::<u32>().unwrap();
                Instruction(c, steps)
            })
            .collect()
    }

    (parse_line(first_line), parse_line(second_line))
}

fn part1(input: &str) -> i32 {
    use std::collections::HashSet;

    let mut points = HashSet::new();
    let (first, second) = parse_input(input);

    let mut current_location = (0i32, 0i32);
    for instruction in first {
        let Instruction(direction, steps) = instruction;
        let delta = match direction {
            b'R' => (0, 1),
            b'L' => (0, -1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            current_location.0 += delta.0;
            current_location.1 += delta.1;
            points.insert(current_location);
        }
    }

    current_location = (0, 0);
    let mut min_distance = i32::MAX;
    for instruction in second {
        let Instruction(direction, steps) = instruction;
        let delta = match direction {
            b'R' => (0, 1),
            b'L' => (0, -1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            current_location.0 += delta.0;
            current_location.1 += delta.1;

            if points.contains(&current_location) {
                min_distance =
                    min_distance.min(current_location.0.abs() + current_location.1.abs());
            }
        }
    }

    min_distance
}

fn part2(input: &str) -> u32 {
    use std::collections::HashMap;

    let mut points = HashMap::new();
    let (first, second) = parse_input(input);

    let mut current_location = (0i32, 0i32);
    let mut total_steps = 0;
    for instruction in first {
        let Instruction(direction, steps) = instruction;
        let delta = match direction {
            b'R' => (0, 1),
            b'L' => (0, -1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            total_steps += 1;
            current_location.0 += delta.0;
            current_location.1 += delta.1;
            points.entry(current_location).or_insert(total_steps);
        }
    }

    current_location = (0, 0);
    total_steps = 0;
    let mut min_steps = u32::MAX;
    for instruction in second {
        let Instruction(direction, steps) = instruction;
        let delta = match direction {
            b'R' => (0, 1),
            b'L' => (0, -1),
            b'U' => (-1, 0),
            b'D' => (1, 0),
            _ => unreachable!(),
        };

        for _ in 0..steps {
            total_steps += 1;
            current_location.0 += delta.0;
            current_location.1 += delta.1;

            if points.contains_key(&current_location) {
                min_steps = min_steps.min(total_steps + *points.get(&current_location).unwrap());
            }
        }
    }
    min_steps
}

fn main() {
    let test = include_str!("../test");
    println!("Part 1 (test): {}", part1(test));
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}

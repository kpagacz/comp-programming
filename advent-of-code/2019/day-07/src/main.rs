mod intcode;
use intcode::IntCode;

fn part1(input: &str) -> i64 {
    let mut intcode = IntCode::from(input);
    let mut max_thruster_input = i64::MIN;

    fn run_acs(intcode: &mut IntCode, phases: &[i64]) -> i64 {
        let mut input = 0;
        for phase in phases {
            intcode.reset();
            input = intcode.run(vec![*phase, input])[0];
        }
        input
    }

    fn heaps(k: usize, arr: &mut [i64], max_thruster_input: &mut i64, intcode: &mut IntCode) {
        if k == 1 {
            *max_thruster_input = (*max_thruster_input).max(run_acs(intcode, arr));
        } else {
            heaps(k - 1, arr, max_thruster_input, intcode);
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    arr.swap(i, k - 1);
                } else {
                    arr.swap(0, k - 1);
                }

                heaps(k - 1, arr, max_thruster_input, intcode);
            }
        }
    }

    let mut phases = (0..=4).collect::<Vec<_>>();
    heaps(
        phases.len(),
        &mut phases,
        &mut max_thruster_input,
        &mut intcode,
    );

    max_thruster_input
}

fn part2(input: &str) -> i64 {
    let mut max_thruster_input = i64::MIN;

    fn run_acs(phases: &[i64], input: &str) -> i64 {
        let mut intcodes = [
            IntCode::from(input),
            IntCode::from(input),
            IntCode::from(input),
            IntCode::from(input),
            IntCode::from(input),
        ];
        let mut input = 0;

        for (pos, &phase) in phases.iter().enumerate() {
            intcodes[pos].input.extend([phase, input]);
            input = intcodes[pos].run_interruptible().unwrap();
        }

        loop {
            for intcode in intcodes.iter_mut() {
                intcode.input.push_back(input);
                if let Some(value) = intcode.run_interruptible() {
                    input = value;
                }
            }

            if intcodes[4].is_terminated() {
                break;
            }
        }

        intcodes[4].last_output.unwrap()
    }

    fn heaps(k: usize, arr: &mut [i64], max_thruster_input: &mut i64, input: &str) {
        if k == 1 {
            *max_thruster_input = (*max_thruster_input).max(run_acs(arr, input));
        } else {
            heaps(k - 1, arr, max_thruster_input, input);
            for i in 0..k - 1 {
                if k % 2 == 0 {
                    arr.swap(i, k - 1);
                } else {
                    arr.swap(0, k - 1);
                }

                heaps(k - 1, arr, max_thruster_input, input);
            }
        }
    }

    let mut phases = (5..=9).collect::<Vec<_>>();
    heaps(phases.len(), &mut phases, &mut max_thruster_input, input);

    max_thruster_input
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    let test3 = include_str!("../test3");

    println!("test: {test}");
    println!("Part 1 (test): {}", part1(test));
    println!("Part 1 (test2): {}", part1(test2));
    println!("Part 1 (test3): {}", part1(test3));
    println!("Part 1: {}", part1(input));

    let test4 = include_str!("../test4");
    let test5 = include_str!("../test5");
    println!("Part 2 (test4): {}", part2(test4));
    println!("Part 2 (test5): {}", part2(test5));
    println!("Part 2: {}", part2(input));
}

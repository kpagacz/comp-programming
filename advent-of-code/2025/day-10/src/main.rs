type Machine = (u32, Vec<Vec<u32>>, Vec<i32>);
fn parse_input(input: &str) -> Vec<Machine> {
    use regex::Regex;
    let re = Regex::new(r"\[(?<bulbs>[\.#]+)\] (?<buttons>.*) \{(?<joltage>.*)\}").unwrap();

    input
        .lines()
        .map(|line| {
            let mut bulbs = 0u32;
            let mut buttons = vec![];
            let mut joltage = vec![];

            let Some(captures) = re.captures(line) else {
                panic!("Cannot capture: {line}");
            };
            captures["bulbs"].chars().enumerate().for_each(|(pos, c)| {
                if c == '#' {
                    bulbs |= 1 << pos;
                }
            });

            captures["buttons"].split_whitespace().for_each(|button| {
                let button_nums = button
                    .strip_prefix('(')
                    .map(|button| button.strip_suffix(')').unwrap())
                    .unwrap();
                let connected_bulbs = button_nums
                    .split(",")
                    .map(|num| num.parse::<u32>().unwrap())
                    .collect();
                buttons.push(connected_bulbs);
            });

            captures["joltage"]
                .split(",")
                .map(|num| num.parse::<i32>().unwrap())
                .for_each(|num| joltage.push(num));

            (bulbs, buttons, joltage)
        })
        .collect()
}

fn make_moves(state: u32, moves: &[Vec<u32>]) -> Vec<u32> {
    moves
        .iter()
        .map(|offsets| {
            let mut state = state;
            for offset in offsets {
                state ^= 1 << offset;
            }
            state
        })
        .collect()
}

fn solve_machine(machine: &Machine) -> usize {
    type State = (usize, u32); // moves, turned on bulbs (binary) - bulbs are reversed
    // so [..#] is 0b100
    let (target, moves, _) = machine;

    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    use std::collections::HashSet;

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    pq.push(Reverse((0usize, 0u32)));

    while let Some(Reverse((moves_count, state))) = pq.pop() {
        if visited.contains(&state) {
            continue;
        } else {
            visited.insert(state);
        }
        if state == *target {
            return moves_count;
        }

        make_moves(state, moves)
            .into_iter()
            .filter(|new_state| !visited.contains(new_state))
            .for_each(|new_state| pq.push(Reverse((moves_count + 1, new_state))));
    }

    unreachable!("There is always a way xd")
}

fn pretty_print_machines(machines: &[Machine]) {
    fn pretty_print_machine(machine: &Machine) {
        println!(
            "Machine. Bulb state: {:b}. Buttons: {:?}. Joltage: {:?}",
            machine.0, machine.1, machine.2
        );
    }
    machines.iter().for_each(pretty_print_machine);
}

fn part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines
        .into_iter()
        .map(|machine| solve_machine(&machine))
        .sum::<usize>()
}

fn make_moves_for_joltage(joltage: Vec<i32>, moves: &[Vec<u32>]) -> Vec<Vec<i32>> {
    moves
        .iter()
        .filter_map(|button| {
            let mut new_joltage = joltage.clone();
            let mut all_negative = true;
            for &bulb in button {
                new_joltage[bulb as usize] -= 1;
                if new_joltage[bulb as usize] >= 0 {
                    all_negative = false;
                }
            }
            if all_negative {
                None
            } else {
                Some(new_joltage)
            }
        })
        .collect()
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");
    println!("Part 1 (test): {} expected 7", part1(test));
    println!("Part 1: {}", part1(input));
}

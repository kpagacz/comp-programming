fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

use std::collections::HashMap;
use std::sync::LazyLock;
static BUTTON_TO_COORDS: LazyLock<HashMap<char, (i32, i32)>> = LazyLock::new(|| {
    let mut coords = HashMap::new();
    coords.insert('7', (0, 0));
    coords.insert('8', (0, 1));
    coords.insert('9', (0, 2));
    coords.insert('4', (1, 0));
    coords.insert('5', (1, 1));
    coords.insert('6', (1, 2));
    coords.insert('1', (2, 0));
    coords.insert('2', (2, 1));
    coords.insert('3', (2, 2));
    coords.insert('0', (3, 1));
    coords.insert('A', (3, 2));
    coords.insert('^', (0, 1));
    coords.insert('B', (0, 2));
    coords.insert('<', (1, 0));
    coords.insert('v', (1, 1));
    coords.insert('>', (1, 2));
    coords
});

const DOWN: char = 'v';
const UP: char = '^';
const LEFT: char = '<';
const RIGHT: char = '>';
const ACTIVATE: char = 'A';
const ROBOT_ACTIVATE: char = 'B';
#[allow(clippy::same_item_push)]
fn on_numeric_keypad(from: char, to: char) -> Vec<char> {
    let from_c = *BUTTON_TO_COORDS.get(&from).unwrap();
    let to_c = *BUTTON_TO_COORDS.get(&to).unwrap();

    let mut path = vec![];

    if from_c.1 == 0 && to_c.0 == 3 {
        for _ in from_c.1..to_c.1 {
            path.push(RIGHT);
        }
        for _ in from_c.0..to_c.0 {
            path.push(DOWN);
        }
        for _ in to_c.0..from_c.0 {
            path.push(UP);
        }
        for _ in to_c.1..from_c.1 {
            path.push(LEFT);
        }
    } else if from_c.0 != 3 || to_c.1 > 0 {
        for _ in to_c.1..from_c.1 {
            path.push(LEFT);
        }
        for _ in from_c.0..to_c.0 {
            path.push(DOWN);
        }
        for _ in from_c.1..to_c.1 {
            path.push(RIGHT);
        }
        for _ in to_c.0..from_c.0 {
            path.push(UP);
        }
    } else {
        for _ in from_c.0..to_c.0 {
            path.push(DOWN);
        }
        for _ in from_c.1..to_c.1 {
            path.push(RIGHT);
        }
        for _ in to_c.0..from_c.0 {
            path.push(UP);
        }
        for _ in to_c.1..from_c.1 {
            path.push(LEFT);
        }
    }
    path.push(ROBOT_ACTIVATE);

    path
}

#[allow(clippy::same_item_push)]
fn on_numeric_keypad_for_dp(from: char, to: char) -> Vec<Vec<char>> {
    let from_c = *BUTTON_TO_COORDS.get(&from).unwrap();
    let to_c = *BUTTON_TO_COORDS.get(&to).unwrap();

    let mut paths = vec![];

    if from_c.0 == 3 && to_c.1 == 0 {
        let mut path = vec![];
        for _ in to_c.0..from_c.0 {
            path.push(UP);
        }
        for _ in to_c.1..from_c.1 {
            path.push(LEFT);
        }
        paths.push(path);
    } else if from_c.1 == 0 && to_c.0 == 3 {
        let mut path = vec![];
        for _ in from_c.1..to_c.1 {
            path.push(RIGHT);
        }
        for _ in from_c.0..to_c.0 {
            path.push(DOWN);
        }
        paths.push(path);
    } else {
        let mut path1 = vec![];
        for _ in from_c.0..to_c.0 {
            path1.push(DOWN);
        }
        for _ in to_c.0..from_c.0 {
            path1.push(UP);
        }
        for _ in from_c.1..to_c.1 {
            path1.push(RIGHT);
        }
        for _ in to_c.1..from_c.1 {
            path1.push(LEFT);
        }
        paths.push(path1);
        let mut path2 = vec![];
        for _ in to_c.1..from_c.1 {
            path2.push(LEFT);
        }
        for _ in from_c.1..to_c.1 {
            path2.push(RIGHT);
        }
        for _ in from_c.0..to_c.0 {
            path2.push(DOWN);
        }
        for _ in to_c.0..from_c.0 {
            path2.push(UP);
        }
        paths.push(path2);
    }

    paths.iter_mut().for_each(|path| path.push(ROBOT_ACTIVATE));
    paths
}

fn code_to_paths(code: &[char]) -> Vec<Vec<char>> {
    assert!(code.len() == 5);
    let mut answer = vec![];
    let possible_transitions: Vec<_> = code
        .windows(2)
        .map(|window| on_numeric_keypad_for_dp(window[0], window[1]))
        .collect();

    fn rec(
        possible_transitions: &[Vec<Vec<char>>],
        answer: &mut Vec<Vec<char>>,
        current: Vec<char>,
    ) {
        if possible_transitions.is_empty() {
            answer.push(current);
            return;
        }

        for transition in possible_transitions.first().unwrap() {
            let mut copy = current.clone();
            copy.extend(transition);
            rec(&possible_transitions[1..], answer, copy);
        }
    }
    rec(&possible_transitions, &mut answer, vec![]);

    answer
}

#[allow(clippy::same_item_push)]
fn on_navigation_keypad(from: char, to: char) -> Vec<char> {
    let from_c = *BUTTON_TO_COORDS.get(&from).unwrap();
    let to_c = *BUTTON_TO_COORDS.get(&to).unwrap();

    let mut path = vec![];

    if from_c.0 != 0 || to_c.1 > 0 {
        for _ in to_c.1..from_c.1 {
            path.push(LEFT);
        }
        for _ in from_c.0..to_c.0 {
            path.push(DOWN);
        }
        for _ in from_c.1..to_c.1 {
            path.push(RIGHT);
        }
        for _ in to_c.0..from_c.0 {
            path.push(UP);
        }
    } else {
        for _ in from_c.0..to_c.0 {
            path.push(DOWN);
        }
        for _ in to_c.1..from_c.1 {
            path.push(LEFT);
        }
        for _ in from_c.1..to_c.1 {
            path.push(RIGHT);
        }
        for _ in to_c.0..from_c.0 {
            path.push(UP);
        }
    }
    path.push(ROBOT_ACTIVATE);

    path
}

#[allow(clippy::same_item_push)]
fn on_navigation_keypad_for_dp(from: char, to: char) -> Vec<Vec<char>> {
    let mut paths = vec![];

    match (from, to) {
        (ROBOT_ACTIVATE, UP) => paths.push(vec![LEFT]),
        (ROBOT_ACTIVATE, RIGHT) => paths.push(vec![DOWN]),
        (ROBOT_ACTIVATE, DOWN) => paths.extend([vec![LEFT, DOWN], vec![DOWN, LEFT]]),
        (ROBOT_ACTIVATE, LEFT) => paths.extend([vec![LEFT, DOWN, LEFT], vec![DOWN, LEFT, LEFT]]),
        (UP, ROBOT_ACTIVATE) => paths.push(vec![RIGHT]),
        (UP, DOWN) => paths.push(vec![DOWN]),
        (UP, RIGHT) => paths.extend([vec![RIGHT, DOWN], vec![DOWN, RIGHT]]),
        (UP, LEFT) => paths.push(vec![DOWN, LEFT]),
        (RIGHT, ROBOT_ACTIVATE) => paths.push(vec![UP]),
        (RIGHT, UP) => paths.extend([vec![UP, LEFT], vec![LEFT, UP]]),
        (RIGHT, DOWN) => paths.push(vec![LEFT]),
        (RIGHT, LEFT) => paths.push(vec![LEFT, LEFT]),
        (DOWN, ROBOT_ACTIVATE) => paths.extend([vec![UP, RIGHT], vec![RIGHT, UP]]),
        (DOWN, RIGHT) => paths.push(vec![RIGHT]),
        (DOWN, UP) => paths.push(vec![UP]),
        (DOWN, LEFT) => paths.push(vec![LEFT]),
        (LEFT, DOWN) => paths.push(vec![RIGHT]),
        (LEFT, RIGHT) => paths.push(vec![RIGHT, RIGHT]),
        (LEFT, UP) => paths.push(vec![RIGHT, UP]),
        (LEFT, ROBOT_ACTIVATE) => paths.extend([vec![RIGHT, UP, RIGHT], vec![RIGHT, RIGHT, UP]]),
        _ => paths.push(Vec::default()),
    }

    paths.iter_mut().for_each(|path| path.push(ROBOT_ACTIVATE));
    paths
}

fn number_from_code(code: &[char]) -> usize {
    String::from_iter(&code[..code.len() - 1])
        .parse::<usize>()
        .unwrap()
}

fn encode(insts: &[char], times: usize) -> Vec<char> {
    assert!(insts[0] == 'B' || insts[0] == 'A');
    let mut answer = insts.to_vec();
    for _ in 0..times {
        let mut new_insts = vec![ROBOT_ACTIVATE];
        answer
            .windows(2)
            .for_each(|window| new_insts.extend(on_navigation_keypad(window[0], window[1])));
        ensure_does_not_pass_empty(
            &new_insts,
            *BUTTON_TO_COORDS.get(&ROBOT_ACTIVATE).unwrap(),
            (0, 0),
        );
        answer = new_insts;
    }
    answer
}

const MOVES: [char; 5] = [LEFT, RIGHT, UP, DOWN, ROBOT_ACTIVATE];
fn encode_with_dp(insts: &[char], times: usize) -> usize {
    assert!(insts[0] == 'B' || insts[0] == 'A');
    let mut dp = vec![[[1usize; 256]; 256]; times + 1];
    for time in 1..dp.len() {
        for from in MOVES {
            for to in MOVES {
                let possible_new_moves = on_navigation_keypad_for_dp(from, to);
                dp[time][from as usize][to as usize] = usize::MAX;
                for mut new_moves in possible_new_moves {
                    new_moves.insert(0, ROBOT_ACTIVATE);
                    dp[time][from as usize][to as usize] = dp[time][from as usize][to as usize]
                        .min(
                            new_moves
                                .windows(2)
                                .map(|window| dp[time - 1][window[0] as usize][window[1] as usize])
                                .sum::<usize>(),
                        );
                }
            }
        }
    }

    insts
        .windows(2)
        .map(|window| dp[times][window[0] as usize][window[1] as usize])
        .sum::<usize>()
        + 1
}

fn ensure_does_not_pass_empty(insts: &[char], mut start: (i32, i32), empty: (i32, i32)) {
    for &inst in insts {
        match inst {
            UP => start.0 -= 1,
            DOWN => start.0 += 1,
            LEFT => start.1 -= 1,
            RIGHT => start.1 += 1,
            ACTIVATE | ROBOT_ACTIVATE => {}
            _ => unreachable!(),
        }

        if start == empty {
            panic!("LOL hit empty running these insts: {insts:?}");
        }
    }
}

fn part1(input: &str, times: usize) -> usize {
    let codes = parse_input(input);

    let mut answer = 0;
    for mut code in codes {
        let number = number_from_code(&code);
        code.insert(0, 'A');
        let mut navigation = vec!['B'];
        code.windows(2)
            .for_each(|window| navigation.extend(on_numeric_keypad(window[0], window[1])));
        ensure_does_not_pass_empty(
            &navigation,
            *BUTTON_TO_COORDS.get(&ACTIVATE).unwrap(),
            (3, 0),
        );

        let robot_navigation2 = encode(&navigation, times);

        answer += (robot_navigation2.len() - 1) * number;
    }
    answer
}

fn part2(input: &str, times: usize) -> usize {
    let codes = parse_input(input);

    let mut answer = 0;
    for mut code in codes {
        let number = number_from_code(&code);
        code.insert(0, 'A');

        let mut lowest = usize::MAX;
        let mut possible_paths = code_to_paths(&code);
        possible_paths.sort_unstable();
        possible_paths.dedup();
        for mut navigation in possible_paths {
            navigation.insert(0, 'B');
            lowest = lowest.min(encode_with_dp(&navigation, times));
        }
        answer += (lowest - 1) * number;
    }
    answer
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("For times = 2");
    println!("Part 1 (test): {}, expected 126384", part1(test, 2));
    println!("Part 1: {}", part1(input, 2));
    println!("Part 2: {}", part2(input, 2));

    println!("For times = 3");
    println!("Part 1: {}", part1(input, 3));
    println!("Part 2: {}", part2(input, 3));

    println!("For times = 4");
    println!("Part 1: {}", part1(input, 4));
    println!("Part 2: {}", part2(input, 4));

    println!("For times = 9");
    println!("Part 1: {}", part1(input, 9));
    println!("Part 2: {}", part2(input, 9));

    println!("For times = 25");
    println!("Part 2: {}", part2(input, 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_up2() {
        let ways = [
            [ROBOT_ACTIVATE, UP, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, UP, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        for way in ways {
            let res = encode(&way, 9);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_left_up5() {
        let ways = [
            [ROBOT_ACTIVATE, UP, UP, UP, LEFT, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, LEFT, UP, UP, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, UP, UP, UP, LEFT, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_left_up4() {
        let ways = [
            [ROBOT_ACTIVATE, UP, UP, LEFT, UP, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, UP, UP, UP, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, UP, LEFT, UP, UP, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, UP, UP, UP, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, UP, UP, LEFT, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, LEFT, UP, UP, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, UP, LEFT, UP, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        println!();
        for way in ways {
            let res = encode(&way, 4);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }
        panic!()
    }

    #[test]
    fn test_left_down4() {
        let ways = [
            [ROBOT_ACTIVATE, DOWN, DOWN, LEFT, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, LEFT, DOWN, DOWN, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, LEFT, DOWN, LEFT, DOWN, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }
        println!();

        for way in ways {
            let res = encode(&way, 4);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_left_down3() {
        let ways = [
            [ROBOT_ACTIVATE, LEFT, DOWN, LEFT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, DOWN, LEFT, LEFT, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        for way in ways {
            let res = encode(&way, 10);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_left_down2() {
        let ways = [
            [ROBOT_ACTIVATE, LEFT, DOWN, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, DOWN, LEFT, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }
        println!();
        for way in ways {
            let res = encode(&way, 9);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_right_up() {
        let ways = [
            [ROBOT_ACTIVATE, UP, UP, RIGHT, RIGHT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, RIGHT, RIGHT, UP, UP, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, RIGHT, UP, RIGHT, UP, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_right_down4() {
        let ways = [
            [ROBOT_ACTIVATE, DOWN, DOWN, RIGHT, RIGHT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, RIGHT, RIGHT, DOWN, DOWN, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, RIGHT, DOWN, RIGHT, DOWN, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }
        println!();
        for way in ways {
            let res = encode(&way, 9);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_right_down3() {
        let ways = [
            [ROBOT_ACTIVATE, DOWN, DOWN, RIGHT, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, RIGHT, DOWN, DOWN, ROBOT_ACTIVATE],
            [ROBOT_ACTIVATE, DOWN, RIGHT, DOWN, ROBOT_ACTIVATE],
        ];

        for way in ways {
            let res = encode(&way, 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        panic!()
    }

    #[test]
    fn test_examples() {
        let ways = ["A<A^A>^^AvvvA", "A<A^A^>^AvvvA", "A<A^A^^>AvvvA"];
        for way in ways {
            let res = encode(&way.chars().collect::<Vec<_>>(), 2);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }

        for way in ways {
            let res = encode(&way.chars().collect::<Vec<_>>(), 9);
            println!("Insts: {:?} Length of res: {}", way, res.len());
        }
    }
}

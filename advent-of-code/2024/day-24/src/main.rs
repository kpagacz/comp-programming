use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Logic {
    Wire(usize),
    Gate(String, String, LogicType),
}

#[derive(Debug, Clone, Copy)]
enum LogicType {
    And,
    Or,
    Xor,
}

impl Logic {
    fn eval(&self, logics: &HashMap<String, Self>) -> usize {
        match &self {
            Logic::Wire(val) => *val,
            Logic::Gate(first, second, logic_type) => match *logic_type {
                LogicType::And => {
                    logics.get(first).unwrap().eval(logics)
                        & logics.get(second).unwrap().eval(logics)
                }
                LogicType::Or => {
                    logics.get(first).unwrap().eval(logics)
                        | logics.get(second).unwrap().eval(logics)
                }
                LogicType::Xor => {
                    logics.get(first).unwrap().eval(logics)
                        ^ logics.get(second).unwrap().eval(logics)
                }
            },
        }
    }
}

fn eval_with_cache(
    logic: &str,
    logics: &HashMap<String, Logic>,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(val) = cache.get(logic) {
        return *val;
    }
    let ans = match logics.get(logic).unwrap() {
        Logic::Wire(val) => *val,
        Logic::Gate(first, second, logic_type) => match *logic_type {
            LogicType::And => {
                logics.get(first).unwrap().eval(logics) & logics.get(second).unwrap().eval(logics)
            }
            LogicType::Or => {
                logics.get(first).unwrap().eval(logics) | logics.get(second).unwrap().eval(logics)
            }
            LogicType::Xor => {
                logics.get(first).unwrap().eval(logics) ^ logics.get(second).unwrap().eval(logics)
            }
        },
    };
    cache.insert(logic.to_owned(), ans);

    ans
}

fn parse_input(input: &str) -> HashMap<String, Logic> {
    let mut map = HashMap::new();
    let (first, second) = input.split_once("\n\n").unwrap();

    first.lines().for_each(|line| {
        let (name, val) = line.split_once(": ").unwrap();
        let name = name.to_owned();
        let val = val.parse::<usize>().unwrap();
        map.insert(name, Logic::Wire(val));
    });

    second.lines().for_each(|line| {
        let (first, name) = line.split_once(" -> ").unwrap();
        let mut it = first.split_whitespace();
        let (first, logic_type, second) = (
            it.next().unwrap().to_owned(),
            match it.next().unwrap() {
                "OR" => LogicType::Or,
                "AND" => LogicType::And,
                "XOR" => LogicType::Xor,
                _ => unreachable!(),
            },
            it.next().unwrap().to_owned(),
        );

        map.insert(name.to_owned(), Logic::Gate(first, second, logic_type));
    });

    map
}

fn part1(input: &str) -> usize {
    let logics = parse_input(input);

    let mut z_logics: Vec<_> = logics.keys().filter(|&key| key.starts_with('z')).collect();
    z_logics.sort_unstable();

    let bits: Vec<_> = z_logics
        .into_iter()
        .map(|gate| logics.get(gate).unwrap().eval(&logics))
        .collect();

    let mut num = 0usize;
    for (pos, bit) in bits.into_iter().enumerate() {
        num |= bit << pos;
    }
    num
}

fn num_from_bits(start_with: char, logics: &HashMap<String, Logic>) -> usize {
    let mut cache = HashMap::new();
    let mut z_logics: Vec<_> = logics
        .keys()
        .filter(|&key| key.starts_with(start_with))
        .collect();
    z_logics.sort_unstable();
    let bits: Vec<_> = z_logics
        .into_iter()
        .map(|gate| eval_with_cache(gate, logics, &mut cache))
        .collect();

    let mut num = 0usize;
    for (pos, bit) in bits.into_iter().enumerate() {
        num |= bit << pos;
    }
    num
}

fn wrong_wires(logics: &mut HashMap<String, Logic>) -> Vec<String> {
    let mut ans = vec![];

    // Set x to 0 y to 1
    for (key, value) in logics.iter_mut() {
        if key.starts_with('y') {
            *value = Logic::Wire(0);
        } else if key.starts_with('x') {
            *value = Logic::Wire(1);
        }
    }
    let z = num_from_bits('z', logics);
    let x = num_from_bits('x', logics);
    let y = num_from_bits('y', logics);
    let should_be = x + y;
    for i in 0..64 {
        if ((should_be >> i) ^ (z >> i)) % 2 != 0 {
            ans.push(format!("z{:02}", i));
        }
    }

    // Set x to 1 y to 1
    for (key, value) in logics.iter_mut() {
        if key.starts_with('y') {
            *value = Logic::Wire(1);
        } else if key.starts_with('x') {
            *value = Logic::Wire(1);
        }
    }
    let z = num_from_bits('z', logics);
    let x = num_from_bits('x', logics);
    let y = num_from_bits('y', logics);
    let should_be = x + y;
    for i in 0..64 {
        if ((should_be >> i) ^ (z >> i)) % 2 != 0 {
            ans.push(format!("z{:02}", i));
        }
    }

    ans.sort_unstable();
    ans.dedup();
    ans
}

fn involved_gates(wrong_wires: &[String], logics: &HashMap<String, Logic>) -> Vec<String> {
    fn rec(wrong_wire: &str, logics: &HashMap<String, Logic>) -> Vec<String> {
        match logics.get(wrong_wire).unwrap() {
            Logic::Wire(_) => vec![],
            Logic::Gate(first, second, _) => {
                let mut ans = vec![wrong_wire.to_string()];
                ans.extend(rec(first, logics));
                ans.extend(rec(second, logics));
                ans
            }
        }
    }
    let mut answer = vec![];

    for wire in wrong_wires {
        answer.extend(rec(wire, logics));
    }

    answer.sort();
    answer.dedup();
    answer
}

use std::collections::HashSet;
fn is_looped(logics: &HashMap<String, Logic>) -> bool {
    fn eval_with_loop_det(
        logic: &str,
        logics: &HashMap<String, Logic>,
        cache: &mut HashMap<String, usize>,
        seen: &mut HashSet<String>,
    ) -> Option<usize> {
        if let Some(val) = cache.get(logic) {
            return Some(*val);
        }
        let ans = match logics.get(logic).unwrap() {
            Logic::Wire(val) => *val,
            Logic::Gate(first, second, logic_type) => {
                if seen.contains(logic) {
                    return None;
                } else {
                    seen.insert(logic.to_string());
                }
                match *logic_type {
                    LogicType::And => {
                        eval_with_loop_det(first, logics, cache, seen)?
                            & eval_with_loop_det(second, logics, cache, seen)?
                    }
                    LogicType::Or => {
                        eval_with_loop_det(first, logics, cache, seen)?
                            | eval_with_loop_det(second, logics, cache, seen)?
                    }
                    LogicType::Xor => {
                        eval_with_loop_det(first, logics, cache, seen)?
                            ^ eval_with_loop_det(second, logics, cache, seen)?
                    }
                }
            }
        };
        cache.insert(logic.to_owned(), ans);

        Some(ans)
    }

    let mut z_logics: Vec<_> = logics.keys().filter(|&key| key.starts_with('z')).collect();
    z_logics.sort_unstable();
    let mut cache = HashMap::new();
    let bits: Vec<_> = z_logics
        .into_iter()
        .map(|gate| eval_with_loop_det(gate, logics, &mut cache, &mut HashSet::new()))
        .collect();
    bits.iter().any(|gate| gate.is_none())
}

fn is_swap_better(
    first: &str,
    second: &str,
    logics: &mut HashMap<String, Logic>,
    aggregate_fn: impl Fn(usize, usize) -> usize,
) -> (bool, Vec<String>) {
    let mut cache = HashMap::new();
    let (first_value, second_value) = (
        eval_with_cache(first, logics, &mut cache),
        eval_with_cache(second, logics, &mut cache),
    );
    if first_value == second_value {
        return (false, vec![]);
    }
    let z = num_from_bits('z', logics);
    let x = num_from_bits('x', logics);
    let y = num_from_bits('y', logics);
    let original_to_fix = wrong_wires(logics);

    let first_logic = logics.remove(first).unwrap();
    let second_logic = logics.remove(second).unwrap();
    logics.insert(second.to_string(), first_logic.clone());
    logics.insert(first.to_string(), second_logic.clone());
    if is_looped(logics) {
        logics.insert(first.to_string(), first_logic);
        logics.insert(second.to_string(), second_logic);
        return (false, vec![]);
    }

    let new_to_fix = wrong_wires(logics);

    logics.insert(first.to_string(), first_logic);
    logics.insert(second.to_string(), second_logic);
    (
        new_to_fix.len() <= original_to_fix.len()
            && new_to_fix.iter().all(|gate| original_to_fix.contains(gate)),
        new_to_fix,
    )
}

fn swap_gates(first: &str, second: &str, logics: &mut HashMap<String, Logic>) {
    let first_logic = logics.remove(first).unwrap();
    let second_logic = logics.remove(second).unwrap();
    logics.insert(second.to_string(), first_logic.clone());
    logics.insert(first.to_string(), second_logic.clone());
}

fn print_logics_for_graphviz(input: &str) {
    let mut logics = parse_input(input);
    swap_gates("wsv", "rjm", &mut logics);
    swap_gates("z31", "bgs", &mut logics);
    swap_gates("swt", "z07", &mut logics);
    swap_gates("z13", "pqc", &mut logics);

    println!("digraph G {{");

    for (key, logic) in logics {
        match logic {
            Logic::Wire(_) => {
                println!(
                    r#"  {key} [color="green", pos="{},0!", style="filled"]"#,
                    2 * &key[1..].parse::<i32>().unwrap()
                        + if key.starts_with('y') { 1 } else { 0 }
                );
            }
            Logic::Gate(first, second, logic_type) => {
                println!(
                    r#"  {key} [color="{}"{}, style="filled"]"#,
                    match logic_type {
                        LogicType::And => "yellow",
                        LogicType::Or => "blue",
                        LogicType::Xor => "purple",
                    },
                    if key.starts_with('z') {
                        format!(r#", pos="{},20!""#, 2 * &key[1..].parse::<i32>().unwrap())
                    } else {
                        String::default()
                    }
                );
                println!("  {first} -> {key}");
                println!("  {second} -> {key}");
            }
        }
    }

    println!("}}");
}

fn part2() -> String {
    let mut ans = vec!["wsv", "rjm", "z31", "bgs", "swt", "z07", "z13", "pqc"];
    ans.sort();
    ans.join(",")
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {}, expected 2024", part1(test));
    println!("Part 1: {}", part1(input));

    // let test2 = include_str!("../test2");
    // println!("Part 2 (test): {}", part2(test2, |x, y| x & y));
    println!("Part 2: {}", part2());
    // print_logics_for_graphviz(input);
}

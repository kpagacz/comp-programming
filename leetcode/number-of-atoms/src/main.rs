// https://leetcode.com/problems/number-of-atoms/description/
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut stack = vec![HashMap::new()];
        let formula = formula.as_bytes();

        let mut map = HashMap::new();
        let mut it = 0;

        while it < formula.len() {
            match formula[it] {
                b'A'..=b'Z' => {
                    let top_map = stack.last_mut().unwrap();
                    let mut molecule = vec![formula[it]];
                    while it + 1 < formula.len() && formula[it + 1].is_ascii_lowercase() {
                        it += 1;
                        molecule.push(formula[it]);
                    }
                    let molecule = String::from_utf8(molecule).unwrap();
                    let mut multiplier = 1;
                    if it + 1 < formula.len() && formula[it + 1].is_ascii_digit() {
                        multiplier = Self::parse_number(&mut it, formula);
                    }

                    *top_map.entry(molecule).or_insert(0) += multiplier;
                }
                b'(' => stack.push(HashMap::new()),
                b')' => {
                    let mut multiplier = 1;
                    if it + 1 < formula.len() && formula[it + 1].is_ascii_digit() {
                        multiplier = Self::parse_number(&mut it, formula);
                    }
                    let in_brackets = stack.pop().unwrap();
                    Self::add_stack_map_to_general_map(
                        in_brackets,
                        stack.last_mut().unwrap(),
                        multiplier,
                    );
                }
                _ => unreachable!(),
            }
            it += 1;
            dbg!(&stack);
            dbg!(&map);
        }
        while let Some(in_brackets) = stack.pop() {
            let multiplier = 1;
            Self::add_stack_map_to_general_map(in_brackets, &mut map, multiplier);
        }

        let mut molecules: Vec<_> = map
            .into_iter()
            .map(|(key, value)| {
                if value == 1 {
                    key
                } else {
                    format!("{key}{value}")
                }
            })
            .collect();
        molecules.sort_unstable();
        molecules.join("")
    }

    fn parse_number(it: &mut usize, formula: &[u8]) -> i32 {
        let mut number = 0;
        while *it + 1 < formula.len() && formula[*it + 1].is_ascii_digit() {
            *it += 1;
            let digit = formula[*it] - b'0';
            number = number * 10 + digit as i32;
        }
        number
    }

    fn add_stack_map_to_general_map(
        stack_map: HashMap<String, i32>,
        general_map: &mut HashMap<String, i32>,
        multiplier: i32,
    ) {
        for (key, value) in stack_map {
            *general_map.entry(key).or_insert(0) += value * multiplier;
        }
    }
}

fn main() {
    let test_cases = [
        "H2O".to_string(),
        "Mg(OH)2".to_string(),
        "K4(ON(SO3)2)2".to_string(),
    ];
    for formula in test_cases {
        println!("{}", Solution::count_of_atoms(formula));
    }
}

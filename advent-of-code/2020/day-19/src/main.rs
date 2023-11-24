use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1},
    combinator::{map, map_res, recognize},
    sequence::tuple,
    IResult,
};

struct Solution {
    cache: HashMap<String, Vec<Vec<String>>>,
}

impl Solution {
    fn parse_input(&self, path: &str) -> (HashMap<String, Vec<Vec<String>>>, Vec<String>) {
        let input = std::fs::read_to_string(path).unwrap();
        let parts = input.split("\n\n").collect::<Vec<_>>();
        let texts = parts[1].lines().map(str::to_owned).collect::<Vec<_>>();

        let productions = parts[0]
            .lines()
            .map(|production| {
                let parts = production.split(":").collect::<Vec<_>>();
                let right_side = parts[1]
                    .trim()
                    .split("|")
                    .map(str::trim)
                    .map(|rule| {
                        rule.split_whitespace()
                            .map(str::to_owned)
                            .map(|symbol| symbol.replace("\"", ""))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                (parts[0].to_owned(), right_side)
            })
            .collect();
        (productions, texts)
    }

    fn part1(&mut self, path: &str) -> i32 {
        let (mut productions, texts) = self.parse_input(path);
        println!("{:?}", &productions);

        let res = self.eval(&mut productions, "0");
        let mut res = res
            .iter()
            .map(|symbols| symbols.concat())
            .collect::<Vec<String>>();
        res.sort_unstable();
        // println!("{:?}", res);

        texts
            .iter()
            .map(|text| match res.binary_search(text) {
                Ok(_) => 1,
                Err(_) => 0,
            })
            .reduce(|accum, found| accum + found)
            .unwrap()
    }

    fn replace_production(
        &self,
        production: &Vec<Vec<String>>,
        what: &str,
        with: &Vec<Vec<String>>,
    ) -> Vec<Vec<String>> {
        let mut out = Vec::new();

        production
            .iter()
            .for_each(|symbols| out.extend_from_slice(&self.replace_one(symbols, what, with)));

        out
    }

    fn replace_one(
        &self,
        symbols: &Vec<String>,
        what: &str,
        with: &Vec<Vec<String>>,
    ) -> Vec<Vec<String>> {
        let mut out = Vec::new();
        out.push(Vec::new());

        let symbols = symbols.clone();
        for symbol in symbols {
            if symbol != what {
                for array in &mut out {
                    array.push(symbol.to_string());
                }
            } else {
                let mut new_out = Vec::new();
                with.iter().for_each(|possible_replacement| {
                    let mut out_copy = out.clone();
                    for array in &mut out_copy {
                        array.extend_from_slice(possible_replacement);
                    }
                    new_out.extend_from_slice(&out_copy);
                });
                out = new_out;
            }
        }

        out
    }

    fn eval<'a>(
        &'a mut self,
        productions: &'a mut HashMap<String, Vec<Vec<String>>>,
        current_production: &str,
    ) -> &Vec<Vec<String>> {
        // println!("Evaluating {}", current_production);
        if self.cache.contains_key(current_production) {
            // println!("Cache hit");
            return self.cache.get(current_production).unwrap();
        }
        if self.is_simple(
            &productions
                .get(current_production)
                .expect(format!("could not find {}", &current_production).as_str()),
        ) {
            // println!("Simple case");
            self.cache.insert(
                current_production.to_owned(),
                productions.get(current_production).unwrap().clone(),
            );
            return productions.get(current_production).unwrap();
        } else {
            let mut new_production = Vec::new();

            let old_production = productions.get(current_production).unwrap().clone();
            old_production.iter().for_each(|symbols| {
                let mut generated_productions = vec![symbols.clone()];

                symbols.iter().for_each(|symbol| {
                    if symbol != "a" && symbol != "b" {
                        let symbol_evaluation = self.eval(productions, symbol).clone();
                        generated_productions = self.replace_production(
                            &generated_productions,
                            symbol,
                            &symbol_evaluation,
                        );
                    }
                });

                new_production.extend_from_slice(&generated_productions);
            });

            productions.insert(current_production.to_owned(), new_production);
        }

        productions.get(current_production).unwrap()
    }

    fn is_simple(&self, production: &Vec<Vec<String>>) -> bool {
        production
            .iter()
            .all(|symbols| symbols.iter().all(|symbol| symbol == "a" || symbol == "b"))
    }
}

#[derive(Debug, Clone)]
enum Node {
    Text(String),
    Rules(Vec<(usize, usize)>),
    SingleRules(Vec<usize>),
    SingleRule(usize),
}

fn solution(input: &str) {
    let (rules, examples) = input.split_once("\n\n").unwrap();

    fn parse_node(input: &str) -> IResult<&str, (usize, Node)> {
        fn my_u32(input: &str) -> IResult<&str, usize> {
            map_res(recognize(digit1), str::parse)(input)
        }
        alt((
            map(
                tuple((
                    my_u32,
                    tag(": "),
                    my_u32,
                    tag(" "),
                    my_u32,
                    tag(" | "),
                    my_u32,
                    tag(" "),
                    my_u32,
                )),
                |(id, _, first, _, second, _, third, _, fourth)| {
                    (id, Node::Rules(vec![(first, second), (third, fourth)]))
                },
            ),
            map(
                tuple((my_u32, tag(": "), my_u32, tag(" | "), my_u32)),
                |(id, _, first, _, second)| (id, Node::SingleRules(vec![first, second])),
            ),
            map(
                tuple((my_u32, tag(": \""), alphanumeric1, tag("\""))),
                |(id, _, text, _)| (id, Node::Text(text.to_owned())),
            ),
            map(
                tuple((my_u32, tag(": "), my_u32, tag(" "), my_u32)),
                |(id, _, first, _, second)| (id, Node::Rules(vec![(first, second)])),
            ),
            map(tuple((my_u32, tag(": "), my_u32)), |(id, _, first)| {
                (id, Node::SingleRule(first))
            }),
        ))(input)
    }

    use std::collections::BTreeMap;
    type Rules = BTreeMap<usize, Node>;
    let mut rules_map = Rules::new();
    for line in rules.lines() {
        let node = parse_node(line).unwrap().1;
        rules_map.insert(node.0, node.1);
    }

    fn match_rule<'a>(input: &'a str, rule: usize, rules: &Rules) -> Vec<&'a str> {
        match rules.get(&rule).expect("Rule always exists") {
            Node::Text(pattern) => match input.strip_prefix(pattern) {
                Some(after) => vec![after],
                None => vec![], // did not match = no sufixes
            },
            Node::Rules(double_rules) => {
                let mut ans = vec![];
                for &(first, second) in double_rules {
                    let after_first = match_rule(input, first, rules);
                    if after_first.len() == 0 {
                        // because it did not match at all
                        // there is no point calculating the rest
                        continue;
                    }
                    let after_second: Vec<_> = after_first
                        .into_iter()
                        .map(|sufix| match_rule(sufix, second, rules))
                        .flatten()
                        .collect();
                    ans.extend(after_second.into_iter());
                }
                ans
            }
            Node::SingleRules(single_rules) => {
                let reses: Vec<_> = single_rules
                    .iter()
                    .filter_map(|&rule| {
                        let m = match_rule(input, rule, rules);
                        Some(m)
                    })
                    .flatten()
                    .collect();
                reses
            }
            Node::SingleRule(other_rule) => match_rule(input, *other_rule, rules),
        }
    }

    // println!("{:?}", rules_map);
    let rule_nums: Vec<usize> = rules_map.keys().map(|&key| key).collect();
    let mut answer = 0;
    for example in examples.lines() {
        if rule_nums
            .iter()
            .any(|rule| match_rule(example, *rule, &rules_map).contains(&""))
        {
            // println!("Got a hit on {example}");
            answer += 1;
        }
    }
    println!("Part 1: {answer}");

    // Part 2
    let mut rules = rules_map.clone();
    rules.remove(&8);
    rules.remove(&11);
    rules.remove(&0);
    fn match_42<'a>(input: &'a str, rules: &Rules) -> Vec<&'a str> {
        match_rule(input, 42, rules)
    }
    fn match_31<'a>(input: &'a str, rules: &Rules) -> Vec<&'a str> {
        match_rule(input, 31, rules)
    }
    // Rules 8, 11 and 0 describe the following scenarios.
    // N times rule 42 or K times rule 42 followed by L<=K rule 31
    // Let's cap it at a fixed number times and see what happens.
    const TIMES: i32 = 5;

    let mut answer = 0;
    for example in examples.lines() {
        let mut rest = vec![example];
        for times in 0..TIMES {
            rest = rest
                .into_iter()
                .flat_map(|sufix| match_42(sufix, &rules))
                .collect();

            let mut rest_after_31 = rest.clone();
            let mut hit = false;
            for _ in 0..=times {
                rest_after_31 = rest_after_31
                    .into_iter()
                    .flat_map(|sufix| match_31(sufix, &rules))
                    .collect();
                if rest_after_31.iter().any(|sufix| sufix.is_empty()) {
                    println!("Got a hit on rule 0: {example}");
                    answer += 1;
                    hit = true;
                }
            }
            if hit {
                break;
            }
        }
    }
    println!("Part 2: {answer}");
}

fn main() {
    // let mut s = Solution {
    //     cache: HashMap::new(),
    // };
    // println!("Part 1: {}", s.part1("input"));

    let test = include_str!("../test2");
    println!("TEST");
    solution(test);
    let input = include_str!("../input");
    println!("INPUT");
    solution(input);
}

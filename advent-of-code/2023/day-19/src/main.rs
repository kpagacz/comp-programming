use regex::Regex;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Effect {
    Accepted,
    Rejected,
    Delegate(String),
    Continue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Relation {
    Greater,
    Smaller,
}

#[derive(Debug, Clone)]
struct Rule {
    category: char,
    relation: Relation,
    level: usize,
    effect: Effect,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    effect: Effect,
}

type Part = BTreeMap<char, usize>;
fn parse_part(line: &str) -> Part {
    let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
    let Some(caps) = re.captures(line) else {
        panic!("Could not parse part: {line}");
    };

    BTreeMap::from([
        ('x', caps[1].parse::<usize>().unwrap()),
        ('m', caps[2].parse::<usize>().unwrap()),
        ('a', caps[3].parse::<usize>().unwrap()),
        ('s', caps[4].parse::<usize>().unwrap()),
    ])
}

fn parse_rule(rule: &str) -> Rule {
    let re = Regex::new(r"([xmas])([<>])([0-9]+):([AR]|[a-z]+)").unwrap();
    let Some(caps) = re.captures(rule) else {
        panic!("Could not parse rule: {rule}");
    };

    Rule {
        category: caps[1].as_bytes()[0] as char,
        relation: if &caps[2] == ">" {
            Relation::Greater
        } else {
            Relation::Smaller
        },
        level: caps[3].parse::<usize>().unwrap(),
        effect: match &caps[4] {
            "A" => Effect::Accepted,
            "R" => Effect::Rejected,
            other_workflow => Effect::Delegate(other_workflow.to_string()),
        },
    }
}

fn parse_workflow(line: &str) -> (String, Workflow) {
    let re =
        Regex::new(r"([a-z]+)\{((?:[xmas][<>][0-9]+:(?:[AR]|[a-z]+),)*)([AR]|[a-z]+)\}").unwrap();

    let Some(caps) = re.captures(line) else {
        panic!("Could not parse workflow: {line}");
    };

    let workflow_name = caps[1].to_string();
    let rules: Vec<_> = caps[2]
        .split(',')
        .filter(|el| !el.is_empty())
        .map(parse_rule)
        .collect();
    let final_effect = match &caps[caps.len() - 1] {
        "A" => Effect::Accepted,
        "R" => Effect::Rejected,
        some => Effect::Delegate(some.to_string()),
    };

    (
        workflow_name,
        Workflow {
            rules,
            effect: final_effect,
        },
    )
}

fn parse_input(input: &str) -> (BTreeMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();
    let workflows: BTreeMap<String, Workflow> = workflows.lines().map(parse_workflow).collect();
    let parts: Vec<_> = parts.lines().map(parse_part).collect();

    (workflows, parts)
}

fn eval_rule(rule: &Rule, part: &Part) -> Effect {
    match rule.relation {
        Relation::Smaller => {
            if part.get(&rule.category).unwrap() < &rule.level {
                return rule.effect.clone();
            }
        }
        Relation::Greater => {
            if part.get(&rule.category).unwrap() > &rule.level {
                return rule.effect.clone();
            }
        }
    }

    Effect::Continue
}

fn eval_workflow(workflow: &str, part: &Part, workflows: &BTreeMap<String, Workflow>) -> Effect {
    let wflow = workflows.get(workflow).unwrap();
    let mut effect = Effect::Continue;

    for rule in &wflow.rules {
        effect = eval_rule(rule, part);
        if effect != Effect::Continue {
            break;
        }
    }

    match effect {
        Effect::Continue => match &wflow.effect {
            Effect::Delegate(other_workflow) => eval_workflow(other_workflow, part, workflows),
            verdict => verdict.clone(),
        },
        Effect::Delegate(other_workflow) => eval_workflow(&other_workflow, part, workflows),
        other_effect => other_effect.clone(),
    }
}

fn eval_part(part: &Part, workflows: &BTreeMap<String, Workflow>) -> Effect {
    eval_workflow("in", part, workflows)
}

fn score(part: &Part) -> usize {
    ['x', 'm', 'a', 's']
        .into_iter()
        .map(|category| part.get(&category).unwrap())
        .sum()
}

fn part1(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    parts
        .iter()
        .filter(|&part| eval_part(part, &workflows) == Effect::Accepted)
        .map(score)
        .sum()
}

fn split_on_rule(
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
    rule: &Rule,
) -> Vec<(
    (
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    ),
    Effect,
)> {
    fn apply_rule(interval: (usize, usize), rule: &Rule) -> Vec<((usize, usize), Effect)> {
        match rule.relation {
            Relation::Greater => {
                if interval.0 > rule.level {
                    vec![(interval, rule.effect.clone())]
                } else if interval.1 > rule.level - 2 {
                    vec![
                        ((interval.0, rule.level + 1), Effect::Continue),
                        ((rule.level + 1, interval.1), rule.effect.clone()),
                    ]
                } else {
                    vec![(interval, Effect::Continue)]
                }
            }
            Relation::Smaller => {
                if interval.1 <= rule.level {
                    vec![(interval, rule.effect.clone())]
                } else if interval.0 < rule.level {
                    vec![
                        ((interval.0, rule.level), rule.effect.clone()),
                        ((rule.level, interval.1), Effect::Continue),
                    ]
                } else {
                    vec![(interval, Effect::Continue)]
                }
            }
        }
    }

    match rule.category {
        'x' => apply_rule(x, rule)
            .into_iter()
            .map(|(new_interval, effect)| ((new_interval, m, a, s), effect))
            .collect(),
        'm' => apply_rule(m, rule)
            .into_iter()
            .map(|(new_interval, effect)| ((x, new_interval, a, s), effect))
            .collect(),

        'a' => apply_rule(a, rule)
            .into_iter()
            .map(|(new_interval, effect)| ((x, m, new_interval, s), effect))
            .collect(),

        's' => apply_rule(s, rule)
            .into_iter()
            .map(|(new_interval, effect)| ((x, m, a, new_interval), effect))
            .collect(),

        _ => unreachable!(),
    }
}

fn split_on_workflow(
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
    workflow: &Workflow,
) -> Vec<(
    (
        (usize, usize),
        (usize, usize),
        (usize, usize),
        (usize, usize),
    ),
    Effect,
)> {
    let mut non_continued_intervals = vec![];

    let mut continued_interval = (x, m, a, s);
    for rule in &workflow.rules {
        let mut new_intervals = split_on_rule(
            continued_interval.0,
            continued_interval.1,
            continued_interval.2,
            continued_interval.3,
            rule,
        );

        new_intervals.retain(|(intervals, effect)| {
            if effect == &Effect::Continue {
                true
            } else {
                non_continued_intervals.push((*intervals, effect.clone()));
                false
            }
        });

        if new_intervals.is_empty() {
            break;
        } else {
            if new_intervals.len() != 1 {
                panic!("Continued intervals is > 1! {new_intervals:?}");
            } else {
                continued_interval = new_intervals[0].0;
            }
        }
    }
    non_continued_intervals.push((continued_interval, workflow.effect.clone()));

    non_continued_intervals
}

const INITIAL_INTERVAL: (usize, usize) = (1, 4001);
fn part2(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    let mut stack = vec![(
        (
            INITIAL_INTERVAL,
            INITIAL_INTERVAL,
            INITIAL_INTERVAL,
            INITIAL_INTERVAL,
        ),
        Effect::Delegate("in".to_string()),
    )];

    let mut possibilities = 0;
    while let Some(top) = stack.pop() {
        let ((x, m, a, s), effect) = top;
        // println!("Found {x:?} {m:?} {a:?} {s:?} with {effect:?} on top");
        match effect {
            Effect::Accepted => {
                possibilities += (x.1 - x.0) * (m.1 - m.0) * (a.1 - a.0) * (s.1 - s.0);
            }
            Effect::Rejected => {}
            Effect::Delegate(workflow) => {
                let workflow = workflows.get(&workflow).unwrap();
                split_on_workflow(x, m, a, s, workflow)
                    .into_iter()
                    .for_each(|intervals_with_effect| stack.push(intervals_with_effect));
            }
            _ => unreachable!(),
        }
    }
    possibilities
}

fn main() {
    println!("TEST");
    let test = include_str!("../test");
    println!("Part 1: {}", part1(test));
    println!("Part 2: {}", part2(test));

    println!("INPUT");
    let input = include_str!("../input");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

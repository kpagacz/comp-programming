use num::rational::Rational64;
use std::collections::HashMap;

struct Expr {
    left: String,
    op: String,
    right: String,
}

enum Node {
    Expr(Expr),
    Literal(Rational64),
}

fn parse_line(line: &str) -> (String, Node) {
    let name = line.split(":").next().unwrap().to_string();
    let expression = line.split(":").last().unwrap().trim();
    if expression.contains(|c: char| c.is_whitespace()) {
        let mut expression = expression.split_whitespace();
        let left = expression.next().unwrap().to_string();
        let op = expression.next().unwrap().to_string();
        let right = expression.next().unwrap().to_string();
        (name, Node::Expr(Expr { left, op, right }))
    } else {
        let value = expression.parse::<Rational64>().unwrap();
        (name, Node::Literal(value))
    }
}

fn read_input(input: &str) -> HashMap<String, Node> {
    let mut nodes = HashMap::new();
    input.lines().for_each(|line| {
        let (name, node) = parse_line(line);
        nodes.insert(name, node);
    });
    nodes
}

fn eval(name: &String, nodes: &HashMap<String, Node>) -> Rational64 {
    match nodes.get(name).unwrap() {
        Node::Literal(literal) => *literal,
        Node::Expr(expr) => {
            let left = eval(&expr.left, nodes);
            let right = eval(&expr.right, nodes);
            match expr.op.as_str() {
                "+" => left + right,
                "-" => left - right,
                "*" => left * right,
                "/" => left / right,
                _ => panic!("Unknown operator"),
            }
        }
    }
}

fn part1(input: &str) -> Rational64 {
    let nodes = read_input(input);
    eval(&"root".to_string(), &nodes)
}

fn solve(name: &String, nodes: &HashMap<String, Node>, root: Rational64) -> Rational64 {
    if name == "humn" {
        return root;
    } else {
        match nodes.get(name).unwrap() {
            Node::Literal(literal) => *literal,
            Node::Expr(expr) => {
                let left = solve(&expr.left, nodes, root);
                let right = solve(&expr.right, nodes, root);
                match expr.op.as_str() {
                    "+" => left + right,
                    "-" => left - right,
                    "*" => left * right,
                    "/" => left / right,
                    _ => panic!("Unknown operator"),
                }
            }
        }
    }
}

fn part2(input: &str) -> Rational64 {
    let nodes = read_input(input);

    let root = nodes.get("root").unwrap();
    if let Node::Expr(expr) = root {
        let left = &expr.left;
        let right = &expr.right;
        // I knew from the input that the human node was in the left subtree
        let b = solve(left, &nodes, Rational64::from(0)) - eval(right, &nodes);
        let a = solve(left, &nodes, Rational64::from(1)) - solve(left, &nodes, Rational64::from(0));
        return (-b) / a;
    }
    return Rational64::from(-1);
}

fn main() {
    println!("Part 1: {}", part1(TEST));
    println!("Part 2: {}", part2(TEST));
}

const TEST: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";

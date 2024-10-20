// https://leetcode.com/problems/parsing-a-boolean-expression/description/
pub struct Solution;

#[derive(Debug, Copy, Clone)]
enum Operand {
    Value(bool),
    End,
}
impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let expression = expression.chars();
        let mut values_stack = Vec::default();
        type LogicalOp = fn(bool, bool) -> bool;
        let mut ops_stack: Vec<(bool, LogicalOp)> = Vec::default();

        use Operand::*;
        for c in expression {
            match c {
                '(' => values_stack.push(End),
                ')' => {
                    let (mut acc, ops) = ops_stack.pop().unwrap();
                    while let Some(Value(logical)) = values_stack.pop() {
                        acc = ops(acc, logical);
                    }
                    values_stack.push(Value(acc));
                }
                '&' => ops_stack.push((true, |acc, logical| acc && logical)),
                '|' => ops_stack.push((false, |acc, logical| acc || logical)),
                '!' => ops_stack.push((true, |acc, logical| acc && !logical)),
                't' => values_stack.push(Value(true)),
                'f' => values_stack.push(Value(false)),
                ',' => {}
                _ => unreachable!(),
            }
        }

        match values_stack.pop().unwrap() {
            End => unreachable!(),
            Value(logical) => logical,
        }
    }
}

fn main() {
    let test_cases = [
        ("&(|(f))", false),
        ("|(f,f,f,t)", true),
        ("!(&(f,t))", true),
    ];

    for (expression, expected) in test_cases {
        println!(
            "{} expected: {}",
            Solution::parse_bool_expr(expression.to_string()),
            expected
        );
    }
}

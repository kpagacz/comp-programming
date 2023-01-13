struct Solution {}

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug)]
enum Token {
    Num(i64),
    LeftPar,
    RightPar,
    Op(Operator),
}

fn lex(expression: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    expression.split_whitespace().for_each(|slice| {
        if slice == "+" {
            tokens.push(Token::Op(Operator::Plus));
            return;
        }
        if slice == "*" {
            tokens.push(Token::Op(Operator::Multiply));
            return;
        }
        if slice.ends_with(")") {
            slice.split_inclusive(")").for_each(|slice| {
                if slice == ")" {
                    tokens.push(Token::RightPar);
                } else {
                    let (num, _) = slice.split_at(slice.len() - 1);
                    tokens.push(Token::Num(num.parse::<i64>().unwrap()));
                    tokens.push(Token::RightPar);
                }
            });
            return;
        }
        if slice.starts_with("(") {
            slice.split_inclusive("(").for_each(|slice| {
                if slice == "(" {
                    tokens.push(Token::LeftPar);
                } else {
                    tokens.push(Token::Num(slice.parse::<i64>().unwrap()));
                }
            });
            return;
        }
        tokens.push(Token::Num(slice.parse::<i64>().unwrap()));
    });
    return tokens;
}

impl Solution {
    fn part2(self: Solution, path: &str) -> i64 {
        let expressions = Self::parse_input(path);
        expressions.iter().for_each(|expression| {
            println!("{}", expression);
            let tokens = lex(expression);
            println!("{:?}", tokens);
        });

        return -1;
    }

    fn parse_input(path: &str) -> Vec<String> {
        let contents = std::fs::read_to_string(path).unwrap();
        return contents.lines().map(|line| line.to_owned()).collect();
    }
}

fn main() {
    let s = Solution {};
    println!("Part 2: {}", s.part2("test"));
}

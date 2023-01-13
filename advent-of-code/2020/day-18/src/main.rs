#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Multiply,
}

#[derive(Debug, PartialEq)]
enum Token {
    Num(i64),
    LeftPar,
    RightPar,
    Op(Operator),
    Eof,
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
    tokens.push(Token::Eof);
    return tokens;
}

#[derive(Debug)]
struct Expr {
    term: Term,
    expr_opts: Vec<ExprOpt>,
}
#[derive(Debug)]
struct ExprOpt {
    expr: Expr,
}
#[derive(Debug)]
struct Term {
    num: i64,
    term_opts: Vec<TermOpt>,
}
#[derive(Debug)]
struct TermOpt {
    expr: Expr,
}

struct Parser {
    tokens: Vec<Token>,
    lookahead: usize,
}

impl<'a> Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            lookahead: 0,
        }
    }

    fn parse(&mut self) -> Expr {
        self.expr()
    }

    fn accept(&'a mut self) -> &'a Token {
        let token = &self.tokens[self.lookahead];
        self.lookahead += 1;
        token
    }

    fn term_opt(&mut self) -> TermOpt {
        if self.accept() != &Token::Op(Operator::Plus) {
            panic!("Expected a plus")
        }

        if self.tokens[self.lookahead] != Token::LeftPar {
            let num_token = self.accept();
            let value;
            match num_token {
                Token::Num(int) => value = *int,
                _ => panic!("Expected a Num"),
            }
            return TermOpt {
                expr: Expr {
                    term: Term {
                        num: value,
                        term_opts: Vec::new(),
                    },
                    expr_opts: Vec::new(),
                },
            };
        } else {
            return TermOpt { expr: self.expr() };
        }
    }

    fn term(&mut self) -> Term {
        let num = self.accept();
        let value: i64;
        match num {
            Token::Num(int) => value = *int,
            _ => panic!("Expect Num"),
        }

        let mut term_opts = Vec::new();
        while &self.tokens[self.lookahead] == &Token::Op(Operator::Plus) {
            term_opts.push(self.term_opt());
        }

        return Term {
            num: value,
            term_opts,
        };
    }

    fn expr_opt(&mut self) -> ExprOpt {
        let multiply = self.accept();
        if multiply != &Token::Op(Operator::Multiply) {
            panic!("Expected a multiplication")
        }

        ExprOpt { expr: self.expr() }
    }

    fn expr(&mut self) -> Expr {
        while self.tokens[self.lookahead] == Token::LeftPar {
            self.accept();
        }
        let term = self.term();
        let mut expr_opts = Vec::new();
        while self.tokens[self.lookahead] == Token::Op(Operator::Multiply) {
            expr_opts.push(self.expr_opt());
        }

        while self.tokens[self.lookahead] == Token::RightPar {
            self.accept();
        }

        Expr { term, expr_opts }
    }
}

fn eval_expr(expr: &Expr) -> i64 {
    let mut term = eval_term(&expr.term);
    expr.expr_opts
        .iter()
        .for_each(|expr_opt| term *= eval_expr_opt(expr_opt));

    term
}

fn eval_term(term: &Term) -> i64 {
    let mut num = term.num;
    term.term_opts
        .iter()
        .for_each(|term_opt| num += eval_term_opt(term_opt));

    num
}

fn eval_expr_opt(expr_opt: &ExprOpt) -> i64 {
    eval_expr(&expr_opt.expr)
}

fn eval_term_opt(term_opt: &TermOpt) -> i64 {
    eval_expr(&term_opt.expr)
}

struct Solution {}
impl Solution {
    fn part2(self: Solution, path: &str) -> i64 {
        let expressions = Self::parse_input(path);
        expressions.iter().for_each(|expression| {
            println!("{}", expression);
            let tokens = lex(expression);
            // println!("{:?}", tokens);

            let mut parser = Parser::new(tokens);
            let expr = parser.parse();
            println!("{:?}", expr);

            let result = eval_expr(&expr);
            println!("{}", &result);
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

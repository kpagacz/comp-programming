use std::collections::HashMap;

struct Solution {}

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

    fn part1(&self, path: &str) -> i64 {
        let (productions, texts) = self.parse_input(path);
        println!("{:?}", &productions);

        let symbols = vec!["1".to_owned(), "2".to_owned()];
        let with = vec![vec!["a".to_owned()], vec!["b".to_owned(), "a".to_owned()]];
        let res = self.replace_one(&symbols, "1", &with);
        println!("{:?}", res);
        -1
    }

    fn replace_production(
        self,
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

    fn eval(
        &self,
        productions: &HashMap<String, Vec<Vec<String>>>,
        current_production: &str,
        depth: i32,
    ) {
        if self.is_simple(
            &productions
                .get(current_production)
                .expect(format!("could not find {}", &current_production).as_str()),
        ) {
            println!("Dug to depth: {}", &depth);
        } else {
            productions
                .get(current_production)
                .expect(format!("could not find {}", &current_production).as_str())
                .iter()
                .for_each(|production| {
                    production
                        .iter()
                        .for_each(|symbol| self.eval(productions, symbol, depth + 1))
                });
        }
    }

    fn is_simple(&self, production: &Vec<Vec<String>>) -> bool {
        production.iter().count() == 1
            && production[0].iter().count() == 1
            && (production[0][0] == "a" || production[0][0] == "b")
    }
}

fn main() {
    let s = Solution {};
    s.part1("input");
}

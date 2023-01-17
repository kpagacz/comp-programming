use std::{collections::HashMap, thread::current};

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

fn main() {
    let mut s = Solution {
        cache: HashMap::new(),
    };
    println!("Part 1: {}", s.part1("test2"));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_replace_one() {
        let s = Solution {
            cache: HashMap::new(),
        };
        let symbols = vec!["1".to_owned(), "2".to_owned()];
        let with = vec![vec!["a".to_owned()], vec!["b".to_owned(), "a".to_owned()]];
        let res = s.replace_one(&symbols, "1", &with);
        assert_eq!(
            res,
            vec![
                vec!["a".to_owned(), "2".to_owned()],
                vec!["b".to_owned(), "a".to_owned(), "2".to_owned()]
            ]
        );
    }

    #[test]
    fn test_replace_production() {
        let s = Solution {
            cache: HashMap::new(),
        };
        let symbols = vec!["1".to_owned(), "2".to_owned()];
        let symbols2 = vec!["1".to_owned(), "1".to_owned()];
        let with = vec![vec!["a".to_owned()], vec!["b".to_owned(), "a".to_owned()]];
        let production = vec![symbols, symbols2];

        let res = s.replace_production(&production, "1", &with);
        println!("{:?}", res);
    }

    #[test]
    fn test_is_simple() {
        let s = Solution {
            cache: HashMap::new(),
        };
        let production = vec![vec!["a".to_owned()]];
        assert!(s.is_simple(&production) == false);
    }
}

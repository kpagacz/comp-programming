// https://adventofcode.com/2024/day/5
fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let (rules, productions) = input.split_once("\n\n").unwrap();

    let mut parents = vec![vec![]; 100];
    rules.lines().for_each(|line| {
        let (parent, child) = line.split_once('|').unwrap();
        let parent = parent.parse::<usize>().unwrap();
        let child = child.parse::<usize>().unwrap();
        parents[child].push(parent);
    });

    fn rec_parents(page: usize, parents: &[Vec<usize>], res: &mut Vec<usize>) {
        if res.contains(&page) {
            return;
        }
        res.extend_from_slice(&parents[page]);
        parents[page]
            .iter()
            .for_each(|parent| rec_parents(*parent, parents, res));
    }
    let mut full_ancestry = parents.clone();
    (0..parents.len()).for_each(|page| {
        let mut new_parents = vec![];
        rec_parents(page, &parents, &mut new_parents);
        full_ancestry[page] = new_parents;
    });
    full_ancestry.iter_mut().for_each(|tree| {
        tree.sort_unstable();
        tree.dedup();
    });

    let productions = productions
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    (full_ancestry, productions)
}

fn is_production_valid(ancestry: &[usize], future_pages: &[usize]) -> bool {
    future_pages
        .iter()
        .all(|future_page| ancestry.binary_search(future_page).is_err())
}

fn part1(input: &str) -> usize {
    let (full_ancestry, productions) = parse_input(input);
    // println!("{full_ancestry:?}\n{productions:?}");

    let mut answer = 0;
    'outer: for production in productions {
        let mut future_pages = vec![];
        for &page in production.iter().rev() {
            let ancestry = &full_ancestry[page];
            if !is_production_valid(ancestry, &future_pages) {
                continue 'outer;
            }
            future_pages.push(page);
        }
        answer += production[production.len() / 2];
    }
    answer
}

fn part2(input: &str) -> usize {
    let (rules, productions) = input.split_once("\n\n").unwrap();
    use std::cmp::Ordering::*;
    use std::collections::HashMap;
    let relations: HashMap<(usize, usize), std::cmp::Ordering> =
        rules.lines().fold(HashMap::new(), |mut map, line| {
            let (first, second) = line.split_once('|').unwrap();
            let first = first.parse::<usize>().unwrap();
            let second = second.parse::<usize>().unwrap();
            map.insert((first, second), Less);
            map.insert((second, first), Greater);
            map
        });
    let productions: Vec<Vec<usize>> = productions
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<usize>().unwrap())
                .collect()
        })
        .collect();
    let mut answer = 0;
    for production in productions {
        let mut copy = production.clone();
        copy.sort_unstable_by(|a, b| *relations.get(&(*a, *b)).unwrap_or(&Equal));
        if copy != production {
            answer += copy[copy.len() / 2];
        }
    }
    answer
}

fn main() {
    let test = include_str!("../test");
    let input = include_str!("../input");

    println!("Part 1 (test): {}", part1(test));
    println!("Part 1: {}", part1(input));

    println!("Part 2 (test): {}", part2(test));
    println!("Part 2: {}", part2(input));
}

use std::collections::{HashMap, HashSet};

type List<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;
type AllergenMap<'a> = HashMap<&'a str, Vec<HashSet<&'a str>>>;

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    let mut out = vec![];
    let regex = regex::Regex::new(r"([[:alpha:][:space:]]+) \(contains ([[:alpha:][:space:],]+)\)")
        .unwrap();
    input.lines().for_each(|line| {
        let captures = regex.captures(line).unwrap();
        let ingredients = captures
            .get(1)
            .unwrap()
            .as_str()
            .split_whitespace()
            .collect();
        let allergens = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(",")
            .map(str::trim)
            .collect();
        out.push((ingredients, allergens));
    });

    out
}

fn map_allergens_to_ingredient_lists<'a>(list: &'a List) -> AllergenMap<'a> {
    let mut map = HashMap::new();
    list.iter().for_each(|(ingredients, allergens)| {
        allergens.iter().for_each(|&allergen| {
            map.entry(allergen)
                .or_insert(vec![])
                .push(ingredients.iter().cloned().collect());
        });
    });
    map
}

fn find_intersections<'a>(mut allergen_map: AllergenMap<'a>) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut intersections = HashMap::new();
    allergen_map
        .iter_mut()
        .for_each(|(&key, ingredients_lists)| {
            let mut intersection = ingredients_lists.pop().unwrap();
            intersection.retain(|ingredient| {
                ingredients_lists
                    .iter()
                    .all(|list| list.contains(ingredient))
            });
            intersections.insert(key, intersection);
        });

    intersections
}

fn guess_alergens<'a>(allergen_map: AllergenMap<'a>) -> HashMap<&'a str, &'a str> {
    let allergen_map_size = allergen_map.len();
    let mut intersections = find_intersections(allergen_map);
    let mut known_allergens = HashMap::new();

    while known_allergens.len() != allergen_map_size {
        intersections.iter_mut().for_each(|(&allergen, suspects)| {
            suspects.retain(|ingredient| known_allergens.values().all(|known| known != ingredient));
            if suspects.len() == 1 {
                known_allergens.insert(allergen, suspects.iter().copied().next().unwrap());
            }
        });
    }

    known_allergens
}

fn solve(path: &str) {
    let input = std::fs::read_to_string(path).unwrap();
    let list = parse_input(&input);
    let allergen_map = map_allergens_to_ingredient_lists(&list);

    let mut known_allergens = guess_alergens(allergen_map); // allergen -> ingredient

    // Part 1
    let known_alergic_ingredients: HashSet<_> = known_allergens.values().copied().collect();
    let part1 = list
        .iter()
        .flat_map(|(ingredients, _)| {
            ingredients
                .iter()
                .map(|ingredient| known_alergic_ingredients.contains(ingredient))
        })
        .filter(|b| *b == false)
        .count();
    println!("Part 1: {}", part1);

    // Part 2
    let mut allergens_vec: Vec<_> = known_allergens.drain().collect();
    allergens_vec.sort_by(|first, second| first.0.partial_cmp(second.0).unwrap());
    let canonical_dangerous_list = allergens_vec
        .iter()
        .map(|(_, ingredient)| &**ingredient)
        .map(String::from)
        .collect::<Vec<String>>()
        .join(",");
    println!("Part 2: {canonical_dangerous_list}");
}
fn main() {
    solve("input");
}

type Molecule = (usize, String);
type Recipe = (Vec<Molecule>, Molecule);
fn parse_input(input: &str) -> Vec<Recipe> {
    input
        .lines()
        .map(|line| {
            let (ingredients, result) = line.split_once(" => ").unwrap();
            let ingredients: Vec<_> = ingredients
                .split(", ")
                .map(|molecule| {
                    let (amount, name) = molecule.split_once(" ").unwrap();
                    (amount.parse::<usize>().unwrap(), name.to_string())
                })
                .collect();
            let (amount, name) = result.split_once(" ").unwrap();
            (
                ingredients,
                (amount.parse::<usize>().unwrap(), name.to_string()),
            )
        })
        .collect()
}

fn sort_recipes_by_result(recipes: &mut Vec<Recipe>) {
    recipes.sort_unstable_by_key(|(_, res)| res.1.clone());
}

fn find_recipe<'a>(recipes: &'a [Recipe], molecule: &str) -> &'a Recipe {
    let found = recipes
        .binary_search_by_key(&molecule, |(_, (_, name))| name)
        .unwrap();
    recipes.get(found).unwrap()
}

fn molecules_for_a_molecule(
    recipes: &[Recipe],
    needed: &str,
    amount: usize,
) -> (Vec<Molecule>, usize) {
    // needed molecules, spare result
    let (ingredients, (res_amount, _)) = find_recipe(recipes, needed);
    let needed_reactions = (amount + res_amount - 1) / res_amount;

    let needed_ingredients: Vec<_> = ingredients
        .iter()
        .map(|molecule| (molecule.0 * needed_reactions, molecule.1.clone()))
        .collect();

    (needed_ingredients, needed_reactions * res_amount - amount)
}

use std::collections::HashMap;
fn molecules_for_a_molecule_with_spares(
    recipes: &[Recipe],
    needed: &str,
    amount: usize,
    spares: &mut HashMap<String, usize>,
) -> Vec<Molecule> {
    let spares_for_needed = spares.entry(needed.to_string()).or_insert(0);
    let amount_needed = amount.saturating_sub(*spares_for_needed);
    *spares_for_needed = spares_for_needed.saturating_sub(amount);
    let (ingredients_needed, spares_count) =
        molecules_for_a_molecule(recipes, needed, amount_needed);
    *spares.entry(needed.to_string()).or_insert(0) += spares_count;
    ingredients_needed
}

fn ore_for_molecule(recipes: &[Recipe], needed: &str, amount: usize) -> usize {
    let mut needed_molecules = HashMap::default();
    needed_molecules.insert(needed.to_string(), amount);

    fn is_only_ore_in_needed(needed: &HashMap<String, usize>) -> bool {
        needed
            .iter()
            .filter(|(key, value)| *key != "ORE" && **value > 0)
            .count()
            == 0
    }

    let mut spare_molecules = HashMap::default();
    while !is_only_ore_in_needed(&needed_molecules) {
        let non_ore_needed: Vec<_> = needed_molecules
            .iter()
            .filter(|(key, value)| *key != "ORE" && **value > 0)
            .map(|(key, value)| (key.clone(), *value))
            .collect();
        for (name, amount) in non_ore_needed {
            *needed_molecules.entry(name.clone()).or_insert(0) -= amount;
            let needed =
                molecules_for_a_molecule_with_spares(recipes, &name, amount, &mut spare_molecules);
            for (amount, needed) in needed {
                *needed_molecules.entry(needed).or_insert(0) += amount;
            }
        }
    }

    *needed_molecules.get("ORE").unwrap()
}

fn ore_for_fuel(recipes: &[Recipe], fuel_requested: usize) -> usize {
    ore_for_molecule(recipes, "FUEL", fuel_requested)
}

fn part1(input: &str) -> usize {
    let mut recipes = parse_input(input);
    sort_recipes_by_result(&mut recipes);

    ore_for_fuel(&recipes, 1)
}

fn part2(input: &str) -> usize {
    let mut recipes = parse_input(input);
    sort_recipes_by_result(&mut recipes);

    let possible_answers: Vec<_> = (0..50000000).collect();
    let partition_point = possible_answers
        .partition_point(|&possible_fuel| ore_for_fuel(&recipes, possible_fuel) <= 1000000000000);
    partition_point - 1
}

fn main() {
    let input = include_str!("../input");
    let test = include_str!("../test");
    let test2 = include_str!("../test2");
    let test3 = include_str!("../test3");
    let test4 = include_str!("../test4");

    // println!("Part 1 (test): {}", part1(test));
    // println!("Part 1 (test): {}", part1(test2));
    // println!("Part 1 (test): {}", part1(test3));
    // println!("Part 1 (test): {}", part1(test4));
    println!("Part 1: {}", part1(input));

    // println!("Part 2 (test): {}", part2(test2));
    // println!("Part 2 (test): {}", part2(test3));
    // println!("Part 2 (test): {}", part2(test4));
    println!("Part 2: {}", part2(input));
}

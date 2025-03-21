// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/description/?envType=daily-question&envId=2025-03-21
pub struct Solution;

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        use std::collections::HashMap;
        let mut all_things = HashMap::new();

        for supply in supplies {
            all_things.insert(supply, Some(true));
        }

        for recipe in recipes.iter() {
            all_things.insert(recipe.to_owned(), None);
        }

        fn is_recipe_possible(
            thing: &str,
            all_things: &mut HashMap<String, Option<bool>>,
            ingredients: &[Vec<String>],
            recipes: &[String],
        ) -> bool {
            match all_things.get(thing) {
                Some(optionally) => {
                    if let Some(is_possible) = optionally {
                        return *is_possible;
                    }
                }
                None => return false,
            }

            let pos = recipes.iter().position(|recipe| recipe == thing).unwrap();
            all_things.insert(thing.to_string(), Some(false));
            let answer = ingredients[pos]
                .iter()
                .all(|ingredient| is_recipe_possible(ingredient, all_things, ingredients, recipes));
            all_things.insert(thing.to_string(), Some(answer));

            answer
        }

        recipes
            .iter()
            .filter(|recipe| is_recipe_possible(recipe, &mut all_things, &ingredients, &recipes))
            .map(String::to_owned)
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

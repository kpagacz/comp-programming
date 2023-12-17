// https://leetcode.com/problems/design-a-food-rating-system/description/
use std::{cmp::Reverse, collections::BTreeMap};
struct FoodRatings {
    string_to_cuisine: BTreeMap<String, String>,
    foods: BTreeMap<String, Vec<(i32, Reverse<String>)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let food_to_cuisine =
            foods
                .iter()
                .zip(&cuisines)
                .fold(BTreeMap::new(), |mut map, (food, cuisine)| {
                    map.insert(food.clone(), cuisine.clone());
                    map
                });
        let mut food_map = BTreeMap::new();
        for i in 0..foods.len() {
            let (food, cuisine, rating) = (foods[i].clone(), cuisines[i].clone(), ratings[i]);
            food_map
                .entry(cuisine)
                .or_insert(vec![])
                .push((rating, Reverse(food)));
        }
        food_map
            .values_mut()
            .for_each(|cuisine| cuisine.sort_unstable());
        Self {
            string_to_cuisine: food_to_cuisine,
            foods: food_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = self.string_to_cuisine.get(&food).unwrap().clone();
        self.foods.entry(cuisine).and_modify(|cuisine| {
            let mut item = cuisine
                .iter_mut()
                .find(|(_, item)| &item.0 == &food)
                .unwrap();
            *item = (new_rating, Reverse(food));
            cuisine.sort_unstable();
        });
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.foods
            .get(&cuisine)
            .unwrap()
            .last()
            .unwrap()
            .1
             .0
            .clone()
    }
}

fn main() {
    println!("Hello, world!");
}

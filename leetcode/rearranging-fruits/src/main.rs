// https://leetcode.com/problems/rearranging-fruits/description/?envType=daily-question&envId=2025-08-02
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        use std::collections::BTreeMap;
        use std::collections::HashMap;

        let mut all_fruits: BTreeMap<i32, i32> = BTreeMap::new();

        for fruit in basket1.iter().chain(basket2.iter()) {
            *all_fruits.entry(*fruit).or_insert(0) += 1;
        }

        if !all_fruits.values().all(|&count| count % 2 == 0) {
            return -1;
        }
        // println!("all_fruits: {all_fruits:?}");

        let mut basket1_map: HashMap<i32, i32> = HashMap::new();
        for fruit in &basket1 {
            *basket1_map.entry(*fruit).or_insert(0) += 1;
        }

        let mut mismatched_count = 0i32;
        for (fruit, &count) in all_fruits.iter() {
            if let Some(&basket1_count) = basket1_map.get(fruit) {
                mismatched_count += (count / 2 - basket1_count).abs();
            } else {
                mismatched_count += count / 2;
            }
        }

        // println!("mismatched_count: {mismatched_count}");
        let mut min_cost = 0i64;

        let min_item = *all_fruits.iter().next().unwrap().0;
        let mut all_fruits_iter = all_fruits.into_iter();
        while mismatched_count > 0 {
            let (fruit, count) = all_fruits_iter.next().unwrap();

            let in_basket1 = *basket1_map.get(&fruit).unwrap_or(&0);
            let to_swap = (count / 2 - in_basket1).abs().min(mismatched_count / 2);
            min_cost += (to_swap as i64 * fruit as i64).min(to_swap as i64 * min_item as i64 * 2);

            mismatched_count -= to_swap * 2;
        }

        min_cost
    }
}

fn main() {
    let test_cases = [(
        vec![84, 80, 43, 8, 80, 88, 43, 14, 100, 88],
        vec![32, 32, 42, 68, 68, 100, 42, 84, 14, 8],
        48,
    )];

    for (basket1, basket2, expected) in test_cases {
        println!("basket1: {basket1:?}");
        println!("basket2: {basket2:?}");
        println!("{}  exp: {expected}", Solution::min_cost(basket1, basket2));
    }
}

// https://leetcode.com/problems/fruits-into-baskets-ii/description/?envType=daily-question&envId=2025-08-05
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, mut baskets: Vec<i32>) -> i32 {
        let mut unplaced = 0;
        'outer: for quantity in fruits {
            for basket in baskets.iter_mut() {
                if *basket >= quantity {
                    *basket = 0;
                    continue 'outer;
                }
            }
            unplaced += 1;
        }
        unplaced
    }
}

fn main() {
    let test_cases = [
        (vec![4, 2, 5], vec![3, 5, 4], 1),
        (vec![3, 6, 1], vec![6, 4, 7], 0),
    ];

    for (fruits, baskets, exp) in test_cases {
        println!(
            "{}  exp {exp}",
            Solution::num_of_unplaced_fruits(fruits, baskets)
        );
    }
}

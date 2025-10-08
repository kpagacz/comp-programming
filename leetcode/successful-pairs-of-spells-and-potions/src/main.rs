// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/description/?envType=daily-question&envId=2025-10-08
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn successful_pairs(mut spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        spells.iter_mut().for_each(|spell| {
            let pp = potions.partition_point(|&potion| (*spell as i64 * potion as i64) < success);
            *spell = (potions.len() - pp) as i32;
        });
        spells
    }
}
fn main() {
    println!("Hello, world!");
}

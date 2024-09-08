// https://leetcode.com/problems/minimum-amount-of-damage-dealt-to-bob/description/
pub struct Solution;

impl Solution {
    pub fn min_damage(power: i32, damage: Vec<i32>, health: Vec<i32>) -> i64 {
        let turns_to_kill: Vec<_> = health.iter().map(|h| (h + power - 1) / power).collect();

        let mut order: Vec<_> = (0..damage.len()).collect();
        order.sort_unstable_by(|&a, &b| {
            (turns_to_kill[a] * damage[b]).cmp(&(turns_to_kill[b] * damage[a]))
        });
        let mut t = 0i64;
        let mut total_damage = 0i64;
        for monster in order {
            t += turns_to_kill[monster] as i64;
            total_damage += t * damage[monster] as i64;
        }
        total_damage
    }
}

fn main() {
    println!("Hello, world!");
}

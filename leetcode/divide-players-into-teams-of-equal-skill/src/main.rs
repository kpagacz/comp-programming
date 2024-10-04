// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/description/
pub struct Solution;

impl Solution {
    pub fn divide_players(mut skill: Vec<i32>) -> i64 {
        skill.sort_unstable();

        let n = skill.len();
        let team_skill = skill[0] + skill[n - 1];

        let mut chemistry = 0i64;
        for i in 0..n / 2 {
            if skill[i] + skill[n - 1 - i] != team_skill {
                return -1;
            } else {
                chemistry += i64::from(skill[i] * skill[n - 1 - i]);
            }
        }

        chemistry
    }
}

fn main() {
    println!("Hello, world!");
}

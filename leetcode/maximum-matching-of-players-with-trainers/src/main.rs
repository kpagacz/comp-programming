// https://leetcode.com/problems/maximum-matching-of-players-with-trainers/description/?envType=daily-question&envId=2025-07-13
pub struct Solution;

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort_unstable();
        trainers.sort_unstable();

        let mut matchings = 0;
        let mut players = players.into_iter().rev();
        let mut it = trainers.len() - 1;

        while it < trainers.len() {
            if let Some(player) = players.next() {
                if trainers[it] >= player {
                    matchings += 1;
                    it = it.wrapping_sub(1);
                }
            } else {
                break;
            }
        }
        matchings
    }
}

fn main() {
    println!("Hello, world!");
}

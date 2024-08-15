// https://leetcode.com/problems/find-the-number-of-winning-players/description/
pub struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut player_picks = vec![vec![0usize; 11]; n];

        for pick in pick {
            player_picks[pick[0] as usize][pick[1] as usize] += 1;
        }

        player_picks
            .iter()
            .enumerate()
            .filter(|(pos, picks)| picks.iter().any(|count| count > pos))
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}

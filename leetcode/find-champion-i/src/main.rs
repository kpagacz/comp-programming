pub struct Solution {}

impl Solution {
    pub fn find_champion(grid: Vec<Vec<i32>>) -> i32 {
        let mut weaker = vec![false; grid.len()];
        for score in grid {
            (0..score.len()).for_each(|other_team| {
                if score[other_team] == 1 {
                    weaker[other_team] = true;
                }
            });
        }

        weaker.iter().position(|&team| !team).unwrap() as i32
    }
}
fn main() {
    println!("Hello, world!");
}

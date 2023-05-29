pub struct Solution {}

impl Solution {
    fn dfs(dp: &mut Vec<Vec<i32>>, piles: &Vec<i32>, step: usize, m: usize, total: i32) -> i32 {
        if dp[step][m] != -1 {
            return dp[step][m];
        }

        let mut res = 0;
        let mut taken = 0;

        for i in step..std::cmp::min(step + 2 * m, piles.len()) {
            taken += piles[i];
            res = std::cmp::max(
                res,
                total
                    - Self::dfs(
                        dp,
                        piles,
                        i + 1,
                        std::cmp::max(m, i - step + 1),
                        total - taken,
                    ),
            );
        }

        dp[step][m] = res;
        dp[step][m]
    }

    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut dp = vec![vec![-1; 101]; 200];
        Self::dfs(&mut dp, &piles, 0, 1, piles.iter().sum())
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/description/?envType=daily-question&envId=2026-01-03
struct Solution;

// For posterity: I solved this one myself! Without any hints
// and it's marked as hard. It was a fun little riddle.
// What helped me figure out a solution was thinking about:
// * what if there was no constraint - what's the number of ways then?
// * alright, what if it wasn't a grid, but just one row - how would I calculate
// the answer then?
// The second question helped me tremendously because I realized that I would
// calculate the number of ways the previous tile ended up being color A/B/C
// and then based on that calculate the number of ways the newly added tile
// can be A, B, C.
// I realized that for this problem, I need to consider not single tiles,
// but entire row of the grid - the last row.
// So then it was just a matter of matching what previous row can be
// based on the last row.
// I made one observation that helped, which is that the tile colouring
// is symmetric along the A -> B -> C axis, so I only considered
// four possible tile colourings of the last row (ABA, ABC, ACA, ACB).
// Turns out, I could consider only ABA and ABC and just doubled my
// answer, but I did not notice this symmetry when I was solving this
// on my own.
#[allow(dead_code)]
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut dp: [i64; 4] = [1, 1, 1, 1];
        const MOD: i64 = 10i64.pow(9) + 7;

        for _ in 1..n {
            let mut new_dp: [i64; 4] = [0, 0, 0, 0];
            new_dp[0] = (2 * dp[0] + dp[1] + dp[2] + dp[3]) % MOD;
            new_dp[1] = (dp[0] + 2 * dp[1] + dp[2]) % MOD;
            new_dp[2] = (dp[0] + dp[1] + 2 * dp[2] + dp[3]) % MOD;
            new_dp[3] = (dp[0] + dp[2] + 2 * dp[3]) % MOD;
            dp = new_dp;
        }

        ((3 * dp.into_iter().fold(0, |acc, elem| (acc + elem) % MOD)) % MOD) as _
    }
}

fn main() {
    println!("Hello, world!");
}

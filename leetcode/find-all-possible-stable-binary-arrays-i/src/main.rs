// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-i/description/?envType=daily-question&envId=2026-03-09
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let z = zero as usize;
        let o = one as usize;
        let l = limit as usize;
        let n = z.max(o);

        let mut dp = vec![vec![0; n + 1]; n + 1];

        for total in 1..=z + o {
            // diagonal walk
            for x in total.saturating_sub(n).max(1)..=total.min(n) {
                let y = total - x;
                dp[x][y] = if y == 0 {
                    (x <= l) as i32
                } else {
                    let mut ways = (dp[x - 1][y] + dp[y][x - 1]) % MOD;
                    if x > l {
                        ways = (ways - dp[y][x - 1 - l] + MOD) % MOD;
                    }
                    ways
                };
            }
        }

        ((dp[z][o] + dp[o][z]) % MOD) as i32
    }
}

fn main() {
    println!("Hello, world!");
}

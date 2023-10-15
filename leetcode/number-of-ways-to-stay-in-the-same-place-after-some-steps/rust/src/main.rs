// https://leetcode.com/problems/number-of-ways-to-stay-in-the-same-place-after-some-steps
pub struct Solution {}

impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        fn dfs(pos: i32, remaining: i32, mem: &mut Vec<Vec<i64>>, arr_len: i32) -> i64 {
            match (remaining == 0, pos == 0) {
                (true, true) => return 1,
                (true, false) => return 0,
                (_, _) => {}
            }
            if pos < 0 || pos >= arr_len {
                return 0;
            }
            if mem[pos as usize][remaining as usize] != -1 {
                return mem[pos as usize][remaining as usize];
            }

            let stayed = dfs(pos, remaining - 1, mem, arr_len);
            let moved_right = dfs(pos + 1, remaining - 1, mem, arr_len);
            let moved_left = dfs(pos - 1, remaining - 1, mem, arr_len);

            mem[pos as usize][remaining as usize] =
                (stayed + moved_right + moved_left) % 1_000_000_007;
            mem[pos as usize][remaining as usize]
        }

        let mut mem = vec![vec![-1; steps as usize + 1]; steps as usize + 1];
        dfs(0, steps, &mut mem, arr_len) as i32
    }

    pub fn num_ways_bottom_up(steps: i32, arr_len: i32) -> i32 {
        let available_positions = (steps / 2).min(arr_len - 1) as usize;
        let mut dp = vec![vec![0; steps as usize + 1]; available_positions + 3];
        dp[1][0] = 1;
        let m = 1_000_000_007;
        (1..dp[0].len()).for_each(|remaining| {
            (1..(dp.len() - 1)).for_each(|pos| {
                let moved_left = dp[pos - 1][remaining - 1];
                let moved_right = dp[pos + 1][remaining - 1];
                let not_moved = dp[pos][remaining - 1];
                dp[pos][remaining] = (((moved_left + moved_right) % m) + not_moved) % m;
            })
        });
        dp[1][steps as usize]
    }

    pub fn num_ways_bottom_up_space_optimized(steps: i32, arr_len: i32) -> i32 {
        let available_positions = (steps / 2).min(arr_len - 1) as usize;
        let mut dp = vec![0; available_positions + 3];
        let mut prev_dp = dp.clone();
        let n = dp.len();
        dp[1] = 1;
        let m = 1_000_000_007;
        (0..steps).for_each(|_| {
            std::mem::swap(&mut dp, &mut prev_dp);
            (1..(n - 1)).for_each(|pos| {
                dp[pos] = (((prev_dp[pos - 1] + prev_dp[pos]) % m) + prev_dp[pos + 1]) % m;
            })
        });

        dp[1]
    }
}

fn main() {
    let test_cases = vec![(3, 2), (2, 4), (4, 2)];
    for (steps, arr_len) in test_cases {
        println!("{}", Solution::num_ways(steps, arr_len));
        println!("{}", Solution::num_ways_bottom_up(steps, arr_len));
        println!(
            "{}",
            Solution::num_ways_bottom_up_space_optimized(steps, arr_len)
        );
    }
}

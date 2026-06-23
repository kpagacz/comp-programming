// https://leetcode.com/problems/number-of-zigzag-arrays-i/description/?envType=daily-question&envId=2026-06-23
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let m = (r - l + 1) as usize;

        let mut up = vec![0i64; m];
        let mut down = vec![0i64; m];

        for x in 0..m {
            up[x] = x as i64;
            down[x] = (m - 1 - x) as i64;
        }

        for _ in 3..=n {
            let mut new_up = vec![0i64; m];
            let mut new_down = vec![0i64; m];

            let mut prefix = 0i64;
            for y in 0..m {
                new_up[y] = prefix;
                prefix = (prefix + down[y]) % MOD;
            }

            let mut suffix = 0i64;
            for y in (0..m).rev() {
                new_down[y] = suffix;
                suffix = (suffix + up[y]) % MOD;
            }

            up = new_up;
            down = new_down;
        }

        let mut answer = 0i64;

        for x in 0..m {
            answer = (answer + up[x] + down[x]) % MOD;
        }

        answer as i32
    }
}

fn main() {
    println!("Hello, world!");
}

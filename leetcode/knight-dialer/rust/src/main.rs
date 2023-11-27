// https://leetcode.com/problems/knight-dialer/description/
pub struct Solution {}

const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let mut dp = [1i32; 10];
        for _ in 1..n {
            dp = [
                (dp[4] + dp[6]) % MOD,
                (dp[6] + dp[8]) % MOD,
                (dp[7] + dp[9]) % MOD,
                (dp[4] + dp[8]) % MOD,
                (((dp[0] + dp[3]) % MOD) + dp[9]) % MOD,
                0,
                (((dp[0] + dp[1]) % MOD) + dp[7]) % MOD,
                (dp[2] + dp[6]) % MOD,
                (dp[1] + dp[3]) % MOD,
                (dp[2] + dp[4]) % MOD,
            ];
        }

        dp.iter().fold(0, |total, summand| (total + summand) % MOD)
    }
    pub fn knight_dialer_golfing(n: i32) -> i32 {
        (0..n)
            .fold(([1i32; 10], [1i32; 10]), |(mut dp, mut prev_dp), _| {
                std::mem::swap(&mut dp, &mut prev_dp);
                dp[0] = (prev_dp[4] + prev_dp[6]) % MOD;
                dp[1] = (prev_dp[6] + prev_dp[8]) % MOD;
                dp[2] = (prev_dp[7] + prev_dp[9]) % MOD;
                dp[3] = (prev_dp[4] + prev_dp[8]) % MOD;
                dp[4] = (((prev_dp[0] + prev_dp[3]) % MOD) + prev_dp[9]) % MOD;
                dp[5] = 0;
                dp[6] = (((prev_dp[0] + prev_dp[1]) % MOD) + prev_dp[7]) % MOD;
                dp[7] = (prev_dp[2] + prev_dp[6]) % MOD;
                dp[8] = (prev_dp[1] + prev_dp[3]) % MOD;
                dp[9] = (prev_dp[2] + prev_dp[4]) % MOD;
                (dp, prev_dp)
            })
            .0
            .iter()
            .fold(0, |total, summand| (total + summand) % MOD)
    }
    pub fn knight_dialer_slow(n: i32) -> i32 {
        use std::collections::BTreeMap;
        let possible_moves = BTreeMap::from([
            (0, vec![4, 6]),
            (1, vec![6, 8]),
            (2, vec![7, 9]),
            (3, vec![4, 8]),
            (4, vec![0, 3, 9]),
            (5, vec![]),
            (6, vec![0, 1, 7]),
            (7, vec![2, 6]),
            (8, vec![1, 3]),
            (9, vec![2, 4]),
        ]);

        fn rec(
            moves_left: i32,
            last_move: i32,
            possible_moves: &BTreeMap<i32, Vec<i32>>,
            cache: &mut BTreeMap<(i32, i32), i32>,
        ) -> i32 {
            if let Some(cached) = cache.get(&(moves_left, last_move)) {
                return *cached;
            }
            if moves_left == 0 {
                return 1;
            }
            let next_moves = possible_moves.get(&last_move).unwrap();
            let answer = next_moves
                .iter()
                .map(|&next_move| rec(moves_left - 1, next_move, possible_moves, cache))
                .fold(0, |total, summand| (total + summand) % MOD);
            cache.insert((moves_left, last_move), answer);
            answer
        }

        let mut cache = BTreeMap::new();
        (0..=9i32)
            .map(|first_move| rec(n - 1, first_move, &possible_moves, &mut cache))
            .fold(0, |total, summand| (total + summand) % MOD /*  */)
    }
}
fn main() {
    println!("Hello, world!");
}

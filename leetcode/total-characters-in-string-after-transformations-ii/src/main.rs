// https://leetcode.com/problems/total-characters-in-string-after-transformations-ii/description/?envType=daily-question&envId=2025-05-14
pub struct Solution;

const MOD: i64 = 10i64.pow(9) + 7;
impl Solution {
    fn multiply(m1: [[i64; 26]; 26], m2: [[i64; 26]; 26]) -> [[i64; 26]; 26] {
        let mut answer = [[i64::default(); 26]; 26];
        for row in 0..26 {
            for col in 0..26 {
                (0..26).for_each(|k| {
                    answer[row][col] += m1[row][k] * m2[k][col];
                    answer[row][col] %= MOD;
                });
            }
        }
        answer
    }
    pub fn length_after_transformations(s: String, mut t: i32, nums: Vec<i32>) -> i32 {
        // 0. Count the frequencies of letters
        let freqs = s.as_bytes().iter().fold([0i64; 26], |mut acc, &b| {
            acc[(b - b'a') as usize] += 1;
            acc
        });
        // println!("Frequency matrix: {freqs:?}");
        // 1. Fill the transition matrix
        let mut transition = [[0i64; 26]; 26];
        for row in 0..26 {
            for col in (row + 1)..(row + 1 + nums[row] as usize) {
                transition[row][col % 26] = 1;
            }
        }
        // 2. Exponentiate the transition matrix T^t
        // Intialize as identity
        let mut ans = [[0i64; 26]; 26];
        (0..26).for_each(|i| {
            ans[i][i] = 1;
        });
        while t > 0 {
            if t & 1 == 1 {
                ans = Solution::multiply(ans, transition);
            }
            transition = Solution::multiply(transition, transition);
            t >>= 1;
        }
        // println!("Final result of exponentiation:");
        // for row in ans {
        //     println!("{row:?}");
        // }
        // 3. Multiply the frequency matrix with the result of exponentiation
        let mut final_freqs = [0i64; 26];
        for i in 0..26 {
            (0..ans[0].len()).for_each(|k| {
                final_freqs[i] += freqs[i] * ans[i][k];
                final_freqs[i] %= MOD;
            });
        }
        // 4. Sum the resulting matrix and return
        final_freqs
            .into_iter()
            .fold(0i32, |acc, freq| (acc + freq as i32) % MOD as i32)
    }
}

fn main() {
    let test_cases = [
        (
            "abcyy",
            2,
            vec![
                1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
            ],
        ),
        (
            "azbk",
            1,
            vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            ],
        ),
        (
            "azbk",
            10i32.pow(9),
            vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            ],
        ),
        (
            "k",
            2,
            vec![
                2, 2, 1, 3, 1, 1, 2, 3, 3, 2, 1, 2, 2, 1, 1, 3, 1, 2, 2, 1, 3, 3, 3, 2, 2, 1,
            ],
        ),
    ];
    for (s, t, nums) in test_cases {
        println!(
            "{}",
            Solution::length_after_transformations(s.to_string(), t, nums)
        );
    }
}

// https://leetcode.com/problems/find-the-maximum-number-of-fruits-collected/description/?envType=daily-question&envId=2025-08-07
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_collected_fruits(mut fruits: Vec<Vec<i32>>) -> i32 {
        fn dp(fruits: &[Vec<i32>]) -> i32 {
            let mut prev = vec![i32::MIN; fruits.len()];
            let mut curr = vec![i32::MIN; fruits.len()];

            let n = fruits.len();
            prev[n - 1] = fruits[0][n - 1];
            for i in 1..=n - 2 {
                for j in (n - 1 - i).max(i + 1)..n {
                    curr[j] = prev[j];
                    if j > 0 {
                        curr[j] = curr[j].max(prev[j - 1]);
                    }
                    if j < n - 1 {
                        curr[j] = curr[j].max(prev[j + 1]);
                    }
                    curr[j] += fruits[i][j];
                }
                std::mem::swap(&mut prev, &mut curr);
            }
            prev[n - 1]
        }

        let mut collected: i32 = (0..fruits.len()).map(|i| fruits[i][i]).sum();
        collected += dp(&fruits);
        for i in 0..fruits.len() {
            for j in 0..i {
                let tmp = fruits[i][j];
                fruits[i][j] = fruits[j][i];
                fruits[j][i] = tmp;
            }
        }
        collected += dp(&fruits);
        collected
    }
}

fn main() {
    println!("Hello, world!");
}

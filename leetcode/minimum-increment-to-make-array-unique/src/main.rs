// https://leetcode.com/problems/minimum-increment-to-make-array-unique/description/
pub struct Solution;

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 100_001];
        nums.iter().for_each(|num| freq[*num as usize] += 1);

        let mut answer = 0;
        for i in 0..freq.len() - 1 {
            if freq[i] > 1 {
                freq[i + 1] += freq[i] - 1;
                answer += freq[i] - 1;
            }
        }

        let past_end = freq[100_000];
        if past_end > 1 {
            let to_move = past_end - 1;
            answer += (to_move + 1) * to_move / 2;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

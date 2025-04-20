// https://leetcode.com/problems/rabbits-in-forest/description/?envType=daily-question&envId=2025-04-20
pub struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut freq = [0; 1001];

        for answer in answers {
            freq[answer as usize] += 1;
        }

        freq.into_iter()
            .enumerate()
            .map(|(num, rabbits)| {
                let num = num as i32;
                ((rabbits + num) / (num + 1)) * (num + 1)
            })
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
}

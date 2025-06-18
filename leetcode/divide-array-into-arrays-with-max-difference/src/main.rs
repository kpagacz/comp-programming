// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/description/?envType=daily-question&envId=2025-06-18
pub struct Solution;

impl Solution {
    pub fn divide_array(nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        let n = nums.len();
        let mut counts = vec![0; 100_001];
        for num in nums {
            counts[num as usize] += 1;
        }

        let mut answer = vec![vec![]; n / 3];
        let mut counts_it = 0usize;
        let mut answer_it = 0usize;

        while answer_it < answer.len() {
            counts_it += counts[counts_it..]
                .iter()
                .position(|&count| count > 0)
                .unwrap();
            let min = counts_it;
            answer[answer_it].push(counts_it as i32);
            counts[counts_it] -= 1;
            for _ in 0..2 {
                counts_it += counts[counts_it..]
                    .iter()
                    .position(|&count| count > 0)
                    .unwrap();
                if counts_it - min > k {
                    return vec![];
                }
                answer[answer_it].push(counts_it as i32);
                counts[counts_it] -= 1;
            }
            answer_it += 1;
        }

        answer
    }
}

fn main() {
    let test_cases = [(vec![1, 3, 4, 8, 7, 9, 3, 5, 1], 2)];
    for (nums, k) in test_cases {
        println!("Nums: {nums:?}");
        println!("{:?}", Solution::divide_array(nums, k));
    }
}

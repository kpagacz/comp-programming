// find-all-k-distant-indices-in-an-array
pub struct Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut k_finder = nums.iter().position(|num| *num == key).unwrap();
        let mut answer = vec![];
        for i in 0..nums.len() {
            while k_finder + k < i {
                k_finder += 1 + nums[k_finder + 1..]
                    .iter()
                    .position(|num| *num == key)
                    .unwrap_or(1000);
            }
            if k_finder < nums.len() && k_finder.abs_diff(i) <= k {
                answer.push(i as _);
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

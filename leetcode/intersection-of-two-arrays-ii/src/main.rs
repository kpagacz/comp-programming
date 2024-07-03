// https://leetcode.com/problems/intersection-of-two-arrays-ii/description/
pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut counting1 = vec![0; 1001];
        let mut counting2 = vec![0; 1001];

        for num in nums1 {
            counting1[num as usize] += 1;
        }
        for num in nums2 {
            counting2[num as usize] += 1;
        }

        counting1.iter_mut().enumerate().for_each(|(pos, freq)| {
            *freq = (*freq).min(counting2[pos]);
        });

        counting1
            .iter()
            .enumerate()
            .flat_map(|(pos, count)| vec![pos as i32; *count as usize])
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

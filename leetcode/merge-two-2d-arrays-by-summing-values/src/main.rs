// https://leetcode.com/problems/merge-two-2d-arrays-by-summing-values/description/
pub struct Solution;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![None; 1000];

        for arr in [nums1, nums2] {
            for id_val in arr {
                let (id, val) = (id_val[0] as usize, id_val[1]);
                let opt = answer[id].get_or_insert(0);
                *opt += val;
            }
        }

        answer
            .into_iter()
            .enumerate()
            .filter_map(|(pos, val)| Some(vec![pos as i32, val?]))
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

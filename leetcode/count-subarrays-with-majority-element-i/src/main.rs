// https://leetcode.com/problems/count-subarrays-with-majority-element-i/description/?envType=daily-question&envId=2026-06-25
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        // You can count inversions on this using customized merge sort
        // apparently and do it in O(nlogn). Crazy
        let mut prefixes = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefixes[i + 1] = prefixes[i];
            if nums[i] == target {
                prefixes[i + 1] += 1;
            }
        }

        let mut subarrays = 0;
        for i in 0..prefixes.len() {
            let to_i = prefixes[i];
            for j in 0..i {
                let to_j = prefixes[j];
                if to_i - to_j > (i - j) / 2 {
                    subarrays += 1;
                }
            }
        }
        subarrays
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 2, 3];
        let target = 2;

        assert_eq!(Solution::count_majority_subarrays(nums, target), 5);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 1, 1];
        let target = 1;

        assert_eq!(Solution::count_majority_subarrays(nums, target), 10);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3];
        let target = 4;

        assert_eq!(Solution::count_majority_subarrays(nums, target), 0);
    }
}

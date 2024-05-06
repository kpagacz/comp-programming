// https://leetcode.com/contest/weekly-contest-395/problems/find-the-integer-added-to-array-ii/
pub struct Solution;

impl Solution {
    pub fn minimum_added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut answer = i32::MAX;
        let sum1 = nums1.iter().sum::<i32>();
        let sum2 = nums2.iter().sum::<i32>();
        let mut vec1 = [0; 1001];
        let mut vec2 = [0; 1001];
        for &num in &nums1 {
            vec1[num as usize] += 1;
        }
        for &num in &nums2 {
            vec2[num as usize] += 1;
        }

        for i in 0..nums1.len() {
            for j in i + 1..nums1.len() {
                let new_sum1 = sum1 - nums1[i] - nums1[j];
                vec1[nums1[i] as usize] -= 1;
                vec1[nums1[j] as usize] -= 1;
                if (sum2 - new_sum1) % nums2.len() as i32 == 0 {
                    let integer = (sum2 - new_sum1) / nums2.len() as i32;
                    println!("i {i} j {j} int: {integer}");
                    if vec2.iter().enumerate().all(|(pos, count)| {
                        let pos1 = pos as i32 - integer;
                        if *count != 0 {
                            pos1 >= 0 && pos1 < vec1.len() as i32 && vec1[pos1 as usize] == *count
                        } else {
                            true
                        }
                    }) {
                        answer = answer.min(integer);
                    }
                }
                vec1[nums1[i] as usize] += 1;
                vec1[nums1[j] as usize] += 1;
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

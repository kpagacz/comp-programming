// https://leetcode.com/problems/range-sum-of-sorted-subarray-sums/description/
pub struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let max = nums.iter().sum::<i32>() as usize;

        // Sum counts
        let mut counts: Vec<i32> = vec![0; 1 + max];

        // Compute subarray sums
        let mut a = vec![0; n as usize];
        for (i, x) in nums.into_iter().enumerate() {
            for j in 0..i {
                a[j] += x;
                counts[a[j] as usize] += 1;
            }
            a[i] = x;
            counts[x as usize] += 1;
        }

        let mut sum = 0;
        let mut cnt = 0;
        let mut left_found = false;
        let modulo = 1_000_000_007;
        for (i, x) in counts.into_iter().enumerate() {
            if x == 0 {
                continue;
            }
            cnt += x;
            if !left_found && cnt >= left {
                sum = (cnt - left + 1) * i as i32;
                left_found = true;
            } else if left_found {
                sum = (sum + x * i as i32) % modulo;
            }
            if cnt >= right {
                sum = sum - (cnt - right) * i as i32;
                break;
            }
        }
        if sum >= 0 {
            sum
        } else {
            modulo - sum
        }
    }
}

fn main() {
    println!("Hello, world!");
}

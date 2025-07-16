// https://leetcode.com/problems/find-the-maximum-length-of-valid-subsequence-i/description/?envType=daily-question&envId=2025-07-16
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        let odds = nums.iter().filter(|&num| *num % 2 == 1).count();
        let evens = nums.len() - odds;

        let mut even_odd = 0;
        let mut even_odd_next_even = true;
        let mut odd_even = 0;
        let mut odd_even_next_even = false;

        for num in nums {
            match num % 2 == 0 {
                true => {
                    if even_odd_next_even {
                        even_odd += 1;
                        even_odd_next_even ^= true;
                    }
                    if odd_even_next_even {
                        odd_even += 1;
                        odd_even_next_even ^= true;
                    }
                }
                false => {
                    if !even_odd_next_even {
                        even_odd += 1;
                        even_odd_next_even ^= true;
                    }
                    if !odd_even_next_even {
                        odd_even += 1;
                        odd_even_next_even ^= true;
                    }
                }
            }
        }

        odds.max(evens).max(even_odd).max(odd_even) as _
    }
}

fn main() {
    println!("Hello, world!");
}

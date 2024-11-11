// https://leetcode.com/problems/prime-subtraction-operation/description/
pub struct Solution;

impl Solution {
    pub fn prime_sub_operation(mut nums: Vec<i32>) -> bool {
        let mut primes = Solution::get_primes(1000);
        primes.reverse();

        let mut partition = primes.partition_point(|&prime| prime >= nums[0]);
        if partition < primes.len() {
            nums[0] -= primes[partition];
        }

        for i in 1..nums.len() {
            // nums[i] - prime > nums[i - 1]
            // prime < nums[i] - nums[i - 1]
            partition = primes.partition_point(|&prime| prime >= nums[i] - nums[i - 1]);
            if partition < primes.len() {
                nums[i] -= primes[partition];
            }
            if nums[i] <= nums[i - 1] {
                return false;
            }
        }

        true
    }

    fn get_primes(limit: usize) -> Vec<i32> {
        let mut is_prime = vec![true; limit + 1];
        let mut primes = Vec::with_capacity(limit + 1);

        for num in 2..is_prime.len() {
            if is_prime[num] {
                primes.push(num as i32);
                let mut multiple = num + num;
                while multiple < is_prime.len() {
                    is_prime[multiple] = false;
                    multiple += num;
                }
            }
        }
        primes
    }
}

fn main() {
    let test_case = [vec![4, 9, 6, 10], vec![5, 8, 3]];

    for nums in test_case {
        println!("{}", Solution::prime_sub_operation(nums));
    }
}

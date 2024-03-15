// https://leetcode.com/problems/product-of-array-except-self/description/
pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let prefix: Vec<_> = nums
            .iter()
            .scan(1, |acc, &num| {
                *acc *= num;
                Some(*acc)
            })
            .collect();
        let suffix: Vec<_> = nums
            .iter()
            .rev()
            .scan(1, |acc, &num| {
                *acc *= num;
                Some(*acc)
            })
            .collect();

        (0..nums.len())
            .map(|id| {
                let mut product_except_self = 1;
                if id > 0 {
                    product_except_self *= prefix[id - 1];
                }
                if id < nums.len() - 1 {
                    product_except_self *= suffix[nums.len() - 1 - 1 - id];
                }
                product_except_self
            })
            .collect()
    }

    pub fn product_except_self_faster(nums: Vec<i32>) -> Vec<i32> {
        let mut products = vec![1; nums.len()];

        for i in 1..nums.len() {
            products[i] = nums[i - 1] * products[i - 1];
        }

        let mut suffix = 1;
        for i in (0..nums.len()).rev() {
            products[i] *= suffix;
            suffix *= nums[i];
        }

        products
    }
}

fn main() {
    println!("Hello, world!");
}

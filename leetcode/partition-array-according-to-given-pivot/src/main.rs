// https://leetcode.com/problems/partition-array-according-to-given-pivot/description/
pub struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let (mut less, mut equal, mut greater) = (Vec::default(), Vec::default(), Vec::default());

        for num in nums {
            match num.cmp(&pivot) {
                std::cmp::Ordering::Less => less.push(num),
                std::cmp::Ordering::Equal => equal.push(num),
                std::cmp::Ordering::Greater => greater.push(num),
            }
        }

        [less, equal, greater].concat()
    }
}

fn main() {
    let tests = [(vec![9, 12, 5, 10, 14, 3, 10], 10)];

    for (nums, pivot) in tests {
        println!("{:?}", Solution::pivot_array(nums, pivot));
    }
}

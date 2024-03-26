// https://leetcode.com/problems/first-missing-positive/description/
pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let mut lowest = 1;
        let guard = nums.len() as i32 + 2;

        for num in &mut nums {
            if *num == guard {
                *num = guard + 1;
            }
        }

        // println!("initial {nums:?}");
        for i in 0..nums.len() {
            // println!("outer LOOP {i}");
            let mut address = nums[i] - 1;
            // println!("address: {address}");
            while address >= 0 && address < nums.len() as i32 && nums[address as usize] != guard {
                // println!("INNER LOOP");
                let tmp = nums[address as usize];
                nums[address as usize] = guard;
                // println!("tmp: {tmp} lowest: {lowest}");
                if lowest == address + 1 {
                    // println!("lowest == address + 1");
                    // println!("nums[lowest as usize - 1] {}", nums[lowest as usize - 1]);
                    while lowest <= nums.len() as i32 && nums[lowest as usize - 1] == guard {
                        lowest += 1;
                    }
                    // println!("new lowest: {lowest}");
                }
                address = tmp - 1;
                // println!("new address: {address} nums: {nums:?}");
            }
        }

        lowest
    }
}

fn main() {
    let test_cases = vec![
        vec![1, 2, 0],
        vec![3, 4, -1, 1],
        vec![7, 8, 9, 11, 12],
        vec![3, 4, 5, 6, 7, 1, 2],
        vec![1, 1, 1, 1, 2],
    ];

    for nums in test_cases {
        println!("{}", Solution::first_missing_positive(nums));
    }
}

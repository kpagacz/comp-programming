// https://leetcode.com/problems/longest-balanced-subarray-i/description/?envType=daily-question&envId=2026-02-10
struct Solution;

#[derive(Debug, Clone)]
struct NumCounter {
    nums: Vec<u32>,
    count: u32,
}

impl NumCounter {
    fn new(size: usize) -> Self {
        Self {
            nums: vec![0; size + 1],
            count: 0,
        }
    }

    fn insert(&mut self, num: i32) {
        self.nums[num as usize] += 1;
        if self.nums[num as usize] == 1 {
            self.count += 1;
        }
    }

    fn unique_count(&self) -> u32 {
        self.count
    }

    fn clear(&mut self) {
        self.count = 0;
        self.nums.fill(0);
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let max_num = *nums.iter().max().unwrap() as usize;
        let mut even_counter = NumCounter::new(max_num);
        let mut odd_counter = NumCounter::new(max_num);

        let mut max_length = 0;
        for i in 0..nums.len() {
            even_counter.clear();
            odd_counter.clear();
            for (j, &num) in nums.iter().enumerate().take(i + 1).rev() {
                if num & 1 == 1 {
                    odd_counter.insert(num);
                } else {
                    even_counter.insert(num);
                }

                if odd_counter.unique_count() == even_counter.unique_count() {
                    max_length = max_length.max(i - j + 1);
                }
            }
        }

        max_length as _
    }
}

fn main() {
    let test_cases = [
        (vec![2, 5, 4, 3], 4),
        (vec![3, 2, 2, 5, 4], 5),
        (vec![1, 2, 3, 2], 3),
    ];
    for (nums, exp) in test_cases {
        println!("{} exp {exp}", Solution::longest_balanced(nums));
    }
}

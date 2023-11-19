// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/description/
pub struct Solution {}

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut counting_sort = vec![0; 5 * 10usize.pow(4) + 1];
        let mut different_nums = 0;

        for num in nums {
            if counting_sort[num as usize] == 0 {
                different_nums += 1;
            }
            counting_sort[num as usize] += 1;
        }

        let mut answer = 0;
        for i in (1..counting_sort.len()).rev() {
            if counting_sort[i] != 0 {
                answer += (different_nums - 1) * counting_sort[i];
                different_nums -= 1;
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

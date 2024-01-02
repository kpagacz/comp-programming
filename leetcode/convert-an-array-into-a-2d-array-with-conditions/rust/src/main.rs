// https://leetcode.com/problems/convert-an-array-into-a-2d-array-with-conditions/description/?envType=daily-question&envId=2024-01-02
pub struct Solution;

impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut answer: Vec<Vec<i32>> = vec![];
        let mut it = 0;
        while let Some(num) = nums.pop() {
            while let Some(v) = answer.get(it) {
                if v.last().unwrap() == &num {
                    it += 1;
                } else {
                    break;
                }
            }
            match answer.get_mut(it) {
                Some(v) => v.push(num),
                None => answer.push(vec![num]),
            }
            it = 0;
        }
        answer
    }
}

fn main() {
    let test_cases = [vec![1, 3, 4, 1, 2, 3, 1]];
    for nums in test_cases {
        println!("{:?}", Solution::find_matrix(nums));
    }
}

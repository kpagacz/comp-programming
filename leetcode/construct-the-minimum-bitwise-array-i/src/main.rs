// https://leetcode.com/problems/construct-the-minimum-bitwise-array-i/description/?envType=daily-question&envId=2026-01-20
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn find_answer(prime: i32) -> i32 {
            let mut answer = -1;
            for it in 0..32 {
                if prime & (1 << it) > 0 {
                    answer = prime - (1 << it);
                } else {
                    break;
                }
            }
            answer
        }
        nums.iter_mut()
            .for_each(|prime| *prime = find_answer(*prime));
        nums
    }
}

fn main() {
    let test_cases = [(vec![2, 3, 5, 7], vec![-1, 1, 4, 3])];

    for (nums, exp) in test_cases {
        println!("{:?} exp: {exp:?}", Solution::min_bitwise_array(nums));
    }
}

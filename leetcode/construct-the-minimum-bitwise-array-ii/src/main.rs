// https://leetcode.com/problems/construct-the-minimum-bitwise-array-ii/description/?envType=daily-question&envId=2026-01-21
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_bitwise_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn find_answer(prime: i32) -> i32 {
            match prime.trailing_ones() {
                0 => -1,
                trailing_ones => prime ^ (1 << (trailing_ones - 1)),
            }
        }
        nums.iter_mut()
            .for_each(|prime| *prime = find_answer(*prime));
        nums
    }
}

fn main() {
    println!("Hello, world!");
}

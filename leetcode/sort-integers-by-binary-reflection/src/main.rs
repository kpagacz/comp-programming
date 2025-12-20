// https://leetcode.com/problems/sort-integers-by-binary-reflection/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_by_reflection(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable_by_key(|original| {
            let mut digits = vec![];
            let mut num = *original;
            while num > 0 {
                digits.push(num % 2);
                num >>= 1;
            }
            let mut reversed = 0;
            for digit in digits {
                reversed += digit;
                reversed <<= 1;
            }
            (reversed, *original)
        });
        nums
    }
}
fn main() {
    println!("Hello, world!");
}

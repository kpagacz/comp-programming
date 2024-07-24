// https://leetcode.com/problems/sort-the-jumbled-numbers/description/
pub struct Solution;

// What I learned:
// * sort_by_cached_key is a nice utility. Don't have to implement it on my own
impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_by_cached_key(|&num| {
            if num == 0 {
                return mapping[0];
            }
            let mut pow = 1;
            let mut mapped = 0;
            let mut num = num;
            while num > 0 {
                mapped += pow * mapping[(num % 10) as usize];
                num /= 10;
                pow *= 10;
            }
            mapped
        });
        nums
    }
}

fn main() {
    println!("Hello, world!");
}

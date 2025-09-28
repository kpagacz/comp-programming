// https://leetcode.com/problems/largest-perimeter-triangle/description/?envType=daily-question&envId=2025-09-28
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.windows(3)
            .rev()
            .find_map(|sides| {
                if sides[0] + sides[1] > sides[2] {
                    Some(sides[0] + sides[1] + sides[2])
                } else {
                    None
                }
            })
            .unwrap_or(0)
    }
}

fn main() {
    println!("Hello, world!");
}

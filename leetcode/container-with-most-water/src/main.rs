// https://leetcode.com/problems/container-with-most-water/description/?envType=daily-question&envId=2025-10-04
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0usize;
        let mut right = height.len() - 1;

        while left < right {
            let area = height[left].min(height[right]) * (right - left) as i32;
            max_area = max_area.max(area);
            if height[left] >= height[right] {
                right -= 1;
            } else {
                left += 1;
            }
        }
        max_area
    }
}
fn main() {
    println!("Hello, world!");
}

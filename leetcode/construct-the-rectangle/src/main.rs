// https://leetcode.com/problems/construct-the-rectangle/description/
pub struct Solution;

impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let square = (area as f32).sqrt();
        let square = square.ceil() as i32;
        for i in (1..=square).rev() {
            if area % i == 0 {
                return vec![i.max(area / i), i.min(area / i)];
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}

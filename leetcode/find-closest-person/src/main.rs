// https://leetcode.com/problems/find-closest-person/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match x.abs_diff(z).cmp(&(y.abs_diff(z))) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Greater => 2,
            std::cmp::Ordering::Equal => 0,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

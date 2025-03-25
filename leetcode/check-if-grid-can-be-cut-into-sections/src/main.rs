// https://leetcode.com/problems/check-if-grid-can-be-cut-into-sections/description/?envType=daily-question&envId=2025-03-25
pub struct Solution;

impl Solution {
    pub fn check_valid_cuts(n: i32, mut rectangles: Vec<Vec<i32>>) -> bool {
        rectangles.sort_by_cached_key(|rectangle| (rectangle[0], rectangle[2]));
        let mut parts = 0;
        let mut mem = rectangles[0][2];

        for rect in &rectangles[1..] {
            let (start_x, end_x) = (rect[0], rect[2]);
            if start_x >= mem {
                parts += 1;
            }
            mem = mem.max(end_x);
        }
        if parts >= 2 {
            return true;
        } else {
            parts = 0;
        }
        rectangles.sort_by_cached_key(|rectangle| (rectangle[1], rectangle[3]));
        mem = rectangles[0][3];
        for rect in &rectangles[1..] {
            let (start_y, end_y) = (rect[1], rect[3]);
            if start_y >= mem {
                parts += 1;
            }
            mem = mem.max(end_y);
        }

        parts >= 2
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/minimum-number-of-operations-to-move-all-balls-to-each-box/description/
pub struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut to_the_left = 0;
        let boxes = boxes.as_bytes();
        let total = boxes.iter().filter(|&b| *b == b'1').count() as i32;
        let mut curr_dist =
            boxes.iter().enumerate().fold(
                0i32,
                |acc, (pos, &b)| {
                    if b == b'1' {
                        acc + pos as i32
                    } else {
                        acc
                    }
                },
            );

        let mut answer = Vec::with_capacity(boxes.len());
        for &b in boxes {
            answer.push(curr_dist);
            if b == b'1' {
                to_the_left += 1;
            }
            curr_dist += to_the_left;
            curr_dist -= total - to_the_left;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

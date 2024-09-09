// https://leetcode.com/problems/spiral-matrix-iv/description/
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let (m, n) = (m as usize, n as usize);
        let (mut top, mut bottom, mut left, mut right) = (0, m - 1, 0, n - 1);
        let mut answer = vec![vec![-1; n]; m];
        'outer: loop {
            for col in left..=right {
                if let Some(mut node) = head {
                    answer[top][col] = node.val;
                    head = node.next.take();
                } else {
                    break 'outer;
                }
            }
            top += 1;

            for row in top..=bottom {
                if let Some(mut node) = head {
                    answer[row][right] = node.val;
                    head = node.next.take();
                } else {
                    break 'outer;
                }
            }
            right -= 1;

            for col in (left..=right).rev() {
                if let Some(mut node) = head {
                    answer[bottom][col] = node.val;
                    head = node.next.take();
                } else {
                    break 'outer;
                }
            }
            bottom -= 1;

            for row in (top..=bottom).rev() {
                if let Some(mut node) = head {
                    answer[row][left] = node.val;
                    head = node.next.take();
                } else {
                    break 'outer;
                }
            }
            left += 1;
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

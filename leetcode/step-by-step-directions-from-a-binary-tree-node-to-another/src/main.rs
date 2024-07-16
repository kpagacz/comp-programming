// https://leetcode.com/problems/step-by-step-directions-from-a-binary-tree-node-to-another/description/
pub struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn get_directions(
        root: Option<Rc<RefCell<TreeNode>>>,
        start_value: i32,
        dest_value: i32,
    ) -> String {
        fn rec(
            node: Option<Rc<RefCell<TreeNode>>>,
            so_far: &mut Vec<u8>,
            destination: i32,
        ) -> bool {
            if let Some(node) = node {
                let borrow = node.borrow_mut();
                if borrow.val == destination {
                    true
                } else {
                    so_far.push(b'L');
                    if rec(borrow.left.clone(), so_far, destination) {
                        true
                    } else {
                        so_far.pop();
                        so_far.push(b'R');
                        if rec(borrow.right.clone(), so_far, destination) {
                            true
                        } else {
                            so_far.pop();
                            false
                        }
                    }
                }
            } else {
                false
            }
        }

        let mut to_start = vec![];
        let mut to_end = vec![];
        rec(root.clone(), &mut to_start, start_value);
        rec(root.clone(), &mut to_end, dest_value);
        let mut it = 0;
        while it < to_start.len() && it < to_end.len() && to_start[it] == to_end[it] {
            it += 1;
        }
        let to_start = &to_start[it..];
        let to_end = &to_end[it..];

        let from = "U".repeat(to_start.len());
        let to_end = std::str::from_utf8(to_end).unwrap();
        format!("{from}{to_end}")
    }
}

fn main() {
    println!("Hello, world!");
}

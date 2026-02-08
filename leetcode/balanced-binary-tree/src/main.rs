// https://leetcode.com/problems/balanced-binary-tree/description/?envType=daily-question&envId=2026-02-08
struct Solution;
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
#[allow(dead_code)]
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn rec(node: Option<Rc<RefCell<TreeNode>>>) -> (bool, u32) {
            if let Some(node) = node {
                let mut borrow = node.borrow_mut();
                let (left_balanced, left_height) = rec(borrow.left.take());
                let (right_balanced, right_height) = rec(borrow.right.take());
                (
                    left_balanced && right_balanced && left_height.abs_diff(right_height) <= 1,
                    left_height.max(right_height) + 1,
                )
            } else {
                (true, 0)
            }
        }

        rec(root).0
    }
}

fn main() {
    println!("Hello, world!");
}

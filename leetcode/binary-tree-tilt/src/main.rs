// https://leetcode.com/problems/binary-tree-tilt/description/
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
    pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            if let Some(node) = node {
                let mut borrow = node.borrow_mut();
                let (left_sum, left_tilt) = rec(borrow.left.take());
                let (right_sum, right_tilt) = rec(borrow.right.take());
                (
                    left_sum + right_sum + borrow.val,
                    (left_sum - right_sum).abs() + left_tilt + right_tilt,
                )
            } else {
                (0, 0)
            }
        }

        rec(root).1
    }
}

fn main() {
    println!("Hello, world!");
}

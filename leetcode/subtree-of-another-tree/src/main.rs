// https://leetcode.com/problems/subtree-of-another-tree/
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
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if Solution::is_identical(&root, &sub_root) {
            return true;
        }

        if let Some(node) = root {
            let mut node = node.borrow_mut();
            Solution::is_subtree(node.left.take(), sub_root.clone())
                || Solution::is_subtree(node.right.take(), sub_root)
        } else {
            false
        }
    }

    fn is_identical(
        first: &Option<Rc<RefCell<TreeNode>>>,
        second: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (first, second) {
            (None, None) => true,
            (Some(first), Some(second)) => {
                let first = first.borrow();
                let second = second.borrow();
                if first.val != second.val {
                    false
                } else {
                    Solution::is_identical(&first.left, &second.left)
                        && Solution::is_identical(&first.right, &second.right)
                }
            }
            _ => false,
        }
    }
}
fn main() {
    println!("Hello, world!");
}

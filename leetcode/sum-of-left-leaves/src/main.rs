// https://leetcode.com/problems/sum-of-left-leaves/description/
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
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        Solution::sum_of_left_leaves_rec(root, false)
    }

    fn sum_of_left_leaves_rec(node: Option<Rc<RefCell<TreeNode>>>, is_left: bool) -> i32 {
        if node.is_none() {
            return 0;
        }
        let node = node.unwrap();
        let ref_node = node.borrow();
        match (&ref_node.left, &ref_node.right) {
            (None, None) => {
                if is_left {
                    ref_node.val
                } else {
                    0
                }
            }
            (None, Some(right_child)) => {
                Solution::sum_of_left_leaves_rec(Some(Rc::clone(right_child)), false)
            }
            (Some(left_child), None) => {
                Solution::sum_of_left_leaves_rec(Some(Rc::clone(left_child)), true)
            }
            (Some(left_child), Some(right_child)) => {
                Solution::sum_of_left_leaves_rec(Some(Rc::clone(left_child)), true)
                    + Solution::sum_of_left_leaves_rec(Some(Rc::clone(right_child)), false)
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

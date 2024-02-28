// https://leetcode.com/problems/find-bottom-left-tree-value/description/
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
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::rec(&root).1
    }

    fn rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            None => (0, -1),
            Some(node) => {
                let borrow = node.borrow();
                let (left_depth, left_value) = Solution::rec(&borrow.left);
                let (right_depth, right_value) = Solution::rec(&borrow.right);
                if left_depth == 0 && right_depth == 0 {
                    return (1, borrow.val);
                }
                if left_depth >= right_depth {
                    (left_depth + 1, left_value)
                } else {
                    (right_depth + 1, right_value)
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

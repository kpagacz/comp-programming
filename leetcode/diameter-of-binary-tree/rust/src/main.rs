// https://leetcode.com/problems/diameter-of-binary-tree/description/
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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::rec(&root).1
    }

    fn rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match node {
            None => (0, 0),
            Some(node) => {
                let borrow = node.borrow();
                let (left_depth, left_diameter) = Solution::rec(&borrow.left);
                let (right_depth, right_diameter) = Solution::rec(&borrow.right);
                (
                    left_depth.max(right_depth + 1),
                    left_diameter
                        .max(right_diameter)
                        .max(left_depth + right_depth),
                )
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

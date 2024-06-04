// https://leetcode.com/problems/merge-two-binary-trees/description/
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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let mut borrow1 = node1.borrow_mut();
                let mut borrow2 = node2.borrow_mut();

                let left = Solution::merge_trees(borrow1.left.take(), borrow2.left.take());
                let right = Solution::merge_trees(borrow1.right.take(), borrow2.right.take());

                borrow1.left = left;
                borrow1.right = right;
                borrow1.val += borrow2.val;
                drop(borrow1);

                Some(node1)
            }
            (Some(node), None) | (None, Some(node)) => Some(node),
            (None, None) => None,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

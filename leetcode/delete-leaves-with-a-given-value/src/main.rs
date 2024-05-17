// https://leetcode.com/problems/delete-leaves-with-a-given-value/description/
pub struct Solution;

// What did I learn? I can manually call drop on a borrow to end it
// if I need to use the borrowed value.

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
    pub fn remove_leaf_nodes_worse(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        if Solution::mark_for_delete(node.clone(), target) {
            None
        } else {
            Some(node)
        }
    }

    fn mark_for_delete(node: Rc<RefCell<TreeNode>>, target: i32) -> bool {
        let mut borrow = node.borrow_mut();

        let left = borrow.left.take();
        if let Some(node) = left {
            if !Solution::mark_for_delete(node.clone(), target) {
                borrow.left = Some(node);
            }
        }

        let right = borrow.right.take();
        if let Some(node) = right {
            if !Solution::mark_for_delete(node.clone(), target) {
                borrow.right = Some(node);
            }
        }

        borrow.left.is_none() && borrow.right.is_none() && borrow.val == target
    }

    pub fn remove_leaf_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let node = root?;
        let mut borrow = node.borrow_mut();

        if borrow.left.is_some() {
            borrow.left = Solution::remove_leaf_nodes(borrow.left.take(), target);
        }

        if borrow.right.is_some() {
            borrow.right = Solution::remove_leaf_nodes(borrow.right.take(), target);
        }

        if borrow.left.is_none() && borrow.right.is_none() && borrow.val == target {
            None
        } else {
            drop(borrow);
            Some(node)
        }
    }
}

fn main() {
    println!("Hello, world!");
}

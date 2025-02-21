// https://leetcode.com/problems/find-elements-in-a-contaminated-binary-tree/description/

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
struct FindElements {
    root: Option<Rc<RefCell<TreeNode>>>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindElements {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }

    fn find(&self, target: i32) -> bool {
        FindElements::rec(&self.root, target, 0)
    }

    fn rec(node: &Option<Rc<RefCell<TreeNode>>>, target: i32, current: i32) -> bool {
        if let Some(node) = node {
            if target == current {
                return true;
            }
            let borr = node.borrow();
            FindElements::rec(&borr.left, target, 2 * current + 1)
                || FindElements::rec(&borr.right, target, 2 * current + 2)
        } else {
            false
        }
    }
}

/**
 * Your FindElements object will be instantiated and called as such:
 * let obj = FindElements::new(root);
 * let ret_1: bool = obj.find(target);
 */

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/two-sum-iv-input-is-a-bst/description/
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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen = HashSet::new();
        Solution::rec(root, k, &mut seen)
    }

    fn rec(node: Option<Rc<RefCell<TreeNode>>>, target: i32, set: &mut HashSet<i32>) -> bool {
        if let Some(node) = node {
            let mut borrow = node.borrow_mut();
            if set.contains(&(target - borrow.val)) {
                return true;
            } else {
                set.insert(borrow.val);
            }
            Solution::rec(borrow.left.take(), target, set)
                || Solution::rec(borrow.right.take(), target, set)
        } else {
            false
        }
    }
}

fn main() {
    println!("Hello, world!");
}

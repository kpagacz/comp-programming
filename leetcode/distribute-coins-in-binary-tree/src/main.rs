// https://leetcode.com/problems/distribute-coins-in-binary-tree/description/
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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::rec(&root).1
    }

    fn rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(ref node) = node {
            let borrow = node.borrow();
            let l = Solution::rec(&borrow.left);
            let r = Solution::rec(&borrow.right);

            let coins = l.0 + r.0 - 1 + borrow.val;
            (coins, l.1 + r.1 + coins.abs())
        } else {
            (0, 0)
        }
    }
}

fn main() {
    println!("Hello, world!");
}

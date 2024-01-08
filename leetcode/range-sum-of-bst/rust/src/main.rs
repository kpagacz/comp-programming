// https://leetcode.com/problems/range-sum-of-bst/
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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(root) = root {
            let borrow = root.borrow();
            match (borrow.val >= low, borrow.val <= high) {
                (true, true) => {
                    borrow.val
                        + Solution::range_sum_bst(borrow.left.clone(), low, high)
                        + Solution::range_sum_bst(borrow.right.clone(), low, high)
                }
                (true, false) => Solution::range_sum_bst(borrow.left.clone(), low, high),
                (false, true) => Solution::range_sum_bst(borrow.right.clone(), low, high),
                (false, false) => unreachable!(),
            }
        } else {
            0
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/sum-root-to-leaf-numbers/description/
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
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => Solution::sum_numbers_rec(&node, 0),
        }
    }

    fn sum_numbers_rec(node: &RefCell<TreeNode>, so_far: i32) -> i32 {
        let borrow = node.borrow();

        let answer = so_far * 10 + borrow.val;
        if borrow.left.is_none() && borrow.right.is_none() {
            return answer;
        }
        let mut from_children = 0;
        if let Some(child) = &borrow.left {
            from_children += Solution::sum_numbers_rec(child.as_ref(), answer);
        }
        if let Some(child) = &borrow.right {
            from_children += Solution::sum_numbers_rec(child.as_ref(), answer);
        }

        from_children
    }
}

fn main() {
    println!("Hello, world!");
}

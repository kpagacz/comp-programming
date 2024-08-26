// https://leetcode.com/problems/binary-tree-postorder-traversal/description/
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn rec(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(node) = node {
                let mut borrow = node.borrow_mut();
                rec(borrow.left.take(), res);
                rec(borrow.right.take(), res);
                res.push(borrow.val);
            }
        }

        let mut res = vec![];
        rec(root, &mut res);
        res
    }

    pub fn postorder_traversal_iter(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut stack = Vec::from_iter(root);
        let mut result = vec![];

        while let Some(node) = stack.pop() {
            if let Some(left) = node.borrow().left.clone() {
                stack.push(left);
            }
            if let Some(right) = node.borrow().right.clone() {
                stack.push(right);
            }
            result.push(node.borrow().val);
        }

        result.into_iter().rev().collect()
    }
}

fn main() {
    println!("Hello, world!");
}

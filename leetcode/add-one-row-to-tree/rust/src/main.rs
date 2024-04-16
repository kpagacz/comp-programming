// https://leetcode.com/problems/add-one-row-to-tree/description/
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
    pub fn add_one_row(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut new_root = TreeNode::new(val);
            new_root.left = root.take();
            return Some(Rc::new(RefCell::new(new_root)));
        } else {
            Solution::add_one_row_rec(root.as_ref().unwrap(), val, depth, 1);
        }

        root
    }

    fn add_one_row_rec(node: &RefCell<TreeNode>, val: i32, target_depth: i32, current_depth: i32) {
        if target_depth - 1 == current_depth {
            let mut borrow = node.borrow_mut();
            let mut new_left = TreeNode::new(val);
            let mut new_right = TreeNode::new(val);
            new_left.left = borrow.left.take();
            new_right.right = borrow.right.take();
            borrow.left = Some(Rc::new(RefCell::new(new_left)));
            borrow.right = Some(Rc::new(RefCell::new(new_right)));
        } else {
            let borrow = node.borrow();
            if let Some(child) = &borrow.left {
                Solution::add_one_row_rec(child, val, target_depth, current_depth + 1);
            }
            if let Some(child) = &borrow.right {
                Solution::add_one_row_rec(child, val, target_depth, current_depth + 1);
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

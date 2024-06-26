// https://leetcode.com/problems/balance-a-binary-search-tree/description/
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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = Vec::new();
        Self::in_order(&root, &mut vals);
        Self::construct_bst(&vals[..])
    }

    fn in_order(node: &Option<Rc<RefCell<TreeNode>>>, arr: &mut Vec<i32>) {
        if let Some(node) = node {
            let borrow = node.borrow();
            Self::in_order(&borrow.left, arr);
            arr.push(borrow.val);
            Self::in_order(&borrow.right, arr);
        }
    }

    fn construct_bst(vals: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() {
            None
        } else {
            let middle = vals.len() / 2;
            let mut node = TreeNode::new(vals[middle]);
            node.left = Self::construct_bst(&vals[..middle]);
            node.right = Self::construct_bst(&vals[middle + 1..]);
            Some(Rc::new(RefCell::new(node)))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

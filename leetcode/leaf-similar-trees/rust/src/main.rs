// https://leetcode.com/problems/leaf-similar-trees/description/
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
    pub fn leaf_similar(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut vals1 = vec![];
        let mut vals2 = vec![];
        Solution::preorder(root1, &mut vals1);
        Solution::preorder(root2, &mut vals2);
        vals1 == vals2
    }

    fn preorder(node: Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = node {
            let borrow = node.borrow();
            if borrow.left.is_none() && borrow.right.is_none() {
                vals.push(borrow.val);
            }
            Solution::preorder(borrow.left.clone(), vals);
            Solution::preorder(borrow.right.clone(), vals);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

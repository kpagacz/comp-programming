// https://leetcode.com/problems/construct-string-from-binary-tree/
pub struct Solution {}

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
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        match root {
            Some(node) => {
                let node_ref = node.borrow();
                let left = Self::tree2str(node_ref.left.clone());
                let right = Self::tree2str(node_ref.right.clone());
                match (left.is_empty(), right.is_empty()) {
                    (true, true) => node_ref.val.to_string(),
                    (false, true) => format!("{}({})", node_ref.val, left),
                    (_, _) => format!("{}({})({})", node_ref.val, left, right),
                }
            }
            None => String::from(""),
        }
    }
}
fn main() {
    println!("Hello, world!");
}

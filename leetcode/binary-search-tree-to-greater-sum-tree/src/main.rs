// https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/description/
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
    pub fn bst_to_gst(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut running_sum = 0;
        Self::rec(&mut root, &mut running_sum);
        root
    }

    fn rec(node: &mut Option<Rc<RefCell<TreeNode>>>, running_sum: &mut i32) {
        if let Some(node) = node {
            let mut mut_borrow = node.borrow_mut();

            Self::rec(&mut mut_borrow.right, running_sum);
            mut_borrow.val += *running_sum;
            *running_sum = mut_borrow.val;
            Self::rec(&mut mut_borrow.left, running_sum);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

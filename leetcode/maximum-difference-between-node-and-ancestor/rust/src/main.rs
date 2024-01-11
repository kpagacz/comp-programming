// https://leetcode.com/problems/maximum-difference-between-node-and-ancestor/
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
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::rec(root).0
    }

    fn rec(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(node) = node {
            let borrow = node.borrow();
            if borrow.left.is_none() && borrow.right.is_none() {
                return (i32::MIN, borrow.val, borrow.val);
            }

            let (mut max_v, mut min_leaf, mut max_leaf) = (i32::MIN, i32::MAX, i32::MIN);
            let left_child = Solution::rec(borrow.left.clone());
            max_v = max_v.max(left_child.0);
            min_leaf = min_leaf.min(left_child.1);
            max_leaf = max_leaf.max(left_child.2);

            let right_child = Solution::rec(borrow.right.clone());
            max_v = max_v.max(right_child.0);
            min_leaf = min_leaf.min(right_child.1);
            max_leaf = max_leaf.max(right_child.2);

            max_v = max_v
                .max((borrow.val - min_leaf).abs())
                .max((borrow.val - max_leaf).abs());
            min_leaf = min_leaf.min(borrow.val);
            max_leaf = max_leaf.max(borrow.val);
            (max_v, min_leaf, max_leaf)
        } else {
            (0, 0, 0)
        }
    }
}

fn main() {
    println!("Hello, world!");
}

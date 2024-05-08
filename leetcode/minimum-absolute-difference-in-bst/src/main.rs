// https://leetcode.com/problems/minimum-absolute-difference-in-bst/description/
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
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut nums = vec![];

        fn preorder(root: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = root {
                let mut borrow = node.borrow_mut();
                preorder(borrow.left.take(), nums);
                nums.push(borrow.val);
                preorder(borrow.right.take(), nums);
            }
        }
        preorder(root, &mut nums);

        nums.sort_unstable();
        nums.windows(2)
            .map(|window| (window[1] - window[0]).abs())
            .min()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}

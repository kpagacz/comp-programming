// https://leetcode.com/problems/evaluate-boolean-binary-tree/description/
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
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::eval(root.unwrap())
    }

    fn eval(node: Rc<RefCell<TreeNode>>) -> bool {
        let mut borrow = node.borrow_mut();

        match borrow.val {
            0 => false,
            1 => true,
            2 => {
                Solution::eval(borrow.left.take().unwrap())
                    || Solution::eval(borrow.right.take().unwrap())
            }
            3 => {
                Solution::eval(borrow.left.take().unwrap())
                    && Solution::eval(borrow.right.take().unwrap())
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

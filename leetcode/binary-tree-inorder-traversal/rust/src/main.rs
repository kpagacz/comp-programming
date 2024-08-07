// https://leetcode.com/problems/binary-tree-inorder-traversal/description/
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let node = node.borrow();
                let left = Self::inorder_traversal(node.left.clone());
                let right = Self::inorder_traversal(node.right.clone());
                left.into_iter()
                    .chain(std::iter::once(node.val))
                    .chain(right)
                    .collect()
            }
            None => vec![],
        }
    }

    pub fn inorder_traversal_it(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            Some(node) => {
                let mut stack = vec![node];
                let mut answer = vec![];
                while let Some(top) = stack.pop() {
                    if top.borrow().left.is_none() && top.borrow().right.is_none() {
                        answer.push(top.borrow().val);
                    } else {
                        let left_child = top.borrow_mut().left.take();
                        let right_child = top.borrow_mut().right.take();
                        if let Some(right_child) = right_child {
                            stack.push(right_child);
                        }
                        stack.push(top);
                        if let Some(left_child) = left_child {
                            stack.push(left_child);
                        }
                    }
                }
                answer
            }
            None => vec![],
        }
    }
}
fn main() {
    println!("Hello, world!");
}

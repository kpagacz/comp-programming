// https://leetcode.com/problems/smallest-string-starting-from-leaf/description/
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
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut chars = vec![];
        let mut smallest = (255 as char).to_string();

        if let Some(root) = root {
            Solution::smallest_from_leaf_rec(root, &mut smallest, &mut chars);
        }

        smallest
    }

    fn smallest_from_leaf_rec(
        node: Rc<RefCell<TreeNode>>,
        smallest: &mut String,
        chars: &mut Vec<char>,
    ) {
        {
            let borrow = node.borrow();
            let c = (b'a' + borrow.val as u8) as char;
            chars.push(c);
        }
        match Rc::try_unwrap(node).unwrap().into_inner() {
            TreeNode {
                left: Some(left),
                right: Some(right),
                ..
            } => {
                Solution::smallest_from_leaf_rec(left, smallest, chars);
                Solution::smallest_from_leaf_rec(right, smallest, chars);
            }
            TreeNode {
                left: Some(child),
                right: None,
                ..
            }
            | TreeNode {
                left: None,
                right: Some(child),
                ..
            } => {
                Solution::smallest_from_leaf_rec(child, smallest, chars);
            }
            TreeNode {
                left: None,
                right: None,
                ..
            } => {
                let s = String::from_iter(chars.iter().rev());
                *smallest = s.min(smallest.clone());
            }
        }
        chars.pop();
    }
}

fn main() {
    println!("Hello, world!");
}

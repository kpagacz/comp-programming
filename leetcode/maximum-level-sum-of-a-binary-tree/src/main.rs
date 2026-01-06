// https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/description/?envType=daily-question&envId=2026-01-06
struct Solution;

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
#[allow(dead_code)]
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let levels_iterator = std::iter::successors(Some(vec![root.unwrap()]), |nodes| {
            let mut next_nodes = vec![];
            for node in nodes {
                if let Some(child) = node.borrow_mut().left.take() {
                    next_nodes.push(child);
                }
                if let Some(child) = node.borrow_mut().right.take() {
                    next_nodes.push(child);
                }
            }
            if next_nodes.is_empty() {
                None
            } else {
                Some(next_nodes)
            }
        });
        levels_iterator
            .into_iter()
            .enumerate()
            .map(|(level, nodes)| {
                (
                    nodes.iter().map(|node| node.borrow().val).sum::<i32>(),
                    std::cmp::Reverse(level + 1),
                )
            })
            .max()
            .unwrap()
            .1
            .0 as _
    }
}

fn main() {
    println!("Hello, world!");
}

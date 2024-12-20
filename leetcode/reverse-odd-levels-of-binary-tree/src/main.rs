// https://leetcode.com/problems/reverse-odd-levels-of-binary-tree/
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
    pub fn reverse_odd_levels(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        use std::iter::successors;

        successors(Some((vec![root.clone().unwrap()], 0)), |(nodes, level)| {
            let level = level + 1;
            if nodes[0].borrow().left.is_some() {
                let children = nodes
                    .iter()
                    .flat_map(|node| {
                        [
                            node.borrow().left.clone().unwrap(),
                            node.borrow().right.clone().unwrap(),
                        ]
                    })
                    .collect::<Vec<_>>();
                Some((children, level))
            } else {
                None
            }
        })
        .filter(|(_, level)| level % 2 != 0)
        .map(|(nodes, _)| nodes)
        .for_each(|nodes| {
            let n = nodes.len();
            for i in 0..n / 2 {
                let left = nodes[i].borrow().val;
                let right = nodes[n - 1 - i].borrow().val;

                nodes[i].borrow_mut().val = right;
                nodes[n - 1 - i].borrow_mut().val = left;
            }
        });

        root
    }
}

fn main() {
    println!("Hello, world!");
}

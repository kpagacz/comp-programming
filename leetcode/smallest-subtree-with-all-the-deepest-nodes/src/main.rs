// https://leetcode.com/problems/smallest-subtree-with-all-the-deepest-nodes/description/?envType=daily-question&envId=2026-01-09
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
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, u32) {
            if let Some(node) = node {
                let borrow = node.borrow();
                let left_child_res = rec(&borrow.left);
                let right_child_res = rec(&borrow.right);
                match (left_child_res, right_child_res) {
                    ((None, _), (None, _)) => (Some(Rc::clone(node)), 1),
                    ((Some(root), height), (None, _)) | ((None, _), (Some(root), height)) => {
                        (Some(root), height + 1)
                    }
                    ((Some(first), first_height), (Some(second), second_height)) => {
                        if first_height > second_height {
                            (Some(first), first_height + 1)
                        } else if first_height < second_height {
                            (Some(second), second_height + 1)
                        } else {
                            (Some(Rc::clone(node)), first_height + 1)
                        }
                    }
                }
            } else {
                (None, 0)
            }
        }

        rec(&root).0
    }
}

fn main() {
    println!("Hello, world!");
}

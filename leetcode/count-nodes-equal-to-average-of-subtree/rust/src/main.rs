// https://leetcode.com/problems/count-nodes-equal-to-average-of-subtree/
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
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            // sum, node count, nodes that have the property
            match node {
                Some(node) => {
                    let left = rec(&node.borrow().left);
                    let right = rec(&node.borrow().right);

                    let sum = left.0 + right.0 + node.borrow().val;
                    let count = left.1 + right.1 + 1;
                    let prop = if sum / count == node.borrow().val {
                        left.2 + right.2 + 1
                    } else {
                        left.2 + right.2
                    };

                    (sum, count, prop)
                }
                None => (0, 0, 0),
            }
        }

        rec(&root).2
    }
}
fn main() {
    println!("Hello, world!");
}

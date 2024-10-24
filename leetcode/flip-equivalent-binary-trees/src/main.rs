// https://leetcode.com/problems/flip-equivalent-binary-trees/description/
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
    pub fn flip_equiv(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => true,
            (Some(_), None) | (None, Some(_)) => false,
            (Some(root1), Some(root2)) => {
                let mut root1 = root1.borrow_mut();
                let mut root2 = root2.borrow_mut();

                if root1.val != root2.val {
                    return false;
                }

                let (root1lc, root1rc) = (
                    root1
                        .left
                        .as_ref()
                        .map(|node| node.borrow().val)
                        .unwrap_or(-1),
                    root1
                        .right
                        .as_ref()
                        .map(|node| node.borrow().val)
                        .unwrap_or(-1),
                );

                let (root2lc, root2rc) = (
                    root2
                        .left
                        .as_ref()
                        .map(|node| node.borrow().val)
                        .unwrap_or(-1),
                    root2
                        .right
                        .as_ref()
                        .map(|node| node.borrow().val)
                        .unwrap_or(-1),
                );

                match (
                    root1lc == root2lc && root1rc == root2rc,
                    root1lc == root2rc && root1rc == root2lc,
                ) {
                    (true, false) => {
                        Solution::flip_equiv(root1.left.take(), root2.left.take())
                            && Solution::flip_equiv(root1.right.take(), root2.right.take())
                    }
                    (false, true) => {
                        Solution::flip_equiv(root1.left.take(), root2.right.take())
                            && Solution::flip_equiv(root1.right.take(), root2.left.take())
                    }
                    (true, true) if root1lc == -1 && root1rc == -1 => true,
                    _ => false,
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

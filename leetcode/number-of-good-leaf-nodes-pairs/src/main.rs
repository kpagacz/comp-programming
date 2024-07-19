// https://leetcode.com/problems/number-of-good-leaf-nodes-pairs/description/?envType=daily-question&envId=2024-07-18
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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut answer = 0;

        Self::rec(root, &mut answer, distance);

        answer
    }

    fn rec(node: Option<Rc<RefCell<TreeNode>>>, pairs: &mut i32, limit: i32) -> Option<Vec<i32>> {
        let node = node?;
        let mut borrow = node.borrow_mut();

        let to_left_leaves = Self::rec(borrow.left.take(), pairs, limit);
        let to_right_leaves = Self::rec(borrow.right.take(), pairs, limit);

        let mut to_leaves = match (to_left_leaves, to_right_leaves) {
            (None, None) => vec![1],
            (Some(mut to_left_leaves), Some(to_right_leaves)) => {
                for to_left in &to_left_leaves {
                    for to_right in &to_right_leaves {
                        if *to_left + *to_right <= limit {
                            *pairs += 1;
                        }
                    }
                }

                to_left_leaves.extend(to_right_leaves);
                to_left_leaves
            }
            (Some(distances), None) | (None, Some(distances)) => distances,
        };

        to_leaves.iter_mut().for_each(|distance| *distance += 1);
        Some(to_leaves)
    }
}

fn main() {
    println!("Hello, world!");
}

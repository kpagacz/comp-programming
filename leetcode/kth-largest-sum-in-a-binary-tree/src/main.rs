// https://leetcode.com/problems/kth-largest-sum-in-a-binary-tree/description/
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
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        fn rec_sum_levels(
            sums: &mut [i64],
            level: usize,
            node: Option<Rc<RefCell<TreeNode>>>,
        ) -> usize {
            if let Some(node) = node {
                let mut borr = node.borrow_mut();
                sums[level] += borr.val as i64;
                rec_sum_levels(sums, level + 1, borr.left.take()).max(rec_sum_levels(
                    sums,
                    level + 1,
                    borr.right.take(),
                )) + 1
            } else {
                0
            }
        }

        let mut sums = vec![0; 100_000];
        let max_level = rec_sum_levels(&mut sums, 0, root);

        while sums.len() > max_level {
            sums.pop();
        }

        sums.sort_unstable();

        sums.into_iter().rev().nth(k as usize - 1).unwrap_or(-1)
    }
}

fn main() {
    println!("Hello, world!");
}

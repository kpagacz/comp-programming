// https://leetcode.com/problems/cousins-in-binary-tree-ii/description/
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
    pub fn replace_value_in_tree(
        mut root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut sums = Vec::with_capacity(100_000);

        fn rec_count_sums(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, sums: &mut Vec<i32>) {
            if let Some(node) = node {
                let borrow = node.borrow();
                while sums.len() <= level {
                    sums.push(0);
                }

                sums[level] += borrow.val;
                rec_count_sums(&borrow.left, level + 1, sums);
                rec_count_sums(&borrow.right, level + 1, sums);
                drop(borrow);
            }
        }
        rec_count_sums(&root, 1, &mut sums);
        println!("{sums:?}");

        fn rec_replace(
            node: &mut Option<Rc<RefCell<TreeNode>>>,
            level: usize,
            sums: &[i32],
            sibling_value: i32,
        ) {
            if let Some(node) = node {
                let mut node = node.borrow_mut();
                let left_child_value = node
                    .left
                    .as_ref()
                    .map(|left_child| left_child.borrow().val)
                    .unwrap_or(0);
                let right_child_value = node
                    .right
                    .as_ref()
                    .map(|right_child| right_child.borrow().val)
                    .unwrap_or(0);

                node.val = sums[level] - sibling_value - node.val;
                rec_replace(&mut node.left, level + 1, sums, right_child_value);
                rec_replace(&mut node.right, level + 1, sums, left_child_value);
                drop(node);
            }
        }
        rec_replace(&mut root, 1, &sums, 0);

        root
    }
}

fn main() {
    println!("Hello, world!");
}

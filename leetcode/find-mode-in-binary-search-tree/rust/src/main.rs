// https://leetcode.com/problems/find-mode-in-binary-search-tree/description/
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
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        use std::collections::HashMap;
        fn count_values(node: &Option<Rc<RefCell<TreeNode>>>, mem: &mut HashMap<i32, i32>) {
            if let Some(node) = node {
                *mem.entry(node.borrow().val).or_insert(0) += 1;
                count_values(&node.borrow().left, mem);
                count_values(&node.borrow().right, mem);
            }
        }
        let mut mem = HashMap::new();
        count_values(&root, &mut mem);

        let max_count = mem.values().max().unwrap();
        mem.iter()
            .filter_map(|(num, count)| if count == max_count { Some(*num) } else { None })
            .collect()
    }

    pub fn find_mode_no_n_memory(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn count_while_traversing(
            node: &Option<Rc<RefCell<TreeNode>>>,
            last_num_in_order: &mut i32,
            num_count: &mut i32,
            max_count: &mut i32,
            modes: &mut Vec<i32>,
        ) {
            if let Some(node) = node {
                count_while_traversing(
                    &node.borrow().left,
                    last_num_in_order,
                    num_count,
                    max_count,
                    modes,
                );
                if node.borrow().val == *last_num_in_order {
                    *num_count += 1;
                } else {
                    *num_count = 1;
                    *last_num_in_order = node.borrow().val;
                }
                if num_count == max_count {
                    modes.push(node.borrow().val);
                } else if num_count > max_count {
                    *modes = vec![node.borrow().val];
                    *max_count = *num_count;
                }

                count_while_traversing(
                    &node.borrow().right,
                    last_num_in_order,
                    num_count,
                    max_count,
                    modes,
                );
            }
        }
        let mut modes = vec![];
        let mut last_num_in_order = -1;
        let mut num_count = 0;
        let mut max_count = 0;
        count_while_traversing(
            &root,
            &mut last_num_in_order,
            &mut num_count,
            &mut max_count,
            &mut modes,
        );
        modes
    }
}
fn main() {
    println!("Hello, world!");
}

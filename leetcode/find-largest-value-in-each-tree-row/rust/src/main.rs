// https://leetcode.com/problems/find-largest-value-in-each-tree-row/description/
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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, maxes: &mut Vec<i32>) {
            match node {
                Some(node) => {
                    if level > maxes.len() {
                        maxes.push(node.borrow().val);
                    } else {
                        maxes[level] = maxes[level].max(node.borrow().val);
                    }
                    dfs(&node.borrow().left, level + 1, maxes);
                    dfs(&node.borrow().right, level + 1, maxes);
                }
                None => {}
            }
        }
        let mut maxes = vec![];
        dfs(&root, 1, &mut maxes);
        maxes
    }
}
fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/minimum-number-of-operations-to-sort-a-binary-tree-by-level/description/
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
    pub fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        use std::iter::successors;

        let nodes = successors(Some(vec![root.unwrap()]), |previous| {
            let mut new_level = vec![];

            for node in previous {
                let mut borrow = node.borrow_mut();
                for child in [borrow.left.take(), borrow.right.take()]
                    .into_iter()
                    .flatten()
                {
                    new_level.push(child);
                }
            }

            if new_level.is_empty() {
                None
            } else {
                Some(new_level)
            }
        });

        nodes.map(Solution::count_swaps).sum()
    }

    fn count_swaps(mut nums: Vec<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::HashMap;

        let mut answer = 0;
        let mut map = nums
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (pos, node)| {
                map.insert(node.borrow().val, pos);
                map
            });
        let mut sorted: Vec<_> = nums.iter().map(|node| node.borrow().val).collect();
        sorted.sort_unstable();

        for i in 0..sorted.len() {
            let original = nums[i].borrow().val;
            let sorted = sorted[i];

            if original != sorted {
                answer += 1;

                let target = *map.get(&sorted).unwrap();
                nums.swap(i, target);
                map.insert(original, target);
                map.insert(sorted, i);
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

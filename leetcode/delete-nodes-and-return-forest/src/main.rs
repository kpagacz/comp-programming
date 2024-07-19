// https://leetcode.com/problems/delete-nodes-and-return-forest/?envType=daily-question&envId=2024-07-17
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
use std::collections::HashSet;
use std::rc::Rc;
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut answer = vec![];

        let to_delete = HashSet::from_iter(to_delete);
        if root.is_some() {
            let val = root.as_ref().unwrap().borrow().val;
            if !to_delete.contains(&val) {
                answer.push(root.clone());
            }
        }
        answer.extend(Self::rec(root, &to_delete));

        answer
    }

    fn rec(
        node: Option<Rc<RefCell<TreeNode>>>,
        to_delete: &HashSet<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if let Some(node) = node {
            let mut borrow = node.borrow_mut();

            let mut roots = vec![];

            if to_delete.contains(&borrow.val) {
                if borrow.right.is_some()
                    && !to_delete.contains(&borrow.right.as_ref().unwrap().borrow().val)
                {
                    roots.push(borrow.right.clone());
                }
                if borrow.left.is_some()
                    && !to_delete.contains(&borrow.left.as_ref().unwrap().borrow().val)
                {
                    roots.push(borrow.left.clone());
                }
            }

            if borrow.left.is_some()
                && to_delete.contains(&borrow.left.as_ref().unwrap().borrow().val)
            {
                roots.extend(Self::rec(borrow.left.take(), to_delete));
            } else {
                roots.extend(Self::rec(borrow.left.clone(), to_delete));
            }

            if borrow.right.is_some()
                && to_delete.contains(&borrow.right.as_ref().unwrap().borrow().val)
            {
                roots.extend(Self::rec(borrow.right.take(), to_delete));
            } else {
                roots.extend(Self::rec(borrow.right.clone(), to_delete));
            }

            roots
        } else {
            vec![]
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/linked-list-in-binary-tree/description/
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(ref uw_head) = head {
            let potential_starts = Self::potential_starts(uw_head, root);

            potential_starts
                .iter()
                .any(|potential_start| Self::rec(&head, potential_start))
        } else {
            true
        }
    }

    fn potential_starts(
        head: &ListNode,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if root.is_none() {
            return vec![];
        }

        let mut answer = vec![];
        if root.as_ref().unwrap().borrow().val == head.val {
            answer.push(root.clone());
        }

        let borrow = root.as_ref().unwrap().borrow();
        answer.extend(Self::potential_starts(head, borrow.left.clone()));
        answer.extend(Self::potential_starts(head, borrow.right.clone()));

        answer
    }

    fn rec(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (head, root) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(head), Some(root)) => {
                let borrow = root.borrow();
                if head.val != borrow.val {
                    false
                } else {
                    Self::rec(&head.next, &borrow.left) || Self::rec(&head.next, &borrow.right)
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

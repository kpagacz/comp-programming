// https://leetcode.com/problems/remove-nodes-from-linked-list/description/
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

impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut reversed = Solution::reverse_nodes(head);
        let mut it = match &mut reversed {
            Some(node) => node,
            None => return None,
        };

        while let Some(mut node) = it.next.take() {
            if node.val < it.val {
                it.next = node.next.take();
            } else {
                it.next = Some(node);
                it = it.next.as_mut()?;
            }
        }

        Solution::reverse_nodes(reversed)
    }

    fn reverse_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pred = None;

        while let Some(mut node) = head {
            head = node.next.take();
            node.next = pred;
            pred = Some(node);
        }

        pred
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/merge-nodes-in-between-zeros/description/
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
    pub fn merge_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let mut total = 0;

            let mut next = node.next.take();
            while next.is_some() {
                let mut unwrapped = next.unwrap();
                if unwrapped.val != 0 {
                    total += unwrapped.val;
                } else {
                    next = Self::merge_nodes(Some(unwrapped));
                    break;
                }
                next = unwrapped.next.take();
            }

            if total == 0 {
                None
            } else {
                let mut new_node = ListNode::new(total);
                new_node.next = next;
                Some(Box::new(new_node))
            }
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

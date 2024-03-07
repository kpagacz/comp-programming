// https://leetcode.com/problems/middle-of-the-linked-list/description/
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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while let Some(node) = fast {
            if let Some(after_fast) = &node.as_ref().next {
                slow = &slow.as_ref().unwrap().next;
                fast = &after_fast.next;
            } else {
                break;
            }
        }

        slow.clone()
    }
}

fn main() {
    println!("Hello, world!");
}

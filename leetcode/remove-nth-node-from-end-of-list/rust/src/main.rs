// https://leetcode.com/problems/remove-nth-node-from-end-of-list/
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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut size = 0;
        let mut it = &head;
        while let Some(node) = it {
            size += 1;
            it = &node.next;
        }

        if n == size {
            return head.unwrap().next;
        }
        let mut to_move = size - n - 1;
        let mut it = &mut head;
        while to_move > 0 {
            to_move -= 1;
            it = &mut it.as_mut().unwrap().next;
        }

        it.as_mut().unwrap().next = it.as_mut().unwrap().next.as_mut().unwrap().next.take();

        head
    }
}

fn main() {
    println!("Hello, world!");
}

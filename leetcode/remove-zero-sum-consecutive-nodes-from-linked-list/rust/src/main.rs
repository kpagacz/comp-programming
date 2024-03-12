// https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/
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
    pub fn remove_zero_sum_sublists(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Option::Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head.clone();

        use std::collections::BTreeMap;
        let mut positions = BTreeMap::new();

        let mut curr = &dummy;
        let mut prefix = 0;
        let mut it = 0;

        while let Some(next) = curr {
            prefix += next.val;
            positions.insert(prefix, it);
            it += 1;
            curr = &next.next;
        }

        let mut curr = &mut dummy;
        let mut prefix = 0;
        let mut it = 0;

        loop {
            let node = curr.as_mut()?;
            prefix += node.val;
            if let Some(&end_it) = positions.get(&prefix) {
                while it < end_it {
                    node.next = node.next.as_mut()?.next.take();
                    it += 1;
                }
            }
            if node.next.is_none() {
                break;
            }

            curr = &mut node.next;
            it += 1;
        }

        dummy.unwrap().next
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/merge-in-between-linked-lists/description/
pub struct Solution;

// What did I learn?  I am getting more and more convinced that I should stick to working
// with $mut Box<ListNode> in these kind of problems instead of &mut Option<Box<ListNode>>
// The added layer of indirection seemingly does not bring any benefits and just makes
// the borrow checker frown upon things that should be easy with &mut Box<ListNode>

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
    pub fn merge_in_between(
        list1: Option<Box<ListNode>>,
        a: i32,
        b: i32,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = list1;

        let mut it = &mut dummy;
        for _ in 0..a {
            it = &mut it.as_mut().unwrap().next;
        }

        let mut after_b = &mut it.as_mut().unwrap().next;
        for _ in a..=b {
            after_b = &mut after_b.as_mut().unwrap().next;
        }
        let after_b_taken = after_b.take();

        let mut it = it.as_mut().unwrap().next.insert(list2.unwrap());
        while let Some(ref mut node) = it.next {
            it = node;
        }
        it.next = after_b_taken;

        dummy.unwrap().next
    }
}

fn main() {
    println!("Hello, world!");
}

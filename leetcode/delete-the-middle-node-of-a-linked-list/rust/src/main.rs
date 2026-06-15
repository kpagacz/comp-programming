// https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/description/?envType=daily-question&envId=2026-06-15
#[allow(dead_code)]
struct Solution;

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
    #[allow(dead_code)]
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().and_then(|node| node.next.as_ref()).is_none() {
            return None;
        }

        let mut fast: *const ListNode = head.as_deref().unwrap() as *const ListNode;
        let mut prev: *mut Option<Box<ListNode>> = &mut head;

        unsafe {
            while let Some(next) = (*fast).next.as_deref() {
                prev = &mut (*prev).as_mut().unwrap().next;
                match next.next.as_deref() {
                    Some(n) => fast = n,
                    None => break,
                }
            }
        }

        unsafe {
            let victim = (*prev).take().unwrap();
            *prev = victim.next;
        }

        head
    }
}

fn main() {
    println!("Hello, world!");
}

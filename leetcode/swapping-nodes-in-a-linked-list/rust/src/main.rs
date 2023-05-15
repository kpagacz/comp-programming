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

pub struct Solution {}

impl Solution {
    pub fn swap_nodes(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut a = head.as_mut().unwrap() as *mut Box<ListNode>;
        for _ in 1..k {
            unsafe {
                a = (*a).next.as_mut().unwrap();
            }
        }

        let mut b = head.as_mut().unwrap() as *mut Box<ListNode>;
        let mut it = a;
        unsafe {
            while let Some(ref mut next_node) = (*it).next {
                it = next_node;
                b = (*b).next.as_mut().unwrap();
            }

            std::mem::swap(&mut (*a).val, &mut (*b).val);
        }

        return head;
    }
}

fn main() {
    println!("Hello, world!");
}

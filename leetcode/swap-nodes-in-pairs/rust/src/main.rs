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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut this) => match this.next {
                Some(mut next) => {
                    this.next = Solution::swap_pairs(next.next);
                    next.next = Some(this);
                    Some(next)
                }
                _ => Some(this),
            },
            _ => head,
        }
    }

    pub fn swap_pairs2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut this = &mut head;
        while this.is_some() && this.as_ref().unwrap().next.is_some() {
            let mut next = this.as_mut().unwrap().next.take();
            let nextnext = next.as_mut().unwrap().next.take();

            this.as_mut().unwrap().next = nextnext;
            next.as_mut().unwrap().next = this.take();

            this.replace(next.unwrap());
            this = &mut this.as_mut().unwrap().next.as_mut().unwrap().next;
        }
        head
    }
}

fn main() {
    println!("Hello, world!");
}

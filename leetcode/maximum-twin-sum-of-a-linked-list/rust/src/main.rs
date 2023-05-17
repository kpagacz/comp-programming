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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut answer = 0;
        if head.is_none() {
            return answer;
        }

        let mut slow = &head;
        let mut fast = &head.as_ref().unwrap().next;
        while fast.as_ref().unwrap().next.is_some()
            && fast.as_ref().unwrap().next.as_ref().unwrap().next.is_some()
        {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        let mut stack = vec![];
        while slow.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            stack.push(slow.as_ref().unwrap().val);
        }

        let mut it = &head;
        while !stack.is_empty() {
            answer = std::cmp::max(answer, it.as_ref().unwrap().val + stack.pop().unwrap());
            it = &it.as_ref().unwrap().next;
        }

        answer
    }
}

pub struct Solution2 {}
impl Solution2 {
    fn length(mut head: Option<&Box<ListNode>>) -> usize {
        let mut count = 0 as usize;
        while let Some(node) = head.take() {
            count += 1;
            head = node.next.as_ref();
        }
        count
    }

    fn split(mut head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mid = Self::length(head.as_ref()) / 2;
        let mut top = None;
        for _ in 0..mid {
            let mut node = head.take().unwrap();
            head = std::mem::replace(&mut node.next, top.take());
            top = Some(node);
        }

        (top, head)
    }

    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        let mut answer = std::i32::MIN;
        let (mut top, mut bottom) = Self::split(head);

        while let (Some(top_node), Some(bottom_node)) = (top.take(), bottom.take()) {
            answer = std::cmp::max(answer, top_node.val + bottom_node.val);
            top = top_node.next;
            bottom = bottom_node.next;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

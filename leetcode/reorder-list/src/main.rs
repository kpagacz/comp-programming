// https://leetcode.com/problems/reorder-list/description/
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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut count = 0;
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head.take();
        let mut it = &dummy;

        // Get the second half of the list
        while let Some(ref node) = it.next {
            count += 1;
            it = &node;
        }
        let mut middle = &mut dummy.as_mut().next;
        for _ in 0..(count - 1) / 2 {
            middle = &mut middle.as_mut().unwrap().next;
        }
        let mut after_middle = middle.as_mut().unwrap().next.take();

        // Reverse the second half of the list
        let mut pre = None;
        while let Some(mut node) = after_middle {
            let tmp = node.next.take();
            node.next = pre;
            pre = Some(node);
            after_middle = tmp;
        }
        // pre is now the head of the reversed second half of the list

        let mut first_half_it = dummy.as_mut().next.take();
        let mut second_half_it = pre;
        let mut insertion_point = &mut dummy.next;

        if second_half_it.is_none() {
            *head = first_half_it;
            return;
        }

        while let (Some(mut first_half), Some(mut second_half)) = (first_half_it, second_half_it) {
            first_half_it = first_half.next.take();
            second_half_it = second_half.next.take();
            first_half.next = Some(second_half);
            let _ = insertion_point.insert(first_half);
            insertion_point = &mut insertion_point
                .as_mut()
                .unwrap()
                .next
                .as_mut()
                .unwrap()
                .next;
            if second_half_it.is_none() {
                if let Some(node) = first_half_it {
                    let _ = insertion_point.insert(node);
                }
                break;
            }
        }

        *head = dummy.next.take();
    }
}

fn main() {
    println!("Hello, world!");
}

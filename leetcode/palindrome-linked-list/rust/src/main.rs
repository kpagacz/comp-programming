// https://leetcode.com/problems/palindrome-linked-list/description/
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
    pub fn is_palindrome_on_space(head: Option<Box<ListNode>>) -> bool {
        let mut num = vec![];

        if head.is_none() {
            return true;
        }

        let mut it = &head.unwrap();

        loop {
            num.push(char::from_digit(it.val as u32, 10).unwrap());
            if it.next.is_some() {
                it = &it.next.as_ref().unwrap();
            } else {
                break;
            }
        }

        for i in 0..(num.len() / 2) {
            if num[i] != num[num.len() - 1 - i] {
                return false;
            }
        }

        true
    }

    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        let mut count = 0;
        let mut it = &head;

        while let Some(ref node) = it {
            count += 1;
            it = &node.next;
        }

        let mut after_middle = &mut head;
        for _ in 0..(count - 1) / 2 {
            after_middle = &mut after_middle.as_mut().unwrap().next;
        }

        let after_middle = after_middle.as_mut().unwrap().next.take();

        let second_half = Solution::reverse_list(after_middle);

        let mut first_it = &head;
        let mut second_it = &second_half;

        while let (Some(first_half), Some(second_half)) = (first_it, second_it) {
            if first_half.val != second_half.val {
                return false;
            }

            first_it = &first_half.next;
            second_it = &second_half.next;
        }

        true
    }

    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;

        while let Some(mut node) = head {
            let tmp = node.next.take();
            node.next = pre;
            pre = Some(node);
            head = tmp;
        }

        pre
    }
}

fn main() {
    println!("Hello, world!");
}

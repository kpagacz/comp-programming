// https://leetcode.com/problems/double-a-number-represented-as-a-linked-list/description/
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
    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut num = vec![];

        let mut it = head;

        loop {
            if it.is_some() {
                let node = it.unwrap();
                num.push(node.val);
                it = node.next;
            } else {
                break;
            }
        }

        let doubled = Solution::double_num(num);

        let mut head = Box::new(ListNode::new(doubled[0]));
        let mut it = &mut head;

        for &digit in &doubled[1..] {
            let new = Box::new(ListNode::new(digit));
            it.next = Some(new);
            it = it.next.as_mut().unwrap();
        }

        Some(head)
    }

    fn double_num(num: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::with_capacity(num.len());
        let mut carry = 0;
        for digit in num.iter().rev() {
            let res = 2 * digit + carry;
            carry = res / 10;
            answer.push(res % 10);
        }

        if carry != 0 {
            answer.push(carry);
        }
        answer.into_iter().rev().collect()
    }
}

fn main() {
    println!("Hello, world!");
}

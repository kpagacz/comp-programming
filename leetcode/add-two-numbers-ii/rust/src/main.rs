// // https://leetcode.com/problems/add-two-numbers-ii/

pub struct Solution {}

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
    // Runtime
    // Details
    // -ms
    // Beats 100.00%of users with Rust
    // Memory
    // Details
    // 2.05mb
    // Beats 100.00%of users with Rust
    pub fn add_two_numbers(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut first = vec![];
        let mut second = vec![];
        while let Some(node) = l1 {
            l1 = node.next;
            first.push(node.val);
        }
        while let Some(node) = l2 {
            l2 = node.next;
            second.push(node.val);
        }

        let mut reversed_answer = vec![];
        let mut memory = 0;
        loop {
            let sum = match (first.pop(), second.pop()) {
                (None, None) => break,
                (None, Some(digit)) | (Some(digit), None) => memory + digit,
                (Some(digit1), Some(digit2)) => memory + digit1 + digit2,
            };
            reversed_answer.push(sum % 10);
            memory = sum / 10;
        }
        if memory != 0 {
            reversed_answer.push(memory);
        }
        reversed_answer.reverse();

        let mut answer = None;
        let mut insert_point = &mut answer;
        for digit in reversed_answer {
            let inserted = insert_point.insert(Box::new(ListNode::new(digit)));
            insert_point = &mut inserted.next;
        }

        answer
    }

    // Runtime
    // Details
    // 9ms
    // Beats 37.50%of users with Rust
    // Memory
    // Details
    // 2.27mb
    // Beats 50.00%of users with Rust
    // Interestingly this has worse runtime and memory
    // probably because answer needs to be in memory twice in each iteration
    // of the main loop
    pub fn add_two_numbers_shorter_version(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut first = vec![];
        let mut second = vec![];
        while let Some(node) = l1 {
            l1 = node.next;
            first.push(node.val);
        }
        while let Some(node) = l2 {
            l2 = node.next;
            second.push(node.val);
        }

        let mut answer = None;
        let mut memory = 0;
        loop {
            let sum = match (first.pop(), second.pop()) {
                (None, None) => break,
                (None, Some(digit)) | (Some(digit), None) => memory + digit,
                (Some(digit1), Some(digit2)) => memory + digit1 + digit2,
            };
            answer = Some(Box::new(ListNode {
                val: sum % 10,
                next: answer,
            }));
            memory = sum / 10;
        }
        if memory != 0 {
            answer = Some(Box::new(ListNode {
                val: memory,
                next: answer,
            }));
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

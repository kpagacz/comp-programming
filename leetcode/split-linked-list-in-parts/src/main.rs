// https://leetcode.com/problems/split-linked-list-in-parts/description/?envType=daily-question&envId=2024-09-08
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
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut length = 0;
        let parts = k as usize;
        let mut it = &head;
        while it.is_some() {
            length += 1;
            it = &it.as_ref().unwrap().next;
        }
        let part_length = length / parts;
        let mut surplus = length % parts;

        let mut answer = vec![None; parts];
        let mut it = head;
        (0..answer.len()).for_each(|part| {
            let mut tail = &mut None;
            if part_length > 0 {
                answer[part] = it.take();
                tail = &mut answer[part];

                for _ in 1..part_length {
                    tail = &mut tail.as_mut().unwrap().next;
                }
                it = tail.as_mut().unwrap().next.take();
            }
            if surplus > 0 {
                if tail.is_none() {
                    answer[part] = it.take();
                    it = answer[part].as_mut().unwrap().next.take();
                } else {
                    tail.as_mut().unwrap().next = it.take();
                    it = tail.as_mut().unwrap().next.as_mut().unwrap().next.take();
                }
                surplus -= 1;
            }
        });
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

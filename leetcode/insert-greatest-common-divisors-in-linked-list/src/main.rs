// https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/description/
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
    pub fn insert_greatest_common_divisors(
        mut head: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn insert_gcd(node: Option<&mut Box<ListNode>>) -> Option<&mut Box<ListNode>> {
            if let Some(node) = node {
                if let Some(nnode) = node.next.take() {
                    let mut gcd_node = ListNode::new(Solution::gcd(node.val, nnode.val));
                    gcd_node.next = Some(nnode);
                    node.next = Some(Box::new(gcd_node));
                    node.next.as_mut().unwrap().next.as_mut()
                } else {
                    None
                }
            } else {
                None
            }
        }

        let mut it = head.as_mut();
        while it.is_some() {
            it = insert_gcd(it);
        }
        head
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

fn main() {
    println!("Hello, world!");
}

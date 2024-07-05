// https://leetcode.com/problems/find-the-minimum-and-maximum-number-of-nodes-between-critical-points/description/
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
    pub fn nodes_between_critical_points(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut minimum = i32::MAX;
        let mut maximum = i32::MIN;
        let (mut closest, mut farthest) = (-1, -1);
        let mut previous = None;

        while let Some(mut node) = head {
            if previous.is_none() || node.next.is_none() {
                previous = Some(node.val);
                head = node.next.take();
                continue;
            }
            let previous_val = previous.unwrap();
            let next = node.next.as_ref().unwrap().val;

            if closest != -1 {
                closest += 1;
                farthest += 1;
            }
            if (next > node.val && previous_val > node.val)
                || (next < node.val && previous_val < node.val)
            {
                if closest != -1 {
                    minimum = minimum.min(closest);
                    maximum = maximum.max(farthest);
                }
                if farthest == -1 {
                    farthest = 0;
                }
                closest = 0;
            }
            head = node.next.take();
            previous = Some(node.val);
        }

        if minimum == i32::MAX {
            vec![-1, -1]
        } else {
            vec![minimum, maximum]
        }
    }
}

fn main() {
    println!("Hello, world!");
}

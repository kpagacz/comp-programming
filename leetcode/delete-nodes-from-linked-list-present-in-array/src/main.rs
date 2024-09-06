// https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/description/
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
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums = nums
            .into_iter()
            .fold(vec![false; 100_001], |mut nums, num| {
                nums[num as usize] = true;
                nums
            });

        Self::rec(head, &nums)
    }

    fn rec(head: Option<Box<ListNode>>, nums: &[bool]) -> Option<Box<ListNode>> {
        if let Some(mut head) = head {
            if nums[head.val as usize] {
                Self::rec(head.next, nums)
            } else {
                head.next = Self::rec(head.next, nums);
                Some(head)
            }
        } else {
            None
        }
    }
}

fn main() {
    println!("Hello, world!");
}

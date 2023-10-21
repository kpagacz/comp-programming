// https://leetcode.com/problems/constrained-subsequence-sum/description/
pub struct Solution {}

#[derive(PartialOrd, PartialEq, Eq)]
struct State {
    pub num: i32,
    pub position: usize,
}
impl State {
    fn new(num: i32, position: usize) -> Self {
        Self { num, position }
    }
}
use std::cmp::Ordering;
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.num.cmp(&other.num)
    }
}

impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        use std::collections::VecDeque;
        let mut monotonic_queue = VecDeque::new();
        monotonic_queue.push_back((0, 0)); // (max subsequence sum ending at index i, index i)
        let mut max = i32::MIN;
        nums.iter().enumerate().for_each(|(id, &num)| {
            while id - monotonic_queue.front().unwrap().1 > k {
                monotonic_queue.pop_front();
            }
            let this_id_max = num.max(num + monotonic_queue.front().unwrap().0);
            while !monotonic_queue.is_empty() && this_id_max >= monotonic_queue.back().unwrap().0 {
                monotonic_queue.pop_back();
            }
            monotonic_queue.push_back((this_id_max, id));
            max = max.max(this_id_max);
        });

        max
    }
    pub fn constrained_subset_sum_pqueue(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        use std::collections::BinaryHeap;
        let mut max_heap: BinaryHeap<State> = BinaryHeap::new();

        (0..nums.len()).for_each(|it| {
            while let Some(state) = max_heap.peek() {
                if it - state.position <= k {
                    nums[it] = nums[it].max(state.num + nums[it]);
                    break;
                } else {
                    max_heap.pop();
                }
            }
            max_heap.push(State::new(nums[it], it));
        });

        println!("{nums:?}");
        *nums.iter().max().unwrap()
    }
    pub fn constrained_subset_sum_quadratic(mut nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len()).for_each(|id| {
            nums[id] = (id.saturating_sub(k as usize)..id)
                .map(|previous| nums[id] + nums[previous])
                .chain(std::iter::once(nums[id]))
                .max()
                .unwrap()
        });
        println!("{nums:?}");
        *nums.iter().max().unwrap()
    }
}

fn main() {
    let test_cases = vec![(vec![10, 2, -10, 5, 20], 2)];
    for (arr, k) in test_cases {
        println!("{}", Solution::constrained_subset_sum_quadratic(arr, k));
    }

    let vec = (0..10000).collect::<Vec<i32>>();
    println!("{vec:?}");
}

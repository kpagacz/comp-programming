use std::cmp::Reverse;

// https://leetcode.com/problems/total-cost-to-hire-k-workers/
pub struct Solution {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Candidate {
    cost: i32,
    position: usize,
    start_bin: bool,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    // Runtime35 ms Beats 44.83%
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        let mut iter = costs.iter();
        let mut pqueue = std::collections::BinaryHeap::new();
        for _ in 0..candidates as usize {
            if let Some(&cost) = iter.next() {
                pqueue.push((Reverse(cost), true));
            }
            if let Some(&cost) = iter.next_back() {
                pqueue.push((Reverse(cost), false));
            }
        }

        (0..k)
            .into_iter()
            .map(|_| {
                let (lowest_cost, is_front) = pqueue.pop().unwrap();
                if is_front {
                    if let Some(&cost) = iter.next() {
                        pqueue.push((Reverse(cost), true));
                    }
                } else {
                    if let Some(&cost) = iter.next_back() {
                        pqueue.push((Reverse(cost), false));
                    }
                }
                lowest_cost.0 as i64
            })
            .sum()
    }
    pub fn total_cost_lame(costs: Vec<i32>, mut k: i32, candidates: i32) -> i64 {
        let mut answer = 0_i64;
        let mut pqueue = std::collections::BinaryHeap::new();
        costs
            .iter()
            .take(candidates as usize)
            .enumerate()
            .for_each(|(i, &val)| {
                pqueue.push(Candidate {
                    cost: val,
                    position: i,
                    start_bin: true,
                });
            });
        let mut left_index = candidates as usize - 1;
        for i in (costs.len() - candidates as usize)..costs.len() {
            if i > left_index {
                pqueue.push(Candidate {
                    cost: costs[i],
                    position: i,
                    start_bin: false,
                });
            }
        }
        let mut right_index = costs.len() - candidates as usize;

        while k > 0 {
            let top = pqueue.pop().unwrap();
            // println!("{:?}", top);
            answer += top.cost as i64;
            if top.start_bin {
                if left_index + 1 < right_index {
                    // println!("adding {}", left_index + 1);
                    left_index += 1;
                    pqueue.push(Candidate {
                        cost: costs[left_index],
                        position: left_index,
                        start_bin: true,
                    });
                }
            } else {
                if right_index - 1 > left_index {
                    // println!("adding {}", right_index - 1);
                    right_index -= 1;
                    pqueue.push(Candidate {
                        cost: costs[right_index],
                        position: right_index,
                        start_bin: false,
                    });
                }
            }
            k -= 1;
        }

        answer
    }
}

fn main() {
    let test_cases = vec![(vec![10, 1, 11, 10], 2, 1)];
    for (cost, k, candidates) in test_cases {
        println!("{}", Solution::total_cost(cost, k, candidates));
    }
}

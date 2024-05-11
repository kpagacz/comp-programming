// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/description/
pub struct Solution;

#[derive(PartialEq, Eq)]
struct Fraction(i32, i32);
impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 * other.1).cmp(&(self.1 * other.0))
    }
}
impl PartialOrd for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        use std::collections::BinaryHeap;

        let mut most_cost_efficient = (0..quality.len()).collect::<Vec<_>>();
        most_cost_efficient.sort_unstable_by_key(|&key| Fraction(wage[key], quality[key]));

        let mut hired = BinaryHeap::new();

        let mut total_quality = 0.0;
        let mut max_cost_efficiency = 0.0;
        let mut answer = f64::MAX;

        let k = k as usize;
        for worker in most_cost_efficient {
            let (wage, quality) = (wage[worker] as f64, quality[worker]);

            hired.push((quality, worker));
            total_quality += quality as f64;

            if hired.len() > k {
                let (popped_quality, popped_worker) = hired.pop().unwrap();
                total_quality -= popped_quality as f64;
                if popped_worker != worker {
                    max_cost_efficiency = wage / quality as f64;
                }
            } else {
                max_cost_efficiency = wage / quality as f64;
            }

            if hired.len() == k {
                answer = answer.min(dbg!(total_quality * max_cost_efficiency));
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [
        // (vec![10, 20, 5], vec![70, 50, 30], 2),
        // (vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3),
        (vec![94, 57, 25, 44, 19], vec![121, 494, 348, 382, 262], 4),
    ];
    for (quality, wage, k) in test_cases {
        println!("{}", Solution::mincost_to_hire_workers(quality, wage, k));
    }
}

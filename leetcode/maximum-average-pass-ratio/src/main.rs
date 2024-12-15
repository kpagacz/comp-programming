// https://leetcode.com/problems/maximum-average-pass-ratio/description/
pub struct Solution;

#[derive(PartialEq, Debug)]
struct Float(f64);

impl Eq for Float {}

impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        use std::collections::BinaryHeap;
        let mut pq = BinaryHeap::from_iter(classes.into_iter().map(|class| {
            let new_ratio = (1f64 + class[0] as f64) / (1f64 + class[1] as f64);
            let old_ratio = class[0] as f64 / class[1] as f64;
            (Float(new_ratio - old_ratio), class[0], class[1])
        }));

        while extra_students > 0 {
            let (_, mut pass, mut total) = pq.pop().unwrap();
            pass += 1;
            total += 1;
            extra_students -= 1;
            let old_ratio = pass as f64 / total as f64;
            let new_ratio = (1f64 + pass as f64) / (1f64 + total as f64);

            pq.push((Float(new_ratio - old_ratio), pass, total));
        }
        println!("Final: {pq:?}");

        let mut total_pass_ratio = 0f64;
        let n = pq.len();
        while let Some((_, pass, total)) = pq.pop() {
            total_pass_ratio += pass as f64 / total as f64;
        }

        total_pass_ratio / n as f64
    }
}

fn main() {
    println!("Hello, world!");
}

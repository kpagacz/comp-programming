// https://leetcode.com/problems/most-beautiful-item-for-each-query/description/
pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|item| item[0]);
        let maxes = items
            .iter()
            .scan(0, |acc, item| {
                *acc = (*acc).max(item[1]);
                Some(*acc)
            })
            .collect::<Vec<_>>();

        queries
            .into_iter()
            .map(|query| {
                let pos = items.partition_point(|item| item[0] <= query);
                if pos > 0 {
                    maxes[pos - 1]
                } else {
                    0
                }
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

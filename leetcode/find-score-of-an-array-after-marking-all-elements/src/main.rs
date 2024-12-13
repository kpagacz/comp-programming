// https://leetcode.com/problems/find-score-of-an-array-after-marking-all-elements/description/
pub struct Solution;

impl Solution {
    pub fn find_score(nums: Vec<i32>) -> i64 {
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let mut marked: HashSet<i32> = HashSet::default();
        let n = nums.len();
        let mut heap = BinaryHeap::from_iter(
            nums.into_iter()
                .enumerate()
                .map(|(pos, val)| (-val as i64, -(pos as i32))),
        );

        let mut score = 0;
        while marked.len() != n {
            println!("Marked :{marked:?}");
            let (val, pos) = heap.pop().unwrap();
            if marked.contains(&pos) {
                continue;
            }
            let val = -val;
            score += val;
            for dx in [-1, 0, 1] {
                if dx + pos <= 0 && dx + pos > -(n as i32) {
                    marked.insert(dx + pos);
                }
            }
        }
        score
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/subsets/description/
pub struct Solution;

impl Solution {
    pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        nums.dedup();

        let mut bitmask = 0;
        let max = 1 << nums.len();
        let mut answer = vec![];

        while bitmask < max {
            let subset =
                nums.iter()
                    .rev()
                    .enumerate()
                    .fold(Vec::default(), |mut vec, (pos, &num)| {
                        if (bitmask >> pos) & 1 == 1 {
                            vec.push(num);
                        }
                        vec
                    });
            answer.push(subset);

            bitmask += 1;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

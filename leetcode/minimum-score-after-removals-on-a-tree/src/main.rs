// https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/solutions/3267170/just-a-runnable-solution/?envType=daily-question&envId=2025-07-24
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        fn dfs(
            dp: &mut Vec<i32>,
            last: &mut Vec<i32>,
            nums: &Vec<i32>,
            neighbours: &Vec<Vec<i32>>,
            current: usize,
            previous: i32,
            ids: &mut i32,
        ) -> i32 {
            let mut res = nums[current];
            for j in neighbours[current].iter() {
                if *j != previous {
                    let id = *ids;
                    *ids += 1;
                    dp[id as usize] =
                        dfs(dp, last, nums, neighbours, *j as usize, current as i32, ids);
                    last[id as usize] = *ids;
                    res ^= dp[id as usize];
                }
            }
            res
        }

        let mut dp = vec![0; 1000];
        let mut last = vec![0; 1000];
        let mut ids = 0;
        let mut res = i32::MAX;
        let mut neighbours = vec![vec![]; nums.len()];
        for e in edges.iter() {
            neighbours[e[0] as usize].push(e[1]);
            neighbours[e[1] as usize].push(e[0]);
        }
        let all = dfs(&mut dp, &mut last, &nums, &neighbours, 0, -1, &mut ids);
        for i in 0..edges.len() {
            for j in i + 1..edges.len() {
                let p1 = if j < last[i] as usize {
                    all ^ dp[i]
                } else {
                    all ^ dp[i] ^ dp[j]
                };
                let p2 = if j < last[i] as usize {
                    dp[i] ^ dp[j]
                } else {
                    dp[i]
                };
                let arr = [p1, p2, dp[j]];
                let max = arr.iter().max().unwrap();
                let min = arr.iter().min().unwrap();
                res = res.min(max - min);
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/most-profitable-path-in-a-tree/
pub struct Solution;

impl Solution {
    pub fn most_profitable_path(edges: Vec<Vec<i32>>, mut bob: i32, mut amount: Vec<i32>) -> i32 {
        let n = amount.len();
        let mut neis = vec![vec![]; n];
        for e in edges {
            neis[e[1] as usize].push(e[0]);
            neis[e[0] as usize].push(e[1]);
        }
        let mut parent = vec![0; n];
        let mut dq = vec![0];
        while !dq.is_empty() {
            for n in std::mem::take(&mut dq) {
                for mut j in 0..neis[n as usize].len() {
                    j = neis[n as usize][j] as usize;
                    parent[j] = n;
                    dq.push(j as i32);
                    neis[j].retain(|&x| x != n);
                }
            }
        }
        // It would be much more convenient if given a parent pointer array instead.
        let mut dq = vec![(0, amount[0])];
        let mut ans = i32::MIN;
        while !dq.is_empty() {
            amount[bob as usize] = 0;
            bob = parent[bob as usize];
            for (a, amt) in std::mem::take(&mut dq) {
                for &ch in &neis[a as usize] {
                    if ch == bob {
                        dq.push((ch, amt + amount[ch as usize] / 2));
                    } else {
                        dq.push((ch, amt + amount[ch as usize]));
                    }
                    amount[ch as usize] = 0;
                }
                if neis[a as usize].is_empty() {
                    ans = ans.max(amt);
                }
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

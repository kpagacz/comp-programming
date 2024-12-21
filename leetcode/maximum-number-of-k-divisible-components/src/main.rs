// https://leetcode.com/problems/maximum-number-of-k-divisible-components/description/
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dfs(
        seen: &mut Vec<bool>,
        G: &HashMap<usize, Vec<usize>>,
        values: &mut Vec<i32>,
        k: &i32,
        u: usize,
        ans: &mut i32,
    ) -> i32 {
        seen[u] = true;
        if !G.contains_key(&u) {
            *ans += 1;
            return 0;
        }
        for v in G[&u].iter() {
            if !seen[*v] {
                values[u] += Self::dfs(seen, G, values, k, *v, ans) % k;
            }
        }
        if values[u] % k == 0 {
            *ans += 1;
            return 0;
        }
        values[u]
    }
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        mut values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let mut G: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n as usize);
        for e in edges {
            G.entry(e[0] as usize)
                .or_insert(Vec::new())
                .push(e[1] as usize);
            G.entry(e[1] as usize)
                .or_insert(Vec::new())
                .push(e[0] as usize);
        }
        let mut seen: Vec<bool> = vec![false; n as usize];
        let mut ans: i32 = 0;
        Self::dfs(&mut seen, &G, &mut values, &k, 0, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}

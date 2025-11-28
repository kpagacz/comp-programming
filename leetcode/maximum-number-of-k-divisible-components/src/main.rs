// https://leetcode.com/problems/maximum-number-of-k-divisible-components/description/
struct Solution;

use std::collections::HashMap;
#[allow(dead_code)]
impl Solution {
    fn dfs(
        seen: &mut Vec<bool>,
        g: &HashMap<usize, Vec<usize>>,
        values: &mut Vec<i32>,
        k: &i32,
        u: usize,
        ans: &mut i32,
    ) -> i32 {
        seen[u] = true;
        for v in g[&u].iter() {
            if !seen[*v] {
                values[u] += Self::dfs(seen, g, values, k, *v, ans) % k;
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
        if n == 1 {
            return 1;
        }
        let mut g: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n as usize);
        for e in edges {
            g.entry(e[0] as usize).or_default().push(e[1] as usize);
            g.entry(e[1] as usize).or_default().push(e[0] as usize);
        }
        let mut seen: Vec<bool> = vec![false; n as usize];
        let mut ans: i32 = 0;
        Self::dfs(&mut seen, &g, &mut values, &k, 0, &mut ans);
        ans
    }
}

fn main() {
    let test_cases = vec![
        (
            5,
            vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]],
            vec![1, 8, 1, 4, 4],
            6,
            2,
        ),
        (
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 5],
                vec![2, 6],
            ],
            vec![3, 0, 6, 1, 5, 2, 1],
            3,
            3,
        ),
        (1, vec![], vec![3], 3, 1),
    ];
    for (n, edges, values, k, exp) in test_cases {
        println!(
            "{}  exp: {exp}",
            Solution::max_k_divisible_components(n, edges, values, k)
        );
    }
}

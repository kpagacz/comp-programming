// https://leetcode.com/problems/find-minimum-diameter-after-merging-two-trees/description/
pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn minimum_diameter_after_merge(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> i32 {
        let d1 = Solution::diameter(Solution::to_graph(edges1));
        let d2 = Solution::diameter(Solution::to_graph(edges2));

        d1.max(d2).max((d1 + 1) / 2 + (d2 + 1) / 2 + 1)
    }

    fn to_graph(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        edges
            .into_iter()
            .fold(HashMap::<i32, Vec<i32>>::new(), |mut map, edge| {
                let (first, second) = (edge[0], edge[1]);

                map.entry(first).or_default().push(second);
                map.entry(second).or_default().push(first);

                map
            })
    }

    fn diameter(graph: HashMap<i32, Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        use std::collections::VecDeque;
        let mut diameter = 0;
        let mut start = 0;
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        for _ in 0..2 {
            seen.insert(start);
            queue.push_back(start);

            let mut height = 0;
            while !queue.is_empty() {
                height += 1;
                for _ in 0..queue.len() {
                    let first = queue.pop_front().unwrap();
                    start = first;
                    for &n in graph.get(&first).unwrap_or(&vec![]) {
                        if !seen.contains(&n) {
                            seen.insert(n);
                            queue.push_back(n);
                        }
                    }
                }
            }

            seen.clear();
            diameter = height - 1;
        }

        diameter
    }
}

fn main() {
    println!("Hello, world!");
}

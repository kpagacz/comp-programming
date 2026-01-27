// https://leetcode.com/problems/minimum-cost-path-with-edge-reversals/description/?envType=daily-question&envId=2026-01-27
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;
        type State = (i32, i32); // (cost, current, used reversed edge)
        let mut pq: BinaryHeap<Reverse<State>> = BinaryHeap::new();
        let mut ns: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        for edge in edges {
            let (from, to, weight) = (edge[0], edge[1], edge[2]);
            ns.entry(from)
                .and_modify(|dests| dests.push((to, weight)))
                .or_insert(vec![(to, weight)]);
            ns.entry(to)
                .and_modify(|dests| dests.push((from, 2 * weight)))
                .or_insert(vec![(from, 2 * weight)]);
        }

        let mut visited = vec![false; n as usize];
        pq.push(Reverse((0, 0)));

        while let Some(Reverse((cost, current))) = pq.pop() {
            if current == n - 1 {
                return cost;
            }
            if visited[current as usize] {
                continue;
            } else {
                visited[current as usize] = true;
            }

            ns.get(&current)
                .unwrap_or(&vec![])
                .iter()
                .for_each(|&(to, weight)| {
                    if !visited[to as usize] {
                        pq.push(Reverse((cost + weight, to)));
                    }
                });
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}

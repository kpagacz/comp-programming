// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-i/description/?envType=daily-question&envId=2025-05-28
pub struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        fn count_targets(tree: &[Vec<i32>], working_distance: i32) -> Vec<i32> {
            let mut targets = vec![0; tree.len() + 1];
            if working_distance < 0 {
                return targets;
            }
            use std::collections::HashMap;
            let mut tree_map: HashMap<usize, Vec<usize>> = HashMap::new();
            for edge in tree {
                let (first, second) = (edge[0] as usize, edge[1] as usize);
                tree_map
                    .entry(first)
                    .and_modify(|dests| dests.push(second))
                    .or_insert(vec![second]);
                tree_map
                    .entry(second)
                    .and_modify(|dests| dests.push(first))
                    .or_insert(vec![first]);
            }
            let tree_map = tree_map;
            // println!("tree: {tree_map:?}");
            for origin in 0..targets.len() {
                // println!("origin: {origin}");
                use std::collections::HashSet;
                let mut visited = HashSet::new();
                visited.insert(origin);
                let mut sources = vec![origin];
                let mut working_distance = working_distance;
                while working_distance > 0 {
                    let mut new_sources = vec![];
                    for source in sources {
                        let empty = Vec::new();
                        let ns = tree_map.get(&source).unwrap_or(&empty);
                        for &n in ns {
                            if !visited.contains(&n) {
                                visited.insert(n);
                                new_sources.push(n);
                            }
                        }
                    }
                    // println!("visited: {visited:?} new_sources: {new_sources:?}");
                    sources = new_sources;
                    working_distance -= 1;
                }
                targets[origin] = visited.len() as _;
            }
            targets
        }

        let second_tree_max_targets = count_targets(&edges2, k - 1).into_iter().max().unwrap();
        let mut first_tree_targets = count_targets(&edges1, k);
        first_tree_targets
            .iter_mut()
            .for_each(|val| *val += second_tree_max_targets);
        first_tree_targets
    }
}

fn main() {
    let test_cases = [(
        vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4]],
        vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![2, 7],
            vec![1, 4],
            vec![4, 5],
            vec![4, 6],
        ],
        2,
    )];
    for (edges1, edges2, k) in test_cases {
        println!("{:?}", Solution::max_target_nodes(edges1, edges2, k));
    }
}

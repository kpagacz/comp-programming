// https://leetcode.com/problems/maximize-the-number-of-target-nodes-after-connecting-trees-ii/description/?envType=daily-question&envId=2025-05-29
pub struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::HashMap;

        /// returns (even, odd)
        fn target_nodes(tree: &HashMap<i32, Vec<i32>>) -> (i32, i32, Vec<i32>) {
            fn paint(node: i32, parent: i32, tree: &HashMap<i32, Vec<i32>>, colours: &mut [i32]) {
                for &child in tree.get(&node).unwrap_or(&vec![]) {
                    if child == parent {
                        continue;
                    }
                    colours[child as usize] = colours[node as usize] ^ 1; // flip colour
                    paint(child, node, tree, colours);
                }
            }

            let mut colours = vec![0; tree.len()];
            paint(0, 0, tree, &mut colours);
            let odd = colours.iter().filter(|&&c| c == 1).count() as i32;
            (colours.len() as i32 - odd, odd, colours)
        }

        let second_tree: HashMap<i32, Vec<i32>> =
            edges2.into_iter().fold(HashMap::new(), |mut map, edge| {
                let (first, second) = (edge[0], edge[1]);
                map.entry(first).or_default().push(second);
                map.entry(second).or_default().push(first);
                map
            });
        let (second_even, second_odd, _) = target_nodes(&second_tree);
        let best_from_second = second_even.max(second_odd);

        let first_tree: HashMap<i32, Vec<i32>> =
            edges1.into_iter().fold(HashMap::new(), |mut map, edge| {
                map.entry(edge[0]).or_default().push(edge[1]);
                map.entry(edge[1]).or_default().push(edge[0]);
                map
            });
        let (even, odd, colours) = target_nodes(&first_tree);

        colours
            .into_iter()
            .map(|i| if i % 2 == 0 { even } else { odd } + best_from_second)
            .collect()
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
    )];
    for (edges1, edges2) in test_cases {
        println!("{:?}", Solution::max_target_nodes(edges1, edges2));
    }
}

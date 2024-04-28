// https://leetcode.com/problems/sum-of-distances-in-tree/description/
pub struct Solution;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let edges = edges
            .iter()
            .fold(vec![vec![]; n as usize], |mut edges, edge| {
                let (a, b) = (edge[0] as usize, edge[1] as usize);
                edges[a].push(b);
                edges[b].push(a);
                edges
            });
        let mut distances = vec![0; n as usize];
        let mut subtree_size = vec![0; n as usize];
        Solution::child_distances_and_subtrees(
            usize::MAX,
            0,
            &mut distances,
            &edges,
            &mut subtree_size,
        );
        Solution::add_parent_distances(usize::MAX, 0, 0, &edges, &mut distances, &subtree_size);

        distances
    }

    fn child_distances_and_subtrees(
        from: usize,
        node: usize,
        distances: &mut [i32],
        edges: &[Vec<usize>],
        subtree_size: &mut [i32],
    ) -> (i32, i32) {
        let mut children = edges[node]
            .iter()
            .filter(|&&child| child != from)
            .peekable();
        if children.peek().is_none() {
            distances[node] = 0;
            subtree_size[node] = 1;
            (0, 1)
        } else {
            let children_res = children
                .map(|&child| {
                    Solution::child_distances_and_subtrees(
                        node,
                        child,
                        distances,
                        edges,
                        subtree_size,
                    )
                })
                .fold((0, 0), |answer, res| (answer.0 + res.0, answer.1 + res.1));
            distances[node] = children_res.0 + children_res.1;
            subtree_size[node] = children_res.1 + 1;
            (distances[node], subtree_size[node])
        }
    }

    fn add_parent_distances(
        from: usize,
        node: usize,
        parent_distances: i32,
        edges: &[Vec<usize>],
        distances: &mut [i32],
        subtree_size: &[i32],
    ) {
        distances[node] += parent_distances;

        let children = edges[node].iter().filter(|&&n| n != from);
        children.for_each(|&child| {
            let parent_distances = distances[node] - distances[child] - subtree_size[child]
                + subtree_size[node]
                - subtree_size[child]
                + if from == usize::MAX {
                    subtree_size[node]
                } else {
                    subtree_size[0]
                }
                - subtree_size[node];
            Solution::add_parent_distances(
                node,
                child,
                parent_distances,
                edges,
                distances,
                subtree_size,
            );
        });
    }
}

fn main() {
    let test_cases = vec![
        (
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]],
        ),
        (
            10,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![2, 3],
                vec![2, 4],
                vec![2, 5],
                vec![5, 6],
                vec![6, 7],
                vec![6, 8],
                vec![6, 9],
            ],
        ),
    ];
    for (n, edges) in test_cases {
        println!("{:?}", Solution::sum_of_distances_in_tree(n, edges));
    }
}

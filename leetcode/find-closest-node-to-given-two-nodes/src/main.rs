// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/description/?envType=daily-question&envId=2025-05-30
pub struct Solution;

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        fn calculate_distances(mut node: i32, edges: &[i32], distances: &mut [i32]) {
            let mut distance = 0;
            distances[node as usize] = distance;
            let mut next = edges[node as usize];
            // Unchangin is that distance = number of edges between start and node
            while next != -1 && distances[next as usize] == -1 {
                distance += 1;
                distances[next as usize] = distance;
                node = next;
                next = edges[node as usize];
            }
        }

        let mut distances_from_one = vec![-1; edges.len()];
        calculate_distances(node1, &edges, &mut distances_from_one);
        let mut distances_from_two = vec![-1; edges.len()];
        calculate_distances(node2, &edges, &mut distances_from_two);
        (0..edges.len())
            .enumerate()
            .filter_map(
                |(pos, i)| match (distances_from_one[i], distances_from_two[i]) {
                    (-1, _) | (_, -1) => None,
                    (first, second) => Some((first.max(second), pos as i32)),
                },
            )
            .min()
            .unwrap_or((0, -1))
            .1
    }
}

fn main() {
    println!("Hello, world!");
}

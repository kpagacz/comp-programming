// https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/description/
pub struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        use std::collections::HashMap;
        let edges: HashMap<i32, Vec<(i32, i32)>> =
            edges.iter().fold(HashMap::new(), |mut map, edge| {
                let (source, destination, weight) = (edge[0], edge[1], edge[2]);
                map.entry(source).or_default().push((destination, weight));
                map.entry(destination).or_default().push((source, weight));
                map
            });

        let mut answer = 0;
        let mut min_visited = usize::MAX;
        // println!("Edges: {edges:?}");
        for origin in 0..n {
            // println!("Origin: {origin}");
            let mut visited = vec![false; n as usize];
            use std::collections::BinaryHeap;
            let mut pq = BinaryHeap::from([(0, origin)]);

            while let Some((distance, node)) = pq.pop() {
                let distance = -distance;
                if distance > distance_threshold || visited[node as usize] {
                    continue;
                }

                visited[node as usize] = true;

                edges
                    .get(&node)
                    .get_or_insert(&Vec::new())
                    .iter()
                    .filter(|&&(node, _)| !visited[node as usize])
                    .for_each(|&(node, weight)| {
                        pq.push((-(distance + weight), node));
                    });
                // println!("{pq:?}");
            }

            // println!("{visited:?}");
            let visited_count = visited.into_iter().filter(|&visited| visited).count();
            if visited_count <= min_visited {
                answer = origin;
                min_visited = visited_count;
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (
            5,
            vec![
                vec![0, 1, 2],
                vec![0, 4, 8],
                vec![1, 2, 3],
                vec![1, 4, 2],
                vec![2, 3, 1],
                vec![3, 4, 1],
            ],
            2,
        ),
        (
            6,
            vec![
                vec![0, 1, 10],
                vec![0, 2, 1],
                vec![2, 3, 1],
                vec![1, 3, 1],
                vec![1, 4, 1],
                vec![4, 5, 10],
            ],
            20,
        ),
    ];

    for (n, edges, distance_threshold) in test_cases {
        println!("{}", Solution::find_the_city(n, edges, distance_threshold));
    }
}

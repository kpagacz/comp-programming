// https://leetcode.com/problems/design-graph-with-shortest-path-calculator/

use std::collections::HashMap;
struct Graph {
    cache: HashMap<(usize, usize), i32>,
    edges: HashMap<usize, Vec<(i32, usize, usize)>>,
    n: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Graph {
    fn new(n: i32, edges: Vec<Vec<i32>>) -> Self {
        Self {
            cache: HashMap::new(),
            edges: edges
                .into_iter()
                .map(|edge| (edge[2], edge[0] as usize, edge[1] as usize))
                .fold(HashMap::new(), |mut map, edge| {
                    map.entry(edge.1).or_insert(vec![]).push(edge);
                    map
                }),
            n: n as usize + 1,
        }
    }

    fn add_edge(&mut self, edge: Vec<i32>) {
        self.edges.entry(edge[0] as usize).or_insert(vec![]).push((
            edge[2],
            edge[0] as usize,
            edge[1] as usize,
        ));
        self.cache.clear();
    }

    fn shortest_path(&mut self, node1: i32, node2: i32) -> i32 {
        if let Some(&mem_cost) = self.cache.get(&(node1 as usize, node2 as usize)) {
            return mem_cost;
        }
        self.djikstra(node1 as usize, node2 as usize)
    }

    fn djikstra(&mut self, source: usize, destination: usize) -> i32 {
        use std::collections::BinaryHeap;

        let mut visited = [false; 101];
        let mut costs = [i32::MAX; 101];
        let mut heap = BinaryHeap::new();
        heap.push((0, source));
        costs[source] = 0;

        while let Some((_, node)) = heap.pop() {
            // println!("{costs:?}");
            if visited[node] {
                continue;
            }
            self.cache.insert((source, node), costs[node]);
            if node == destination {
                return costs[destination];
            }
            if let Some(destinations) = self.edges.get(&node) {
                destinations.iter().for_each(|&(cost, src, dest)| {
                    costs[dest] = costs[dest].min(costs[src] + cost);
                    heap.push((-costs[dest], dest));
                });
            }
            visited[node] = true;
        }

        self.cache.insert((source, destination), -1);
        -1
    }
}

/**
 * Your Graph object will be instantiated and called as such:
 * let obj = Graph::new(n, edges);
 * obj.add_edge(edge);
 * let ret_2: i32 = obj.shortest_path(node1, node2);
 */

fn main() {
    println!("Hello, world!");
}

pub struct Solution {}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut colour = vec![-1; graph.len()];
        let mut colouring_possible = true;
        (0..graph.len()).into_iter().for_each(|index| {
            if colour[index] == -1 {
                colour[index] = 0;
                colouring_possible = colouring_possible && Self::dfs(&mut colour, index, &graph);
            }
        });
        colouring_possible
    }

    fn dfs(colours: &mut Vec<i32>, source: usize, graph: &Vec<Vec<i32>>) -> bool {
        assert!(colours[source] != -1);
        let destinations_colour = 1 - colours[source];
        let mut colouring_possible = true;
        graph[source].iter().for_each(|&destination| {
            if colours[destination as usize] != -1 {
                colouring_possible =
                    colouring_possible && (colours[destination as usize] == destinations_colour);
            } else {
                colours[destination as usize] = destinations_colour;
                colouring_possible =
                    colouring_possible && (Self::dfs(colours, destination as usize, graph));
            }
        });
        colouring_possible
    }
}

fn main() {
    println!("Hello, world!");
}

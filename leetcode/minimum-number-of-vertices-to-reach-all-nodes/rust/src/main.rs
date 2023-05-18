pub struct Solution {}

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut reachable = vec![false; n as usize];
        edges.iter().for_each(|edge| {
            reachable[edge[1] as usize] = true;
        });
        let mut answer = vec![];
        reachable.iter().enumerate().for_each(|(i, val)| {
            if !val {
                answer.push(i as i32);
            }
        });

        answer
    }

    pub fn find_smallest_set_of_vertices2(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        edges
            .iter()
            .fold(vec![false; n as usize], |mut reachable, edge| {
                reachable[edge[1] as usize] = true;
                reachable
            })
            .into_iter()
            .enumerate()
            .filter_map(
                |(index, reachable)| {
                    if !reachable {
                        Some(index as i32)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

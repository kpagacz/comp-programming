// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/description/
pub struct Solution;

impl Solution {
    pub fn maximum_invitations(favourite: Vec<i32>) -> i32 {
        let mut reverse_fav = vec![vec![]; favourite.len()];
        for person in 0..favourite.len() {
            reverse_fav[favourite[person] as usize].push(person);
        }

        fn bfs(source: usize, reverse_fav: &[Vec<usize>], visited: &mut [bool]) -> i32 {
            let mut length = 0;
            let mut current: Vec<usize> = reverse_fav[source]
                .iter()
                .filter(|&n| !visited[*n])
                .copied()
                .collect();
            println!("source: {source} queue:{current:?} visited: {visited:?}");
            while !current.is_empty() {
                let mut new_rev_fav = vec![];
                for target in current {
                    println!("target: {target}");
                    if visited[target] {
                        continue;
                    }
                    println!("Adding: {:?}", reverse_fav[target]);
                    new_rev_fav.extend(reverse_fav[target].iter());
                    visited[target] = true;
                }
                current = new_rev_fav;
                length += 1;
                println!("{current:?}");
            }
            println!("{length}");
            length
        }

        let mut visited = vec![false; favourite.len()];
        let mut max_invited = 0;
        let mut max_2s_invited = 0;
        for start in 0..favourite.len() {
            if visited[start] {
                continue;
            }
            let mut distances = vec![-1; favourite.len()];
            let mut current = start;
            let mut distance = 0;

            loop {
                if visited[current] {
                    break;
                }
                visited[current] = true;
                distances[current] = distance;
                distance += 1;

                let next = favourite[current] as usize;
                if distances[next] != -1 {
                    let cycle_length = distance - distances[next];
                    max_invited = max_invited.max(cycle_length);
                    if cycle_length == 2 {
                        let mut visited_nodes = vec![false; favourite.len()];
                        visited_nodes[current] = true;
                        visited_nodes[next] = true;
                        max_2s_invited = max_2s_invited.max(
                            2 + bfs(current, &reverse_fav, &mut visited_nodes)
                                + bfs(next, &reverse_fav, &mut visited_nodes),
                        );
                    }
                    break;
                }
                current = next;
            }
        }
        println!("{max_invited} {max_2s_invited}");
        max_invited.max(max_2s_invited)
    }
}

fn main() {
    let test_cases = [vec![2, 2, 1, 2]];

    for test in test_cases {
        println!("Test: {test:?}");
        println!("{}", Solution::maximum_invitations(test));
    }
}

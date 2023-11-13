// https://leetcode.com/problems/bus-routes/description/
pub struct Solution {}

impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        use std::collections::BTreeMap;
        use std::collections::BTreeSet;

        let routes: Vec<BTreeSet<i32>> = routes
            .into_iter()
            .map(|route| {
                route.into_iter().fold(BTreeSet::new(), |mut set, el| {
                    set.insert(el);
                    set
                })
            })
            .collect();
        let mut connections = BTreeMap::new();
        (0..routes.len()).for_each(|i| {
            ((i + 1)..routes.len()).for_each(|j| {
                if routes[i].intersection(&routes[j]).count() > 0 {
                    connections.entry(i).or_insert(vec![]).push(j);
                    connections.entry(j).or_insert(vec![]).push(i);
                }
            })
        });

        println!("{routes:?}");
        println!("{connections:?}");

        if source == target {
            return 0;
        }

        let mut lines: Vec<_> = routes
            .iter()
            .enumerate()
            .filter_map(|(id, route)| {
                if route.contains(&source) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        let mut used_lines = vec![false; routes.len()];
        let mut step = 1;

        while !lines.is_empty() {
            for &line in &lines {
                if routes[line].contains(&target) {
                    return step;
                }
                used_lines[line] = true;
            }
            step += 1;
            lines = lines
                .into_iter()
                .flat_map(|line| match connections.get(&line) {
                    Some(other_lines) => other_lines
                        .iter()
                        .filter(|&&el| !used_lines[el])
                        .map(|&el| el)
                        .collect(),
                    None => vec![],
                })
                .collect();
        }

        -1
    }

    // pub fn num_buses_to_destination_tle(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    //     if source == target {
    //         return 0;
    //     }
    //     use std::collections::HashSet;
    //     let routes: Vec<HashSet<_>> = routes
    //         .into_iter()
    //         .map(|route| {
    //             route.into_iter().fold(HashSet::new(), |mut set, el| {
    //                 set.insert(el);
    //                 set
    //             })
    //         })
    //         .collect();

    //     let mut visited = HashSet::new();
    //     visited.insert(source);
    //     let mut queue = vec![source];
    //     let mut step = 0;

    //     while !queue.is_empty() {
    //         queue = queue
    //             .into_iter()
    //             .flat_map(|stop| {
    //                 let mut can_travel_to = vec![];
    //                 routes.iter().for_each(|line| {
    //                     if line.contains(&stop) {
    //                         can_travel_to.extend(line.iter());
    //                     }
    //                 });
    //                 can_travel_to
    //             })
    //             .collect();

    //         queue.sort_unstable();
    //         queue.dedup();
    //         queue = queue
    //             .into_iter()
    //             .filter(|el| !visited.contains(el))
    //             .collect();
    //         visited.extend(queue.iter());
    //         step += 1;
    //         if visited.contains(&target) {
    //             return step;
    //         }
    //     }
    //     -1
    // }
}
fn main() {
    let test_cases = vec![(vec![vec![1, 2, 7], vec![3, 6, 7]], 1, 6)];
    for (routes, source, target) in test_cases {
        println!(
            "{}",
            Solution::num_buses_to_destination(routes, source, target)
        );
    }
}

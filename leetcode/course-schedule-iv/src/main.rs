// https://leetcode.com/problems/course-schedule-iv/description/
pub struct Solution;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let num_courses = num_courses as usize;
        let mut req_lists = vec![vec![]; num_courses];

        // Create req lists
        for prereq in &prerequisites {
            let (pre, post) = (prereq[0], prereq[1]);
            req_lists[post as usize].push(pre as usize);
        }

        use std::collections::HashMap;
        fn ancestry(
            node: usize,
            req_lists: &[Vec<usize>],
            target: usize,
            mem: &mut HashMap<(usize, usize), bool>,
        ) -> bool {
            if let Some(answer) = mem.get(&(node, target)) {
                return *answer;
            }
            if node == target {
                return true;
            }

            let direct_ancestors = &req_lists[node];
            let ans = direct_ancestors
                .iter()
                .any(|ancestor| ancestry(*ancestor, req_lists, target, mem));
            mem.insert((node, target), ans);
            ans
        }

        let mut mem = HashMap::new();
        queries
            .into_iter()
            .map(|query| {
                let (pre, post) = (query[0] as usize, query[1] as usize);
                ancestry(post, &req_lists, pre, &mut mem)
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

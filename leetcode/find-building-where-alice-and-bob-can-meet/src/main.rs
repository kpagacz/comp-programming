// https://leetcode.com/problems/find-building-where-alice-and-bob-can-meet/description/
pub struct Solution;

impl Solution {
    pub fn leftmost_building_queries(heights: Vec<i32>, mut queries: Vec<Vec<i32>>) -> Vec<i32> {
        queries.iter_mut().for_each(|query| {
            if query[0] > query[1] {
                query.swap(0, 1);
            }
        });
        let mut new_queries = vec![vec![]; heights.len()];
        let mut res = vec![-1i32; queries.len()];
        for (pos, query) in queries.into_iter().enumerate() {
            let (a, b) = (query[0] as usize, query[1] as usize);
            if a == b || heights[a] < heights[b] {
                res[pos] = b as i32;
            } else {
                new_queries[b].push((heights[a], pos));
            }
        }

        println!("New queries: {new_queries:?}");
        let mut stack: Vec<(i32, usize)> = vec![];
        for (pos, height) in heights.into_iter().enumerate().rev() {
            for &(building_height, query_pos) in &new_queries[pos] {
                // println!("Query: building_height {building_height} query_pos {query_pos}");
                // println!("Stack: {stack:?}");
                let pp = stack.partition_point(|(height, _)| building_height < *height);
                // println!("pp: {pp}");
                if pp > 0 {
                    res[query_pos] = stack[pp - 1].1 as i32;
                }
            }

            while !stack.is_empty() && height >= stack[stack.len() - 1].0 {
                stack.pop();
            }
            stack.push((height, pos));
        }

        res
    }
}

fn main() {
    let test_cases = [(
        vec![6, 4, 8, 5, 2, 7],
        vec![vec![0, 1], vec![0, 3], vec![2, 4], vec![3, 4], vec![2, 2]],
    )];

    for (heights, queries) in test_cases {
        println!("Heights {heights:?} queries: {queries:?}");
        println!(
            "{:?}, expected [2,5,-1,5,2]",
            Solution::leftmost_building_queries(heights, queries)
        );
    }
}

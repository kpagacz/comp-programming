// https://leetcode.com/problems/zero-array-transformation-ii/description/
pub struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let mut diff_array: Vec<_> = std::iter::once(nums[0])
            .chain(nums.windows(2).map(|window| window[1] - window[0]))
            .collect();

        fn execute_query(diff_array: &mut [i32], query: &[i32], reverse: bool) {
            let (left, right, delta) = (query[0] as usize, query[1] as usize, query[2]);

            if reverse {
                diff_array[left] += delta;
                if right + 1 < diff_array.len() {
                    diff_array[right + 1] -= delta;
                }
            } else {
                diff_array[left] -= delta;
                if right + 1 < diff_array.len() {
                    diff_array[right + 1] += delta;
                }
            }
        }

        fn execute_queries(
            diff_array: &mut [i32],
            queries: &[Vec<i32>],
            how_many: usize,
            reverse: bool,
        ) {
            for query in &queries[..how_many] {
                execute_query(diff_array, query, reverse);
            }
        }

        fn is_all_zero(diff_array: &[i32]) -> bool {
            let mut curr = diff_array[0];
            if curr > 0 {
                return false;
            }

            for &diff in &diff_array[1..] {
                curr += diff;
                if curr > 0 {
                    return false;
                }
            }

            true
        }

        let candidates: Vec<_> = (0..=queries.len()).collect();

        let pp = candidates.partition_point(|&how_many_queries| {
            execute_queries(&mut diff_array, &queries, how_many_queries, false);
            // println!("How many: {how_many_queries}");
            // println!("{:?}", diff_array);
            let is_zero = is_all_zero(&diff_array);
            execute_queries(&mut diff_array, &queries, how_many_queries, true);
            // println!("{:?}", diff_array);
            !is_zero
        });

        if pp == candidates.len() {
            -1
        } else {
            candidates[pp] as _
        }
    }
}

fn main() {
    let test_cases = [(
        vec![2, 0, 2],
        vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]],
    )];

    for (nums, queries) in test_cases {
        println!("{}", Solution::min_zero_array(nums, queries));
    }
}

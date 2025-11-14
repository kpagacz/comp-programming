// https://leetcode.com/problems/increment-submatrices-by-one/description/?envType=daily-question&envId=2025-11-14
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut row_prefixes = vec![vec![0; n]; n];

        for query in queries {
            let (start_row, start_col, end_row, end_col) = (
                query[0] as usize,
                query[1] as usize,
                query[2] as usize,
                query[3] as usize,
            );
            for row in start_row..=end_row {
                row_prefixes[row][start_col] += 1;
                if end_col + 1 < n {
                    row_prefixes[row][end_col + 1] -= 1;
                }
            }
        }

        row_prefixes.iter_mut().for_each(|row| {
            for i in 1..row.len() {
                row[i] += row[i - 1];
            }
        });

        row_prefixes
    }
}

fn main() {
    let test_cases = [(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]])];
    for (n, queries) in test_cases {
        println!("{:?}", Solution::range_add_queries(n, queries));
    }
}

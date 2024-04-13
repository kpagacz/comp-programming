// https://leetcode.com/problems/maximal-rectangle/description/
pub struct Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let column_prefixes = Solution::get_column_prefixes(&matrix);

        let mut max_area = 0;

        for row in &column_prefixes {
            max_area = max_area.max(Solution::max_area_in_a_row(row));
        }

        max_area as _
    }

    fn get_column_prefixes(matrix: &[Vec<char>]) -> Vec<Vec<usize>> {
        let mut column_prefixes = vec![vec![0; matrix[0].len() + 1]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    column_prefixes[i][j] = 1;
                    if i > 0 {
                        column_prefixes[i][j] += column_prefixes[i - 1][j];
                    }
                } else {
                    column_prefixes[i][j] = 0;
                }
            }
        }

        column_prefixes
    }

    fn max_area_in_a_row(col_prefixes: &[usize]) -> usize {
        let mut answer = 0;
        let mut monotonic_stack = vec![];

        for (pos, &ones) in col_prefixes.iter().enumerate() {
            while !monotonic_stack.is_empty()
                && ones < col_prefixes[*monotonic_stack.last().unwrap()]
            {
                let last_index = monotonic_stack.pop().unwrap();
                let height = col_prefixes[last_index];
                let width = match monotonic_stack.last() {
                    Some(before_last_index) => pos - before_last_index - 1,
                    None => pos,
                };
                answer = answer.max(height * width);
            }
            monotonic_stack.push(pos);
        }

        answer
    }
}

fn main() {
    let test_cases = [
        vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0'],
        ],
        vec![vec!['0']],
        vec![vec!['1']],
    ];

    for matrix in test_cases {
        println!("{}", Solution::maximal_rectangle(matrix));
    }
}

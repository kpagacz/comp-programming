// https://leetcode.com/problems/maximal-rectangle/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    // This is the kind of problem that verbose asks
    // to find the largest square.
    // I need to remember that when someone basically asks me to find
    // the largets square then I should immediately think monotonic stack.
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut col_prefixes = vec![vec![0; matrix[0].len() + 1]; matrix.len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '1' {
                    if i > 0 {
                        col_prefixes[i][j] = col_prefixes[i - 1][j] + 1;
                    } else {
                        col_prefixes[i][j] = 1;
                    }
                }
            }
        }

        let mut monotonic_stack = vec![];
        let mut max_area = 0;
        for row in col_prefixes {
            for (pos, &col_height) in row.iter().enumerate() {
                while !monotonic_stack.is_empty()
                    && col_height < row[*monotonic_stack.last().unwrap()]
                {
                    let popped_height_index = monotonic_stack.pop().unwrap();
                    let rectangle_height = row[popped_height_index];
                    let rectangle_width = match monotonic_stack.last() {
                        // indices 0,1,2 => width should be 1, because 2 is added, 1 is considered
                        // and it's just 1 that has the required height
                        Some(before_popped_height_index) => pos - before_popped_height_index - 1,
                        // indices 1,2 => width should be 2, because 2 is added, 1 is considered
                        // and 0 and 1 have the required height
                        None => pos,
                    };
                    max_area = max_area.max(rectangle_height * rectangle_width);
                }
                monotonic_stack.push(pos);
            }
            monotonic_stack.clear();
        }

        max_area as _
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

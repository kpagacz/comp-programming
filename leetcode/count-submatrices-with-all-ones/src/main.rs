// https://leetcode.com/problems/count-submatrices-with-all-ones/description/?envType=daily-question&envId=2025-08-21
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_submat(mut mat: Vec<Vec<i32>>) -> i32 {
        (0..mat.len()).for_each(|row| {
            (0..mat[0].len()).for_each(|col| {
                if mat[row][col] > 0 {
                    mat[row][col] = *mat[row].get(col + usize::MAX).unwrap_or(&0) + 1;
                }
            })
        });

        let mut answer = 0;
        for row in 0..mat.len() {
            for col in 0..mat[0].len() {
                if mat[row][col] > 0 {
                    answer += mat[row][col];
                    let mut running_min = mat[row][col];
                    for previous_row in (0..row).rev() {
                        running_min = running_min.min(mat[previous_row][col]);
                        answer += running_min;
                    }
                }
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]], 5)];

    for (mat, exp) in test_cases {
        for row in &mat {
            println!("{row:?}");
        }
        println!("{}, exp: {exp}", Solution::num_submat(mat));
    }
}

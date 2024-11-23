// https://leetcode.com/problems/rotating-the-box/description/
pub struct Solution;

impl Solution {
    pub fn rotate_the_box(b: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut res = vec![vec!['.'; b.len()]; b[0].len()];
        for row in 0..b.len() {
            let dest_col = b.len() - 1 - row;
            let mut last_ground = b[0].len();

            // Starts from the bottom of the rotated box
            for col in (0..b[0].len()).rev() {
                match b[row][col] {
                    '#' => {
                        last_ground -= 1;
                        res[last_ground][dest_col] = '#';
                    }
                    '*' => {
                        res[col][dest_col] = '*';
                        last_ground = col;
                    }
                    _ => {}
                }
            }
        }
        res
    }
}

fn main() {
    let test_cases = [
        vec![vec!['#', '.', '#']],
        vec![vec!['#', '.', '*', '.'], vec!['#', '#', '*', '.']],
    ];

    for b in test_cases {
        let r = Solution::rotate_the_box(b);
        for line in r {
            println!("{:?}", line);
        }
    }
}

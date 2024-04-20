// https://leetcode.com/problems/find-all-groups-of-farmland/description/
pub struct Solution;

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer = vec![];

        for i in 0..land.len() {
            for j in 0..land[0].len() {
                if Solution::is_top_left_corner(&land, i, j) {
                    let bottom_right_corner = Solution::find_bottom_right_corner(&land, i, j);
                    answer.push(vec![
                        i as i32,
                        j as i32,
                        bottom_right_corner.0 as i32,
                        bottom_right_corner.1 as i32,
                    ]);
                }
            }
        }

        answer
    }

    fn is_top_left_corner(land: &[Vec<i32>], i: usize, j: usize) -> bool {
        if land[i][j] == 0 {
            return false;
        }
        // land[i][j] == 1 below
        if i == 0 && j == 0 {
            true
        } else if i == 0 {
            land[i][j - 1] == 0
        } else if j == 0 {
            land[i - 1][j] == 0
        } else {
            land[i - 1][j] == 0 && land[i][j - 1] == 0
        }
    }

    fn find_bottom_right_corner(land: &[Vec<i32>], mut i: usize, mut j: usize) -> (usize, usize) {
        while i + 1 < land.len() && land[i + 1][j] == 1 {
            i += 1;
        }
        while j + 1 < land[0].len() && land[i][j + 1] == 1 {
            j += 1;
        }
        (i, j)
    }
}

fn main() {
    println!("Hello, world!");
}

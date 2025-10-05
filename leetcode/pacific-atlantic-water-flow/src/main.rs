// https://leetcode.com/problems/pacific-atlantic-water-flow/description/?envType=daily-question&envId=2025-10-05
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        const NEIGHBOURS: [(usize, usize); 4] = [(0, 1), (0, usize::MAX), (1, 0), (usize::MAX, 0)];
        const ATLANTIC: i32 = 0b1;
        const PACIFIC: i32 = 0b10;
        const BOTH: i32 = 0b11;
        fn flood(x: usize, y: usize, marker: i32, heights: &[Vec<i32>], reach: &mut [Vec<i32>]) {
            if reach[x][y] == marker || reach[x][y] == BOTH {
                return;
            }

            if (reach[x][y] == ATLANTIC && marker == PACIFIC)
                || (reach[x][y] == PACIFIC && marker == ATLANTIC)
            {
                reach[x][y] = BOTH;
            } else {
                reach[x][y] = marker;
            }

            for (delta_x, delta_y) in NEIGHBOURS {
                let new_x = x.wrapping_add(delta_x);
                let new_y = y.wrapping_add(delta_y);

                if new_x < heights.len()
                    && new_y < heights[0].len()
                    && heights[x][y] <= heights[new_x][new_y]
                {
                    flood(new_x, new_y, marker, heights, reach);
                }
            }
        }

        let mut reach = vec![vec![0; heights[0].len()]; heights.len()];
        for row in 0..heights.len() {
            flood(row, 0, ATLANTIC, &heights, &mut reach);
            flood(row, heights[0].len() - 1, PACIFIC, &heights, &mut reach);
        }
        for col in 0..heights[0].len() {
            flood(0, col, ATLANTIC, &heights, &mut reach);
            flood(heights.len() - 1, col, PACIFIC, &heights, &mut reach);
        }

        let mut answer = vec![];
        for (row_pos, row) in reach.into_iter().enumerate() {
            for (col_pos, value) in row.into_iter().enumerate() {
                if value == BOTH {
                    answer.push(vec![row_pos as i32, col_pos as i32]);
                }
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [
        vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4],
        ],
        vec![vec![1]],
    ];
    for heights in test_cases {
        println!("{:?}", Solution::pacific_atlantic(heights));
    }
}

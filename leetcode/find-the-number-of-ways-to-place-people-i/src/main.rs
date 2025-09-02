// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let plane = Solution::to_plane(&points);

        let mut count = 0;
        (0..points.len()).for_each(|first| {
            (0..points.len()).for_each(|second| {
                let (first_x, first_y) = (points[first][0] as usize, points[first][1] as usize);
                let (second_x, second_y) = (points[second][0] as usize, points[second][1] as usize);

                if first_x >= second_x
                    && first_y <= second_y
                    && Solution::points_in_rectangle(&plane, first_x, first_y, second_x, second_y)
                        == 2
                {
                    count += 1;
                }
            });
        });
        count
    }

    fn points_in_rectangle(
        plane: &[Vec<i32>],
        first_x: usize,
        first_y: usize,
        second_x: usize,
        second_y: usize,
    ) -> i32 {
        let mut points = 0;
        for x in second_x..=first_x {
            for y in first_y..=second_y {
                points += plane[x][y];
            }
        }
        points
    }

    fn to_plane(points: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let mut plane = vec![vec![0; 51]; 51];
        for point in points {
            let (x, y) = (point[0] as usize, point[1] as usize);
            plane[x][y] += 1;
        }
        plane
    }
}

fn main() {
    println!("Hello, world!");
}

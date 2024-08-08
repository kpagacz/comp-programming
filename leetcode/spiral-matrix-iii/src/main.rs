// https://leetcode.com/problems/spiral-matrix-iii/description/
pub struct Solution;

#[derive(Debug)]
struct SpiralCoordsIter {
    coords: (i32, i32),
    direction: (i32, i32),
    leg: i32,
    leg_length: i32,
    hops_left_in_leg: i32,
}

impl Iterator for SpiralCoordsIter {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.hops_left_in_leg == 0 {
            // New leg
            let old = self.direction.0;
            self.direction.0 = self.direction.1;
            self.direction.1 = -old;

            if self.leg == 2 {
                self.leg = 0;
                self.leg_length += 1;
            }

            self.hops_left_in_leg = self.leg_length;
            self.leg += 1;
        }

        self.coords.0 += self.direction.0;
        self.coords.1 += self.direction.1;
        self.hops_left_in_leg -= 1;

        // println!("{:?}", self);
        Some(self.coords)
    }
}

#[derive(Debug)]
struct Coords((i32, i32));

impl IntoIterator for Coords {
    type Item = (i32, i32);
    type IntoIter = SpiralCoordsIter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            coords: self.0,
            direction: (-1, 0),
            hops_left_in_leg: 0,
            leg: 2,
            leg_length: 0,
        }
    }
}

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut todo = rows * cols;

        let mut it = Coords((r_start, c_start)).into_iter();
        let mut answer = vec![];
        answer.push(vec![r_start, c_start]);
        todo -= 1;
        while todo > 0 {
            let (x, y) = it.next().unwrap();
            if x >= 0 && x < rows && y >= 0 && y < cols {
                answer.push(vec![x, y]);
                todo -= 1;
            }
        }
        answer
    }
}

fn main() {
    let test_cases = [(1, 4, 0, 0), (5, 6, 1, 4)];
    for (rows, cols, r_start, c_start) in test_cases {
        println!(
            "{:?}",
            Solution::spiral_matrix_iii(rows, cols, r_start, c_start)
        );
    }
}

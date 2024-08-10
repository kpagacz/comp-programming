// https://leetcode.com/problems/regions-cut-by-slashes/description/
pub struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let height = grid.len();
        let width = grid[0].len();
        let mut components = (0..width * height * 4).collect::<Vec<usize>>();
        let mut result = (width * height * 4) as i32;
        for (i, line) in grid.iter().enumerate() {
            for (j, &ch) in line.as_bytes().iter().enumerate() {
                match ch {
                    b' ' => {
                        Self::union(
                            (i * width + j) * 4,
                            (i * width + j) * 4 + 1,
                            &mut components,
                            &mut result,
                        );
                        Self::union(
                            (i * width + j) * 4,
                            (i * width + j) * 4 + 2,
                            &mut components,
                            &mut result,
                        );
                        Self::union(
                            (i * width + j) * 4,
                            (i * width + j) * 4 + 3,
                            &mut components,
                            &mut result,
                        );
                    }
                    b'/' => {
                        Self::union(
                            (i * width + j) * 4,
                            (i * width + j) * 4 + 1,
                            &mut components,
                            &mut result,
                        );
                        Self::union(
                            (i * width + j) * 4 + 2,
                            (i * width + j) * 4 + 3,
                            &mut components,
                            &mut result,
                        );
                    }
                    b'\\' => {
                        Self::union(
                            (i * width + j) * 4,
                            (i * width + j) * 4 + 2,
                            &mut components,
                            &mut result,
                        );
                        Self::union(
                            (i * width + j) * 4 + 1,
                            (i * width + j) * 4 + 3,
                            &mut components,
                            &mut result,
                        );
                    }
                    _ => panic!(""),
                }
                Self::connect(i, j, width, height, &mut components, &mut result);
            }
        }
        result
    }

    fn union(n1: usize, n2: usize, components: &mut Vec<usize>, result: &mut i32) {
        let p1 = Solution::find(n1, components);
        let p2 = Solution::find(n2, components);
        if p1 != p2 {
            components[p2] = p1;
            *result -= 1;
        }
    }

    fn find(n: usize, components: &mut Vec<usize>) -> usize {
        let mut p = components[n];
        if p != n {
            p = Solution::find(p, components);
            components[n] = p;
        }

        return p;
    }

    fn connect(
        i: usize,
        j: usize,
        width: usize,
        height: usize,
        components: &mut Vec<usize>,
        result: &mut i32,
    ) {
        if i > 0 {
            Self::union(
                (i * width + j) * 4,
                ((i - 1) * width + j) * 4 + 3,
                components,
                result,
            );
        }
        if j > 0 {
            Self::union(
                (i * width + j) * 4 + 1,
                (i * width + j - 1) * 4 + 2,
                components,
                result,
            );
        }
        if i < height - 1 {
            Self::union(
                (i * width + j) * 4 + 3,
                ((i + 1) * width + j) * 4,
                components,
                result,
            );
        }
        if j < width - 1 {
            Self::union(
                (i * width + j) * 4 + 2,
                (i * width + j + 1) * 4 + 1,
                components,
                result,
            );
        }
    }
}
fn main() {
    println!("Hello, world!");
}

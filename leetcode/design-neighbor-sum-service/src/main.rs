// https://leetcode.com/problems/design-neighbor-sum-service/description/
struct NeighborSum {
    grid: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        Self { grid }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                if self.grid[x][y] == value {
                    let mut sum = 0;
                    if x > 0 {
                        sum += self.grid[x - 1][y];
                    }
                    if x < self.grid.len() - 1 {
                        sum += self.grid[x + 1][y];
                    }
                    if y > 0 {
                        sum += self.grid[x][y - 1];
                    }
                    if y < self.grid.len() - 1 {
                        sum += self.grid[x][y + 1];
                    }
                    return sum;
                }
            }
        }
        -1
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        for x in 0..self.grid.len() {
            for y in 0..self.grid.len() {
                if self.grid[x][y] == value {
                    let mut sum = 0;
                    if x > 0 && y > 0 {
                        sum += self.grid[x - 1][y - 1];
                    }
                    if x > 0 && y < self.grid.len() - 1 {
                        sum += self.grid[x - 1][y + 1];
                    }
                    if x < self.grid.len() - 1 && y > 0 {
                        sum += self.grid[x + 1][y - 1];
                    }
                    if x < self.grid.len() - 1 && y < self.grid.len() - 1 {
                        sum += self.grid[x + 1][y + 1];
                    }
                    return sum;
                }
            }
        }
        -1
    }
}

/**
 * Your NeighborSum object will be instantiated and called as such:
 * let obj = NeighborSum::new(grid);
 * let ret_1: i32 = obj.adjacent_sum(value);
 * let ret_2: i32 = obj.diagonal_sum(value);
 */

fn main() {
    println!("Hello, world!");
}

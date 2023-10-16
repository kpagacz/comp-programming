// https://leetcode.com/problems/construct-product-matrix/solutions/4169695/python-rust-flatten-prefix-suffix-bonus/
pub struct Solution {}
impl Solution {
    pub fn construct_product_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (grid.len(), grid[0].len());
        let (mut prefixes, mut suffixes) = (vec![1], vec![1]);
        let m = 12345;

        for i in 0..rows {
            for j in 0..cols {
                prefixes.push((prefixes[prefixes.len() - 1] * (grid[i][j] % m)) % m);
            }
        }
        for i in (0..rows).rev() {
            for j in (0..cols).rev() {
                suffixes.push((suffixes[suffixes.len() - 1] * (grid[i][j] % m)) % m);
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                let k = i * cols + j;
                grid[i][j] = (prefixes[k] * suffixes[suffixes.len() - k - 2]) % m;
            }
        }

        grid
    }
}
fn main() {
    println!("Hello, world!");
}

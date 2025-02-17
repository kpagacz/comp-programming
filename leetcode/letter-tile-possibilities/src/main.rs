// https://leetcode.com/problems/letter-tile-possibilities/description/
pub struct Solution;

impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut counts = [0; 26];

        tiles.bytes().for_each(|b| counts[(b - b'A') as usize] += 1);

        dfs(&mut counts)
    }
}

fn dfs(counts: &mut [i32; 26]) -> i32 {
    let mut n_perm = 0;

    for i in 0..26 {
        if counts[i] > 0 {
            n_perm += 1;
            counts[i] -= 1;

            n_perm += dfs(counts);

            counts[i] += 1;
        }
    }
    n_perm
}

fn main() {
    println!("Hello, world!");
}

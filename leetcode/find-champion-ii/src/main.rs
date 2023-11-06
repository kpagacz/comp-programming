pub struct Solution {}

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut beaten = vec![false; n];
        for edge in &edges {
            beaten[edge[1] as usize] = true;
        }

        let count = beaten.iter().filter(|&beaten| !beaten).count();
        if count == 1 {
            beaten.iter().position(|&beaten| !beaten).unwrap() as i32
        } else {
            -1
        }
    }
}

fn main() {
    println!("Hello, world!");
}

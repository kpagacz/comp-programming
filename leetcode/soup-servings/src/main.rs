// https://leetcode.com/problems/soup-servings/description/?envType=daily-question&envId=2025-08-08
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        fn rec(a: i32, b: i32, mem: &mut [Vec<f64>]) -> f64 {
            match (a <= 0, b <= 0) {
                (true, true) => return 0.5,
                (true, _) => return 1.0,
                (_, true) => return 0.0,
                _ => {}
            }

            let a = a as usize;
            let b = b as usize;
            if mem[a][b] > 0.0 {
                return mem[a][b];
            }

            let a = a as i32;
            let b = b as i32;
            let val = rec(a - 100, b, mem)
                + rec(a - 75, b - 25, mem)
                + rec(a - 50, b - 50, mem)
                + rec(a - 25, b - 75, mem);
            let a = a as usize;
            let b = b as usize;
            mem[a][b] = 0.25 * val;
            mem[a][b]
        }

        if n > 4750 {
            return 1.0;
        }

        let mut mem = vec![vec![0.0; n as usize + 1]; n as usize + 1];
        rec(n, n, &mut mem)
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options
pub struct Solution {}

const MODULO: i64 = 1000_000_000 + 7;
impl Solution {
    // Beats 100%
    // Formula: (2 * n)! / 2^n
    // Hint: the problem is symmetric!
    // Count the numbers of variations of 2n elements
    // And then divide by the number of ways you can
    // shuffle the pair of corresponding elements (p and d)
    pub fn count_orders(n: i32) -> i32 {
        let mut answer = 1_i64;
        let mut it = 2 * n as i64;
        while it > 1 {
            answer = answer * it * (it - 1);
            answer = answer / 2;
            answer = answer % MODULO;
            it = it - 2;
        }
        answer as i32
    }
}

fn main() {
let tests = vec![1,2,3,4,500];
}

// https://leetcode.com/problems/find-the-winner-of-the-circular-game/description/
pub struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut friends = (0..n).collect::<Vec<i32>>();
        let k = k as usize;
        let mut to_remove = friends.len();

        while friends.len() > 1 {
            let n = friends.len();
            to_remove = dbg!((((to_remove + k) % n) + n - 1) % n);
            friends.remove(to_remove);
        }

        *friends.first().unwrap() + 1
    }
}

fn main() {
    let test_cases = [(6, 5)];
    for (n, k) in test_cases {
        println!("{}", Solution::find_the_winner(n, k));
    }
}

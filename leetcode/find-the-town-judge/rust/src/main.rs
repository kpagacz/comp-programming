// https://leetcode.com/problems/find-the-town-judge/description/
pub struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusts_someone = vec![false; n];
        let mut trust_count = vec![0; n];

        for trust in trust {
            let (trustee, trusted) = (trust[0] - 1, trust[1] - 1);
            trusts_someone[trustee as usize] = true;
            trust_count[trusted as usize] += 1;
        }

        for i in 0..n {
            if !trusts_someone[i] && trust_count[i] == n - 1 {
                return i as i32 + 1;
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}

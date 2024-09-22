// https://leetcode.com/problems/k-th-smallest-in-lexicographical-order/description/
pub struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut k = k;
        let mut cur = 1;
        k -= 1;
        while k > 0 {
            let steps = Self::count_steps(n, cur, cur + 1);
            if steps <= k {
                cur += 1;
                k -= steps;
            } else {
                cur *= 10;
                k -= 1;
            }
        }
        cur
    }

    fn count_steps(n: i32, n1: i32, n2: i32) -> i32 {
        let mut n1 = n1 as i64;
        let mut n2 = n2 as i64;
        let n = n as i64;
        let mut steps = 0i64;
        while n1 <= n {
            steps += std::cmp::min(n + 1, n2) - n1;
            n1 *= 10;
            n2 *= 10;
        }
        steps as _
    }
}

fn main() {
    let test_cases = [(13, 2), (100, 10), (100, 90)];

    for (n, k) in test_cases {
        println!("n: {}, k: {}", n, k);
        println!("Result: {}", Solution::find_kth_number(n, k));
    }
}

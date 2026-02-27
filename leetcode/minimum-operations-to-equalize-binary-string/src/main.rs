// https://leetcode.com/problems/minimum-operations-to-equalize-binary-string/description/?envType=daily-question&envId=2026-02-27
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let zeros = s.chars().filter(|c| *c == '0').count();
        let k = k as usize;
        if zeros == 0 {
            return 0;
        }
        if k > s.len() {
            return -1;
        }

        if k == s.len() {
            if zeros == s.len() {
                return 1;
            } else {
                return -1;
            }
        }

        let base = s.len() - k;
        let mut odd = std::cmp::max(zeros.div_ceil(k), (s.len() - zeros).div_ceil(base));
        odd += !odd & 1;
        let mut even = std::cmp::max(zeros.div_ceil(k), zeros.div_ceil(base));
        even += even & 1;
        let mut res = i32::MAX;
        if (k & 1) == (zeros & 1) {
            res = odd as _;
        }
        if zeros & 1 == 0 {
            res = res.min(even as _);
        }
        if res == i32::MAX { -1 } else { res }
    }
}

fn main() {
    println!("Hello, world!");
}

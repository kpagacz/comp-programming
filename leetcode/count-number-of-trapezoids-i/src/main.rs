// https://leetcode.com/problems/count-number-of-trapezoids-i/description/?envType=daily-question&envId=2025-12-02
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut on_y = HashMap::new();
        for point in points {
            let (_, y) = (point[0], point[1]);
            on_y.entry(y)
                .and_modify(|count| *count += 1)
                .or_insert(1i64);
        }
        let mut answer = 0i64;
        const MOD: i64 = 10i64.pow(9) + 7;
        let mut pairs_so_far = 0i64;
        for (_, count) in on_y {
            let pairs = ((count * (count - 1)) / 2) % MOD;
            answer += pairs * pairs_so_far;
            answer %= MOD;
            pairs_so_far += pairs;
            pairs_so_far %= MOD;
        }
        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/description/?envType=daily-question&envId=2026-01-16
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;
        h_fences.push(1);
        h_fences.push(m);
        v_fences.push(1);
        v_fences.push(n);
        h_fences.sort();
        v_fences.sort();

        let h_diffs = h_fences
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();
        let v_diffs = v_fences
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Vec<_>>();

        use std::collections::HashSet;
        let mut h_possible_sides = HashSet::new();
        for i in 0..h_diffs.len() {
            let mut possible_side: i64 = h_diffs[i] as i64;
            h_possible_sides.insert(possible_side);
            for &diff in h_diffs.iter().skip(i + 1) {
                possible_side += diff as i64;
                h_possible_sides.insert(possible_side);
            }
        }

        let mut max_side = 0i64;
        for i in 0..v_diffs.len() {
            let mut possible_side = v_diffs[i] as i64;
            if h_possible_sides.contains(&possible_side) {
                max_side = max_side.max(possible_side);
            }
            for &diff in v_diffs.iter().skip(i + 1) {
                possible_side += diff as i64;
                if h_possible_sides.contains(&possible_side) {
                    max_side = max_side.max(possible_side);
                }
            }
        }

        if max_side == 0 {
            -1
        } else {
            (((max_side % MOD) * (max_side % MOD)) % MOD) as _
        }
    }
}

fn main() {
    let test_cases = [(4, 5, vec![2], vec![4], 9)];
    for (m, n, h_fences, v_fences, exp) in test_cases {
        println!(
            "{} exp {exp}",
            Solution::maximize_square_area(m, n, h_fences, v_fences)
        );
    }
}

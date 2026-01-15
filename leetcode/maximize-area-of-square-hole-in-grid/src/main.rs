// https://leetcode.com/problems/maximize-area-of-square-hole-in-grid/description/?envType=daily-question&envId=2026-01-15
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximize_square_hole_area(
        _n: i32,
        _m: i32,
        mut h_bars: Vec<i32>,
        mut v_bars: Vec<i32>,
    ) -> i32 {
        fn find_longest_cons(arr: &mut [i32]) -> usize {
            arr.sort();
            arr.iter()
                .fold((1usize, 0, -1i32), |(longest, current, previous), num| {
                    if *num == previous + 1 {
                        (longest.max(current + 1), current + 1, *num)
                    } else {
                        (longest, 1, *num)
                    }
                })
                .0
        }
        let longest_h_cons = find_longest_cons(&mut h_bars);
        let longest_v_cons = find_longest_cons(&mut v_bars);
        let side_length = longest_h_cons.min(longest_v_cons) + 1;
        (side_length * side_length) as _
    }
}

fn main() {
    let test_cases = [
        (1, 1, vec![2], vec![2], 4),
        (2, 3, vec![2, 3], vec![2, 4], 4),
        (4, 40, vec![5, 3, 2, 4], vec![36, 41, 6, 34, 33], 9),
    ];
    for (n, m, h_bars, v_bars, exp) in test_cases {
        println!(
            "{} exp {exp}",
            Solution::maximize_square_hole_area(n, m, h_bars, v_bars)
        );
    }
}

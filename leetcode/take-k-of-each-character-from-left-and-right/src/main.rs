//https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/description/
pub struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }
        let s = s.as_bytes();
        let prefixes = std::iter::once((0, 0, 0))
            .chain(s.iter().scan((0, 0, 0), |acc, &letter| {
                match letter {
                    b'a' => acc.0 += 1,
                    b'b' => acc.1 += 1,
                    b'c' => acc.2 += 1,
                    _ => unreachable!(),
                }
                Some(*acc)
            }))
            .collect::<Vec<_>>();
        let suffixes = std::iter::once((0, 0, 0))
            .chain(s.iter().rev().scan((0, 0, 0), |acc, &letter| {
                match letter {
                    b'a' => acc.0 += 1,
                    b'b' => acc.1 += 1,
                    b'c' => acc.2 += 1,
                    _ => unreachable!(),
                }
                Some(*acc)
            }))
            .collect::<Vec<_>>();

        fn can_take_n(
            solution: usize,
            prefixes: &[(i32, i32, i32)],
            suffixes: &[(i32, i32, i32)],
            min: i32,
        ) -> bool {
            for taken_from_left in 0..=solution {
                let taken_from_right = solution - taken_from_left;
                if prefixes[taken_from_left].0 + suffixes[taken_from_right].0 >= min
                    && prefixes[taken_from_left].1 + suffixes[taken_from_right].1 >= min
                    && prefixes[taken_from_left].2 + suffixes[taken_from_right].2 >= min
                {
                    return true;
                }
            }
            false
        }
        let solution_space = (1..=s.len()).rev().collect::<Vec<_>>();
        let pp = solution_space
            .partition_point(|&solution| can_take_n(solution, &prefixes, &suffixes, k));
        if pp == 0 {
            -1
        } else {
            solution_space[pp - 1] as _
        }
    }
}

fn main() {
    println!("Hello, world!");
}

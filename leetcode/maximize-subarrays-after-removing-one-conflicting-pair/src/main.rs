// https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/description/?envType=daily-question&envId=2025-07-26
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_subarrays(n: i32, mut conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        let mut answer = 0;

        conflicting_pairs.iter_mut().for_each(|v| v.sort_unstable());
        conflicting_pairs.sort_unstable_by_key(|v| v[1]);
        let pairs_n = conflicting_pairs.len();

        let mut first_it = 0usize;
        let mut second_it = 0usize;

        let mut deletion_gains = vec![0; pairs_n];

        for i in 1..=n as i64 {
            while first_it < pairs_n && (conflicting_pairs[first_it][0] as i64) < i {
                first_it += 1;
            }

            while (second_it <= first_it && second_it < pairs_n)
                || (second_it < pairs_n && (conflicting_pairs[second_it][0] as i64) < i)
            {
                second_it += 1;
            }

            let end = if first_it < pairs_n {
                conflicting_pairs[first_it][1] as i64
            } else {
                n as i64 + 1
            };

            answer += end - i;

            let deletion_gain = if second_it < pairs_n {
                conflicting_pairs[second_it][1] as i64 - end
            } else {
                n as i64 + 1 - end
            };

            if first_it < pairs_n {
                deletion_gains[first_it] += deletion_gain;
            }
        }

        answer + deletion_gains.into_iter().max().unwrap()
    }
}

fn main() {
    let test_cases = [
        (4, vec![vec![2, 3], vec![1, 4]], 9),
        (5, vec![vec![1, 2], vec![2, 5], vec![3, 5]], 12),
    ];
    for (n, conflicting_pairs, exp) in test_cases {
        println!(
            "{}, exp: {exp}",
            Solution::max_subarrays(n, conflicting_pairs)
        );
    }
}

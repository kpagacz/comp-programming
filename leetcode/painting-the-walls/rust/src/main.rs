// https://leetcode.com/problems/painting-the-walls/description/
pub struct Solution {}

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        // mem[a][b]: min cost to paint walls starting from i-th wall with b remaining walls to paint
        let n = cost.len();
        let mut mem = vec![vec![-1; n + 1]; n];

        fn rec(
            i: usize,
            remain: i32,
            cost: &Vec<i32>,
            time: &Vec<i32>,
            mem: &mut Vec<Vec<i32>>,
            n: usize,
        ) -> i32 {
            if remain <= 0 {
                return 0;
            }
            if i == n {
                return i32::MAX;
            }
            if mem[i][remain as usize] != -1 {
                return mem[i][remain as usize];
            }

            let take = cost[i].saturating_add(rec(i + 1, remain - time[i] - 1, cost, time, mem, n));
            let dont_take = rec(i + 1, remain, cost, time, mem, n);

            mem[i][remain as usize] = take.min(dont_take);
            mem[i][remain as usize]
        }

        rec(0, n as i32, &cost, &time, &mut mem, n)
    }
}

fn main() {
    let test_cases = vec![
        (vec![1, 2, 3, 2], vec![1, 2, 3, 2], 3),
        (vec![2, 3, 4, 2], vec![1, 1, 1, 1], 4),
        (vec![2, 3, 4, 2, 2], vec![1, 1, 5, 1, 1], 4),
        (vec![8, 7, 5, 15], vec![1, 1, 2, 1], 12),
        (
            vec![42, 8, 28, 35, 21, 13, 21, 35],
            vec![2, 1, 1, 1, 2, 1, 1, 2],
            63,
        ),
    ];
    for (cost, time, expected) in test_cases {
        println!(
            "res: {} expected: {}",
            Solution::paint_walls(cost, time),
            expected
        );
    }
}

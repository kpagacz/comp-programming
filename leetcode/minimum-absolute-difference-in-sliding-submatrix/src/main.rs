// https://leetcode.com/problems/minimum-absolute-difference-in-sliding-submatrix/description/?envType=daily-question&envId=2026-03-20
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        use std::collections::BTreeMap;
        type Frequency = BTreeMap<i32, i32>;
        let mut freqs = Frequency::new();

        let min_diff = |freqs: &Frequency| {
            if freqs.len() == 1 {
                return 0;
            }
            let mut it = freqs.keys();
            let mut prev: i32 = *it.next().expect("There is at least one val");
            let mut min_diff = i32::MAX;
            for next in it {
                min_diff = min_diff.min((prev - *next).abs());
                prev = *next;
            }
            min_diff
        };
        let add_num = |num: i32, freqs: &mut Frequency| {
            freqs
                .entry(num)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        };
        let remove_num = |num: i32, freqs: &mut Frequency| {
            let entry = freqs.entry(num).or_insert(0);
            *entry -= 1;
            if *entry == 0 {
                freqs.remove(&num);
            }
        };

        let mut answer = vec![vec![0; grid[0].len() - k + 1]; grid.len() - k + 1];

        for (row, answer_row) in answer.iter_mut().enumerate().take(grid.len() - k + 1) {
            // Add row..row + k x 0..k submatrix to freqs
            grid.iter()
                .skip(row)
                .take(k)
                .for_each(|row| row.iter().take(k).for_each(|val| add_num(*val, &mut freqs)));
            // Calculate the min diff and assign to answer[row][0]
            answer_row[0] = min_diff(&freqs);
            for col in 1..=grid[0].len() - k {
                // Remove the (col - 1)-th column from the freqs
                for row_it in grid.iter().skip(row).take(k) {
                    remove_num(row_it[col - 1], &mut freqs);
                }
                // Add the (col + k - 1)-th column to the freqs
                for row_it in grid.iter().skip(row).take(k) {
                    add_num(row_it[col + k - 1], &mut freqs);
                }
                // Calculate the min diff and assign to answer[row][col]
                answer_row[col] = min_diff(&freqs);
            }
            freqs.clear();
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (
            vec![vec![1, 8], vec![3, -2], vec![3, -2]],
            2,
            vec![vec![2], vec![5]],
        ),
        (vec![vec![1, -2, 3], vec![2, 3, 5]], 2, vec![vec![1, 2]]),
    ];

    for (grid, k, exp) in test_cases {
        println!("{:?} exp: {exp:?}", Solution::min_abs_diff(grid, k));
    }
}

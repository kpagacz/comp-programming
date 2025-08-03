// https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/description/?envType=daily-question&envId=2025-08-03
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let mut sum = 0;
        let mut max_sum = 0;
        let left = fruits.partition_point(|fruit_desc| fruit_desc[0] < start_pos - k);
        let mut left = if left == fruits.len() { left - 1 } else { left };
        let mut right = left;

        while right < fruits.len() {
            if fruits[right][0] > start_pos + k {
                break;
            }
            sum += fruits[right][1];
            while left <= right
                && fruits[right][0] - fruits[left][0]
                    + i32::min(
                        (start_pos - fruits[left][0]).abs(),
                        (start_pos - fruits[right][0]).abs(),
                    )
                    > k
            {
                sum -= fruits[left][1];
                left += 1;
            }
            max_sum = max_sum.max(sum);
            right += 1;
        }
        max_sum
    }
    pub fn max_total_fruits_old(mut fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        for i in 1..fruits.len() {
            fruits[i][1] += fruits[i - 1][1];
        }
        let count_fruits = |left_end: i32, right_end: i32| {
            let left_pos = fruits.partition_point(|fruit_desc| fruit_desc[0] < left_end);
            let right_pos = fruits.partition_point(|fruit_desc| fruit_desc[0] <= right_end);

            let left_fruits = if left_pos > 0 {
                fruits[left_pos - 1][1]
            } else {
                0
            };
            // println!("left: {left_pos} {left_fruits}");
            let right_fruits = if right_pos > 0 {
                fruits[right_pos - 1][1]
            } else {
                0
            };
            // println!("right: {right_pos} {right_fruits}");

            right_fruits - left_fruits
        };

        let mut max_fruits = 0;
        // println!("{fruits:?}");
        // Go right and backtrack
        for right_step in 0..=k {
            let right_end = start_pos + right_step;
            let left_end = start_pos.min(right_end - k + right_step);
            // println!("left_end: {left_end} right_end: {right_end}");

            max_fruits = max_fruits.max(count_fruits(left_end, right_end));
        }

        // Go left and backtrack
        for left_step in 0..=k {
            let left_end = start_pos - left_step;
            let right_end = start_pos.max(left_end + k - left_step);
            // println!("left_end: {left_end} right_end: {right_end}");
            max_fruits = max_fruits.max(count_fruits(left_end, right_end));
        }

        max_fruits
    }
}

fn main() {
    let test_cases = [
        (vec![vec![2, 8], vec![6, 3], vec![8, 6]], 5, 4, 9),
        (
            vec![
                vec![0, 9],
                vec![4, 1],
                vec![5, 7],
                vec![6, 2],
                vec![7, 4],
                vec![10, 9],
            ],
            5,
            4,
            14,
        ),
    ];
    for (fruits, start_pos, k, exp) in test_cases {
        println!(
            "{}  exp: {exp}",
            Solution::max_total_fruits(fruits, start_pos, k)
        );
    }
}

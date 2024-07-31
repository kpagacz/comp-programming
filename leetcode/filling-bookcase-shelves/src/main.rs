// https://leetcode.com/problems/filling-bookcase-shelves/description/
pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        dp[1] = books[0][1];

        for i in 1..=n {
            let (new_width, new_height) = (books[i - 1][0], books[i - 1][1]);

            // Try adding to a new shelf with variable number of previous books
            let mut remaining_width = shelf_width - new_width;
            let mut current_max = new_height;

            dp[i] = dp[i - 1] + new_height;

            for it in (1..i).rev() {
                if remaining_width < books[it - 1][0] {
                    break;
                }
                remaining_width -= books[it - 1][0];
                current_max = i32::max(current_max, books[it - 1][1]);
                if current_max + dp[it - 1] < dp[i] {
                    dp[i] = current_max + dp[it - 1];
                }
            }
        }

        dp[n]
    }
}

fn main() {
    println!("Hello, world!");
}

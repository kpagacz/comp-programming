// https://leetcode.com/problems/snakes-and-ladders/description/?envType=daily-question&envId=2025-05-31
pub struct Solution;

impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        use std::collections::VecDeque;

        fn find_position_on_board(position: usize, board_size: usize) -> (usize, usize) {
            let row_from_bottom = (position - 1) / board_size;
            let col = if row_from_bottom % 2 == 0 {
                (position - 1) % board_size
            } else {
                (board_size - 1) - (position - 1) % board_size
            };
            (board_size - row_from_bottom - 1, col)
        }

        let mut queue = VecDeque::new();
        queue.push_back((1, 0));
        let mut visited = vec![false; board.len() * board.len() + 1];

        while let Some((position, throw)) = queue.pop_front() {
            if position == board.len() * board.len() {
                return throw;
            }
            for result in 1..=6 {
                let mut new_position = position + result;
                if new_position > board.len() * board.len() {
                    break;
                }
                let (row, col) = find_position_on_board(new_position, board.len());
                if board[row][col] != -1 {
                    new_position = board[row][col] as usize;
                }
                if visited[new_position] {
                    continue;
                }
                visited[new_position] = true;
                queue.push_back((new_position, throw + 1));
            }
        }
        -1
    }
}

fn main() {
    println!("Hello, world!");
}

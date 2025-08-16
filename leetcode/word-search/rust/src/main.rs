// https://leetcode.com/problems/word-search/description/
pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Solution::exists_rec(&board, &word, 0, i, j, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    fn exists_rec(
        board: &[Vec<char>],
        word: &[char],
        it: usize,
        x: usize,
        y: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if visited[x][y] {
            return false;
        }
        if board[x][y] != word[it] {
            return false;
        }
        if it == word.len() - 1 {
            return true;
        }
        visited[x][y] = true;
        let res = Solution::neighbours(board, x, y)
            .into_iter()
            .map(|(new_x, new_y)| Solution::exists_rec(board, word, it + 1, new_x, new_y, visited))
            .any(|x| x);
        visited[x][y] = false;
        res
    }

    fn neighbours(board: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize)> {
        let (rows, cols) = (board.len(), board[0].len());
        let mut neighbours = vec![];
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < rows - 1 {
            neighbours.push((x + 1, y));
        }
        if y < cols - 1 {
            neighbours.push((x, y + 1));
        }

        neighbours
    }
}

fn main() {
    println!("Hello, world!");
}

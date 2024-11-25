// https://leetcode.com/problems/sliding-puzzle/description/
pub struct Solution;

impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashSet;

        let mut states = BinaryHeap::default();
        let mut seen: HashSet<Vec<Vec<i32>>> = HashSet::default();
        states.push(Reverse((0, board)));

        while let Some(Reverse((moves, state))) = states.pop() {
            if seen.contains(&state) {
                continue;
            }
            if Solution::is_solved(&state) {
                return moves;
            }
            seen.insert(state.clone());

            Solution::generate_moves(&state)
                .into_iter()
                .filter(|state| !seen.contains(state))
                .for_each(|state| states.push(Reverse((moves + 1, state))));
        }

        -1
    }

    fn is_solved(state: &[Vec<i32>]) -> bool {
        state == vec![vec![1, 2, 3], vec![4, 5, 0]]
    }

    fn generate_moves(state: &[Vec<i32>]) -> Vec<Vec<Vec<i32>>> {
        let mut x: usize = 0;
        let mut y: usize = 0;

        'outer: for row in 0..2 {
            for col in 0..3 {
                if state[row][col] == 0 {
                    x = row;
                    y = col;
                    break 'outer;
                }
            }
        }

        let mut moved_states = vec![];

        if x + 1 < 2 {
            let mut new_state = state.to_owned();
            new_state[x][y] = new_state[x + 1][y];
            new_state[x + 1][y] = 0;
            moved_states.push(new_state);
        }
        if x > 0 {
            let mut new_state = state.to_owned();
            new_state[x][y] = new_state[x - 1][y];
            new_state[x - 1][y] = 0;
            moved_states.push(new_state);
        }
        if y + 1 < 3 {
            let mut new_state = state.to_owned();
            new_state[x][y] = new_state[x][y + 1];
            new_state[x][y + 1] = 0;
            moved_states.push(new_state);
        }
        if y > 0 {
            let mut new_state = state.to_owned();
            new_state[x][y] = new_state[x][y - 1];
            new_state[x][y - 1] = 0;
            moved_states.push(new_state);
        }

        moved_states
    }
}

fn main() {
    println!("Hello, world!");
}

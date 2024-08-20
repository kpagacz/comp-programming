// https://leetcode.com/problems/stone-game-ii/description/
pub struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut mem = vec![vec![-1; 200]; piles.len()];
        Self::rec(0, &piles, 1, piles.iter().sum::<i32>(), &mut mem)
    }

    fn rec(pos: usize, piles: &[i32], m: usize, stones_left: i32, mem: &mut [Vec<i32>]) -> i32 {
        if pos >= piles.len() {
            return 0;
        }
        if mem[pos][m] != -1 {
            return mem[pos][m];
        }

        let end = piles.len().min(pos + 2 * m);
        mem[pos][m] = (pos + 1..=end)
            .map(|first_non_taken_pile| {
                let stones_taken = piles[pos..first_non_taken_pile].iter().sum::<i32>();
                let piles_taken = first_non_taken_pile - pos;

                stones_left
                    - Self::rec(
                        first_non_taken_pile,
                        piles,
                        m.max(piles_taken),
                        stones_left - stones_taken,
                        mem,
                    )
            })
            .max()
            .unwrap();
        mem[pos][m]
    }
}

fn main() {
    let test_cases = [vec![2, 7, 9, 4, 4]];

    for piles in test_cases {
        println!("{}", Solution::stone_game_ii(piles));
    }
}

// https://leetcode.com/problems/remove-colored-pieces-if-both-neighbors-are-the-same-color/description/
pub struct Solution {}

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        let mut a_score = 0;
        let mut b_score = 0;
        let colors: Vec<u8> = colors.bytes().collect();
        let a_byte = 'A' as u8;

        let mut beginning = 0;
        for i in 0..colors.len() {
            if colors[i] == colors[beginning] {
                match (i - beginning >= 2, a_byte == colors[beginning]) {
                    (false, _) => {}
                    (true, true) => a_score += 1,
                    (true, false) => b_score += 1,
                }
            } else {
                beginning = i;
            }
        }

        a_score > b_score
    }

    // Eh, sliding window...
    pub fn winner_of_game2(colors: String) -> bool {
        let colors: Vec<u8> = colors.bytes().collect();
        let a_byte = 'A' as u8;

        colors
            .windows(3)
            .map(|elems| {
                match (
                    elems[0] == elems[1],
                    elems[1] == elems[2],
                    elems[0] == a_byte,
                ) {
                    (false, _, _) => 0,
                    (_, false, _) => 0,
                    (true, true, true) => 1,
                    (true, true, false) => -1,
                }
            })
            .sum::<i32>()
            > 0
    }
}
fn main() {
    println!("{}", Solution::winner_of_game("AAABBABB".to_owned()));
}

// https://leetcode.com/problems/check-if-two-chessboard-squares-have-the-same-color/description/
pub struct Solution;

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let coordinate1 = coordinate1.as_bytes();
        let coordinate2 = coordinate2.as_bytes();

        (coordinate1[0] - b'a' + coordinate1[1] - b'1') % 2
            == (coordinate2[0] - b'a' + coordinate2[1] - b'1') % 2
    }
}

fn main() {
    println!("Hello, world!");
}

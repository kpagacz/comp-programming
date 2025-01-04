// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/description/
pub struct Solution {}

// There is of course a naive implementation that takes quadratic time.
// This is a linear solution with a hefty constant (26), and I wonder if
// there is a linear solution with a smaller one.
// I can sort of imagine how it would work - remembering the indices of what
// characters went before. Maybe BIT? You can query the number of different characters
// between the current position and the first position that the current character
// was encountered.
// This ultimately would nlogn, but it might be faster considering the high constant.
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        use std::collections::BTreeMap;
        let s: Vec<_> = s.bytes().collect();

        let mut positions = BTreeMap::new();
        s.iter().enumerate().for_each(|(pos, &c)| {
            let entry = positions.entry(c).or_insert((-1i32, -1i32));
            match (entry.0 == -1, entry.1 == -1) {
                (true, true) => entry.0 = pos as i32,
                (false, _) => entry.1 = pos as i32,
                (_, _) => {}
            }
        });

        let mut answer = 0;
        positions.values().for_each(|&(first, last)| {
            if first != -1 && last != -1 {
                let mut middle = vec![false; 30];
                for &c in &s[(first + 1) as usize..last as usize] {
                    middle[(c - b'a') as usize] = true;
                }
                answer += middle.into_iter().filter(|&el| el).count();
            }
        });
        answer as i32
    }
}
fn main() {
    println!("Hello, world!");
}

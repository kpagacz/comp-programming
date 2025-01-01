// https://leetcode.com/problems/find-the-lexicographically-largest-string-from-the-box-i/description/
pub struct Solution;

impl Solution {
    pub fn answer_string(word: String, num_friends: i32) -> String {
        if num_friends == 1 {
            return word;
        }
        let mut answer = word[0..1].to_string();
        let size = word.len() - num_friends as usize + 1;

        for start in 0..word.len() {
            let end = (start + size).min(word.len());
            if word[start..end] > answer[..] {
                answer = word[start..end].to_string();
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

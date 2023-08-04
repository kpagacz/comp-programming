// https://leetcode.com/problems/word-break/description/
pub struct Solution {}
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        use std::iter::FromIterator;
        let mut mem = std::collections::HashMap::new();
        let dict = std::collections::HashSet::from_iter(word_dict);
        Self::word_break_backtrack(0, 0, &dict, &s, &mut mem)
    }

    fn word_break_backtrack(
        save_point: usize,
        current: usize,
        word_dict: &std::collections::HashSet<String>,
        word: &str,
        mem: &mut std::collections::HashMap<usize, bool>,
    ) -> bool {
        if current == word.len() {
            return save_point == word.len();
        }
        if mem.contains_key(&save_point) {
            return mem[&save_point];
        }

        let res = if word_dict.contains(&word[save_point..=current]) {
            Self::word_break_backtrack(save_point, current + 1, word_dict, word, mem)
                || Self::word_break_backtrack(current + 1, current + 1, word_dict, word, mem)
        } else {
            Self::word_break_backtrack(save_point, current + 1, word_dict, word, mem)
        };

        mem.entry(save_point).or_insert(res);
        res
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/sum-of-prefix-scores-of-strings/description/
pub struct Solution;

use std::collections::HashMap;
#[derive(Clone, Debug)]
struct TrieNode {
    leaf_count: usize,
    children: HashMap<u8, TrieNode>,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            leaf_count: 0,
        }
    }

    fn add_word(&mut self, word: &str) {
        fn inner(node: &mut TrieNode, word: &[u8]) {
            if word.is_empty() {
                node.leaf_count += 1;
                return;
            }
            node.leaf_count += 1;
            let first = *word.first().unwrap() - b'a';

            inner(
                node.children.entry(first).or_insert(TrieNode::new()),
                &word[1..],
            );
        }

        let word = word.as_bytes();
        inner(self, word);
    }

    fn score_prefixes(&self, word: &str) -> i32 {
        fn inner(node: &TrieNode, word: &[u8]) -> usize {
            if word.is_empty() {
                return node.leaf_count;
            }

            let first = *word.first().unwrap() - b'a';
            node.leaf_count + inner(node.children.get(&first).unwrap(), &word[1..])
        }

        let word = word.as_bytes();
        inner(self, word) as i32 - self.leaf_count as i32
    }
}

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut trie = TrieNode::new();

        for word in &words {
            trie.add_word(word);
        }

        words.iter().map(|word| trie.score_prefixes(word)).collect()
    }
}

fn main() {
    let test_cases = [vec!["abc", "ab", "bc", "b"]];

    for words in test_cases {
        println!(
            "{:?}",
            Solution::sum_prefix_scores(words.into_iter().map(str::to_string).collect())
        );
    }
}

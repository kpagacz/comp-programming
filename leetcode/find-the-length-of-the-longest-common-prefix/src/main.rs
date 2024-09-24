// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/description/
pub struct Solution;

#[derive(Clone)]
struct TrieNode(Vec<Option<TrieNode>>);

impl TrieNode {
    fn new() -> Self {
        Self(vec![None; 10])
    }

    fn add(&mut self, val: i32) {
        let val = val.to_string();
        let val = val.as_bytes();

        self._add(val);
    }

    fn _add(&mut self, val: &[u8]) {
        if val.is_empty() {
            return;
        }
        let next = (*val.first().unwrap() - b'0') as usize;
        if let Some(ref mut child) = self.0[next] {
            Self::_add(child, &val[1..]);
        } else {
            self.0[next] = Some(TrieNode::new());
            Self::_add(self.0[next].as_mut().unwrap(), &val[1..]);
        }
    }

    fn find_common_prefix_length(&self, val: i32) -> usize {
        let val = val.to_string();
        let val = val.as_bytes();

        fn rec(node: &TrieNode, depth: usize, val: &[u8]) -> usize {
            if val.is_empty() {
                return depth;
            }

            let first = (*val.first().unwrap() - b'0') as usize;
            if let Some(ref node) = node.0[first] {
                rec(node, depth + 1, &val[1..])
            } else {
                depth
            }
        }

        rec(self, 0, val)
    }
}

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = TrieNode::new();

        for num in arr1 {
            trie.add(num);
        }

        let mut longest = 0;
        for num in arr2 {
            longest = longest.max(trie.find_common_prefix_length(num) as i32);
        }
        longest
    }
}

fn main() {
    println!("Hello, world!");
}

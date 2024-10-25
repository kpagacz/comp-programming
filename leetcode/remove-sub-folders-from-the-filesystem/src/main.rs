// https://leetcode.com/problems/remove-sub-folders-from-the-filesystem/description/
pub struct Solution;

use std::collections::HashMap;
#[derive(Default, Debug, Clone)]
struct Trie {
    end: bool,
    children: HashMap<String, Trie>,
}

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        folder.sort_unstable();

        let mut trie = Trie::default();
        folder.retain(|folder| {
            if Self::is_subfolder(&trie, folder) {
                false
            } else {
                Self::add(&mut trie, folder);
                true
            }
        });
        folder
    }

    fn add(trie: &mut Trie, path: &str) {
        let path = path.split('/').skip(1);

        let mut trie_it = trie;
        for dir in path {
            if !trie_it.children.contains_key(dir) {
                trie_it.children.insert(dir.to_string(), Trie::default());
            }
            trie_it = trie_it.children.get_mut(dir).unwrap();
        }
        trie_it.end = true;
    }

    fn is_subfolder(trie: &Trie, path: &str) -> bool {
        let path = path.split('/').skip(1);
        let mut trie_it = trie;
        for dir in path {
            if trie_it.children.contains_key(dir) {
                trie_it = trie_it.children.get(dir).unwrap();
                if trie_it.end {
                    return true;
                }
            } else {
                return false;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}

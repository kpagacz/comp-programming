// https://leetcode.com/problems/delete-duplicate-folders-in-system/description/?envType=daily-question&envId=2025-07-20
struct Solution;

use std::collections::{BTreeMap, HashMap};
use std::hash::{Hash, Hasher};

type Map = BTreeMap<String, Node>;
#[derive(Debug)]
struct Node {
    val: String,
    hash: Option<u64>,
    children: Map,
}

impl Node {
    fn new(val: String) -> Self {
        Self {
            val,
            hash: None,
            children: Map::default(),
        }
    }

    fn add_path(&mut self, path: Vec<String>) {
        let mut it = self;
        for el in path {
            let entry = it.children.entry(el.clone()).or_insert(Node::new(el));
            it = entry;
        }
    }

    fn hash_node(&mut self) -> u64 {
        let res = if let Some(hash) = self.hash {
            hash
        } else {
            if self.children.is_empty() {
                self.hash = Some(0);
            } else {
                let mut s = std::hash::DefaultHasher::new();
                self.children.values_mut().for_each(|child| {
                    child.hash_node().hash(&mut s);
                    child.val.hash(&mut s);
                });
                self.hash = Some(s.finish());
            }
            self.hash.unwrap()
        };
        // println!("Node: {self:?} res: {res}");
        res
    }

    fn visit(&self, f: &mut impl FnMut(&Self)) {
        f(self);
        self.children.values().for_each(|child| child.visit(f));
    }

    fn visit_mut(&mut self, f: &mut impl FnMut(&mut Self)) {
        f(self);
        self.children
            .values_mut()
            .for_each(|child| child.visit_mut(f));
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut trie = Node::new("".to_string());
        for path in paths {
            trie.add_path(path);
        }

        // Get hashes
        let _ = trie.hash_node();
        let mut hashes = HashMap::new();
        let mut map_hashes = |node: &Node| {
            hashes
                .entry(node.hash.unwrap())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        };
        trie.visit(&mut map_hashes);

        // Delete
        let mut delete_duplicated = |node: &mut Node| {
            node.children.retain(|_, node| {
                node.hash.unwrap() == 0 || hashes.get(&node.hash.unwrap()).unwrap() < &2
            });
        };
        trie.visit_mut(&mut delete_duplicated);

        // Reconstruct paths
        let mut paths = Vec::new();
        fn rec(node: &Node, mut path: Vec<String>, paths: &mut Vec<Vec<String>>) {
            path.push(node.val.clone());
            paths.push(path.clone());
            node.children
                .values()
                .for_each(|child| rec(child, path.clone(), paths));
        }
        trie.children
            .values()
            .for_each(|node| rec(node, vec![], &mut paths));
        paths
    }
}

fn main() {
    let test_cases = [
        vec![
            vec!["a"],
            vec!["c"],
            vec!["d"],
            vec!["a", "b"],
            vec!["c", "b"],
            vec!["d", "a"],
        ],
        vec![
            vec!["a"],
            vec!["c"],
            vec!["a", "b"],
            vec!["c", "b"],
            vec!["a", "b", "x"],
            vec!["a", "b", "x", "y"],
            vec!["w"],
            vec!["w", "y"],
        ],
        vec![vec!["a", "b"], vec!["c", "d"], vec!["c"], vec!["a"]],
    ];
    for paths in test_cases {
        let paths = paths
            .into_iter()
            .map(|path| path.into_iter().map(|s| s.to_owned()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        println!("{:?}", Solution::delete_duplicate_folder(paths));
    }
}

// https://leetcode.com/problems/all-nodes-distance-k-in-binary-tree/

pub struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let parents = Self::find_parents(&root);
        let mut nodes_at_k = vec![];
        let mut visited = vec![false; 501];
        if let Some(target) = &target {
            Self::bfs(target, 0, &mut nodes_at_k, &mut visited, k, &parents);
        }
        nodes_at_k
    }

    fn bfs(
        node: &Rc<RefCell<TreeNode>>,
        distance: i32,
        nodes_at_k: &mut Vec<i32>,
        visited: &mut Vec<bool>,
        k: i32,
        parents: &HashMap<i32, Rc<RefCell<TreeNode>>>,
    ) {
        if visited[node.borrow().val as usize] {
            return;
        }
        visited[node.borrow().val as usize] = true;

        if distance == k {
            nodes_at_k.push(node.borrow().val);
            return;
        }

        if let Some(child) = &node.borrow().left {
            Self::bfs(child, distance + 1, nodes_at_k, visited, k, parents);
        }
        if let Some(child) = &node.borrow().right {
            Self::bfs(child, distance + 1, nodes_at_k, visited, k, parents);
        }
        if let Some(parent) = parents.get(&node.borrow().val) {
            Self::bfs(parent, distance + 1, nodes_at_k, visited, k, parents);
        }
    }

    fn find_parents(root: &Option<Rc<RefCell<TreeNode>>>) -> HashMap<i32, Rc<RefCell<TreeNode>>> {
        let mut parents_map = HashMap::new();
        if let Some(root) = root {
            Self::find_parents_rec(root, &mut parents_map);
        }
        parents_map
    }

    fn find_parents_rec(
        parent: &Rc<RefCell<TreeNode>>,
        parents_map: &mut HashMap<i32, Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(child) = &parent.borrow().left {
            parents_map.insert(child.borrow().val, parent.clone());
            Self::find_parents_rec(child, parents_map);
        }
        if let Some(child) = &parent.borrow().right {
            parents_map.insert(child.borrow().val, parent.clone());
            Self::find_parents_rec(child, parents_map);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/create-binary-tree-from-descriptions/description/
pub struct Solution;

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
use std::rc::Rc;
impl Solution {
    // TLE
    pub fn create_binary_tree_tle(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        use std::collections::HashMap;

        let mut map = HashMap::new(); // parent -> (left child, right child)
        for description in &descriptions {
            let (parent, child, is_left) = (description[0], description[1], description[2]);
            if is_left == 1 {
                map.entry(parent).or_insert((-1, -1)).0 = child;
            } else {
                map.entry(parent).or_insert((-1, -1)).1 = child;
            }
        }

        dbg!(&map);

        let mut is_child = map
            .keys()
            .map(|key| (key, false))
            .collect::<HashMap<_, _>>();
        for children in map.values() {
            is_child.insert(&children.0, true);
            is_child.insert(&children.1, true);
        }
        is_child.retain(|_, is_child| !(*is_child));
        let root = **is_child.keys().next().unwrap();

        fn build_tree(node: i32, map: &HashMap<i32, (i32, i32)>) -> Option<Rc<RefCell<TreeNode>>> {
            if node == -1 {
                None
            } else {
                let mut new_node = TreeNode::new(node);
                new_node.left = build_tree(map.get(&node).unwrap_or(&(-1, -1)).0, map);
                new_node.right = build_tree(map.get(&node).unwrap_or(&(-1, -1)).1, map);
                Some(Rc::new(RefCell::new(new_node)))
            }
        }

        build_tree(root, &map)
    }

    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut children = vec![(-1, -1); 100_001];
        let mut is_child = vec![false; 100_001];
        let mut parents = vec![];

        for description in descriptions {
            let (parent, child, is_left) = (description[0], description[1], description[2]);
            if is_left == 1 {
                children[parent as usize].0 = child;
            } else {
                children[parent as usize].1 = child;
            }
            is_child[child as usize] = true;
            parents.push(parent);
        }

        let root = *parents
            .iter()
            .find(|&&parent| !is_child[parent as usize])
            .unwrap();

        fn build_tree(node: i32, children: &[(i32, i32)]) -> Option<Rc<RefCell<TreeNode>>> {
            if node == -1 {
                None
            } else {
                let mut new_node = TreeNode::new(node);
                new_node.left = build_tree(children[node as usize].0, children);
                new_node.right = build_tree(children[node as usize].1, children);
                Some(Rc::new(RefCell::new(new_node)))
            }
        }

        build_tree(root, &children)
    }
}

fn main() {
    println!("Hello, world!");
}

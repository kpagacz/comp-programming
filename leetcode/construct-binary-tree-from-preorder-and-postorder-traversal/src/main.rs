// https://leetcode.com/problems/construct-binary-tree-from-preorder-and-postorder-traversal/
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
    pub fn construct_from_pre_post(
        preorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        construct(&preorder[..], &postorder[..])
    }
}

fn construct(pre: &[i32], post: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    match pre.len() {
        0 => None,
        1 => Some(Rc::new(RefCell::new(TreeNode::new(pre[0])))),
        n => {
            let root = pre[0];
            let left = pre[1];

            let left_size = post.iter().position(|val| *val == left).unwrap();

            let mut root = TreeNode::new(root);
            root.left = construct(&pre[1..=left_size + 1], &post[0..=left_size]);
            root.right = construct(&pre[left_size + 2..], &post[left_size + 1..n - 1]);
            Some(Rc::new(RefCell::new(root)))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

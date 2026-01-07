// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/description/?envType=daily-question&envId=2026-01-07
struct Solution;

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
#[allow(dead_code)]
impl Solution {
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        use std::collections::HashSet;
        fn rec_subtree_sums(node: &Option<Rc<RefCell<TreeNode>>>, sums: &mut HashSet<i64>) -> i64 {
            if let Some(node) = node {
                let borrow = node.borrow();
                let (left, right) = (
                    rec_subtree_sums(&borrow.left, sums),
                    rec_subtree_sums(&borrow.right, sums),
                );
                sums.insert(left + right + borrow.val as i64);
                left + right + borrow.val as i64
            } else {
                0
            }
        }

        let mut subtree_sums = HashSet::new();
        let root_sum = rec_subtree_sums(&root, &mut subtree_sums);
        let mut max_product = i64::MIN;
        for subtree_sum in subtree_sums {
            max_product = max_product.max((root_sum - subtree_sum) * subtree_sum);
        }

        const MOD: i64 = 10i64.pow(9) + 7;
        (max_product % MOD) as _
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries/description/
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
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let parents = Solution::find_parents(&root);
        let depths = Solution::find_depths(&root);
        let heights = Solution::find_heights(&root);
        let nodes = Solution::map_nodes(&root);

        // Plan:
        // depths is the 0-indexed level
        // height is the maximum number of vertices from the root to the deepest subtree
        // root has parent 0
        //
        // Get the node, check if depth + height == root height. If yes, then:
        // iterate through ancestors finding the max of the children without the removed subtree
        fn handle_query(
            query: i32,
            parents: &[usize],
            depths: &[usize],
            heights: &[usize],
            nodes: &[Option<Rc<RefCell<TreeNode>>>],
            root_height: usize,
        ) -> i32 {
            let query = query as usize;
            if depths[query] + heights[query] == root_height {
                let mut it = parents[query];
                let mut child = query;
                let mut max_height = depths[it] + 1;

                while it != 0 {
                    match (
                        Solution::left_child(it, nodes),
                        Solution::right_child(it, nodes),
                    ) {
                        (None, None) => unreachable!("We have come from somewhere!"),
                        (Some(_), None) | (None, Some(_)) => {}
                        (Some(left_child), Some(right_child)) => {
                            if left_child == child {
                                // We have come from the left child
                                max_height =
                                    max_height.max(depths[right_child] + heights[right_child]);
                            } else {
                                // We have come from the right child
                                max_height =
                                    max_height.max(depths[left_child] + heights[left_child]);
                            }
                        }
                    }

                    child = it;
                    it = parents[it];
                }
                max_height as _
            } else {
                root_height as _
            }
        }

        let root = root.as_ref().unwrap().borrow().val as usize;
        let root_height = heights[root];
        queries
            .into_iter()
            .map(|query| handle_query(query, &parents, &depths, &heights, &nodes, root_height) - 1)
            .collect()
    }

    // Fact finding
    fn find_parents(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<usize> {
        let mut parents = vec![0; 100_001];
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, parents: &mut [usize], parent: usize) {
            if let Some(node) = node {
                let borr = node.borrow();
                parents[borr.val as usize] = parent;
                rec(&borr.left, parents, borr.val as usize);
                rec(&borr.right, parents, borr.val as usize);
            }
        }
        rec(node, &mut parents, 0);
        parents
    }
    fn find_depths(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<usize> {
        let mut depths = vec![0; 100_001];
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, depths: &mut [usize], depth: usize) {
            if let Some(node) = node {
                let borr = node.borrow();
                depths[borr.val as usize] = depth;
                rec(&borr.left, depths, depth + 1);
                rec(&borr.right, depths, depth + 1);
            }
        }
        rec(node, &mut depths, 0);
        depths
    }
    fn find_heights(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<usize> {
        let mut heights = vec![0; 100_001];
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, heights: &mut [usize]) -> usize {
            if let Some(node) = node {
                let borr = node.borrow();
                let height = rec(&borr.left, heights).max(rec(&borr.right, heights)) + 1;
                heights[borr.val as usize] = height;
                height
            } else {
                0
            }
        }
        rec(node, &mut heights);
        heights
    }
    fn map_nodes(node: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut nodes = vec![None; 100_001];
        fn rec(node: &Option<Rc<RefCell<TreeNode>>>, nodes: &mut [Option<Rc<RefCell<TreeNode>>>]) {
            if let Some(real) = node {
                nodes[real.borrow().val as usize] = Some(Rc::clone(real));
                rec(&real.borrow().left, nodes);
                rec(&real.borrow().right, nodes);
            }
        }
        rec(node, &mut nodes);
        nodes
    }

    // Child getting
    fn left_child(node: usize, nodes: &[Option<Rc<RefCell<TreeNode>>>]) -> Option<usize> {
        nodes[node]
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .map(|left_child| left_child.borrow().val as usize)
    }
    fn right_child(node: usize, nodes: &[Option<Rc<RefCell<TreeNode>>>]) -> Option<usize> {
        nodes[node]
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .map(|right_child| right_child.borrow().val as usize)
    }
}

fn main() {
    println!("Hello, world!");
}

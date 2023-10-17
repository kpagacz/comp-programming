// https://leetcode.com/problems/validate-binary-tree-nodes/description/
pub struct Solution {}
impl Solution {
    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let n = n as usize;
        let mut not_pointed_to = vec![true; n];
        left_child
            .iter()
            .filter(|&&child| child != -1)
            .for_each(|child| not_pointed_to[*child as usize] = false);
        right_child
            .iter()
            .filter(|&&child| child != -1)
            .for_each(|child| not_pointed_to[*child as usize] = false);
        if not_pointed_to
            .iter()
            .filter(|&&not_pointed| not_pointed == true)
            .count()
            != 1
        {
            return false;
        }
        let root = not_pointed_to
            .iter()
            .enumerate()
            .find(|(_, &not_pointed)| not_pointed == true)
            .unwrap()
            .0;

        fn mark_visited(
            id: usize,
            left_child: &Vec<i32>,
            right_child: &Vec<i32>,
            visited: &mut Vec<bool>,
        ) -> bool {
            if visited[id] {
                return false;
            }
            visited[id] = true;
            let mut is_good = true;
            if left_child[id] != -1 {
                is_good &= mark_visited(left_child[id] as usize, left_child, right_child, visited);
            }
            if right_child[id] != -1 {
                is_good &= mark_visited(right_child[id] as usize, left_child, right_child, visited);
            }
            is_good
        }
        let mut visited = vec![false; n];
        let is_good = mark_visited(root, &left_child, &right_child, &mut visited);
        is_good && visited.iter().all(|&marked| marked)
    }
}
fn main() {
    println!("Hello, world!");
}

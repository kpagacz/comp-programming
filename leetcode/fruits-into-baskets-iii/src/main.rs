mod better;
// https://leetcode.com/problems/fruits-into-baskets-iii/description/?envType=daily-question&envId=2025-08-06
struct Solution;

struct SegmentTree {
    arr: Vec<usize>,
}

impl SegmentTree {
    fn new(in_arr: &[usize]) -> Self {
        let mut tree = vec![0; in_arr.len() * 4];
        SegmentTree::build(in_arr, 1, 0, in_arr.len() - 1, &mut tree);
        Self { arr: tree }
    }

    fn build(in_arr: &[usize], node: usize, left: usize, right: usize, tree: &mut [usize]) {
        if left == right {
            tree[node] = in_arr[left];
        } else {
            let middle = (left + right) / 2;
            SegmentTree::build(in_arr, 2 * node, left, middle, tree);
            SegmentTree::build(in_arr, 2 * node + 1, middle + 1, right, tree);
            tree[node] = usize::min(tree[2 * node], tree[2 * node + 1]);
        }
    }

    fn query(&self, left: usize, right: usize) -> usize {
        fn rec(
            arr: &[usize],
            node: usize,
            tree_left: usize,
            tree_right: usize,
            left: usize,
            right: usize,
        ) -> usize {
            // println!(
            //     "node: {node} tree_left: {tree_left} tree_right: {tree_right} left: {left} right: {right}"
            // );
            if left > right {
                return usize::MAX;
            }
            if left == tree_left && right == tree_right {
                return arr[node];
            }

            let middle = (tree_left + tree_right) / 2;
            usize::min(
                rec(arr, node * 2, tree_left, middle, left, middle.min(right)),
                rec(
                    arr,
                    node * 2 + 1,
                    middle + 1,
                    tree_right,
                    (middle + 1).max(left),
                    right,
                ),
            )
        }

        rec(&self.arr, 1, 0, self.arr.len() / 4 - 1, left, right)
    }

    fn update(&mut self, position: usize, new_value: usize) {
        fn rec(
            arr: &mut [usize],
            node: usize,
            tree_left: usize,
            tree_right: usize,
            position: usize,
            new_value: usize,
        ) {
            if tree_left == tree_right {
                arr[node] = new_value;
            } else {
                let middle = (tree_left + tree_right) / 2;
                if position <= middle {
                    rec(arr, node * 2, tree_left, middle, position, new_value);
                } else {
                    rec(
                        arr,
                        node * 2 + 1,
                        middle + 1,
                        tree_right,
                        position,
                        new_value,
                    );
                }
                arr[node] = usize::min(arr[node * 2], arr[node * 2 + 1]);
            }
        }
        let n = self.arr.len() / 4;
        rec(&mut self.arr, 1, 0, n - 1, position, new_value);
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut sorted_baskets: Vec<usize> = (0..baskets.len()).collect();
        sorted_baskets.sort_by_key(|basket| baskets[*basket]);
        let mut enumerated: Vec<_> = sorted_baskets
            .iter()
            .enumerate()
            .map(|(id, basket)| (*basket, id))
            .collect();
        enumerated.sort_unstable();

        let mut seg_tree = SegmentTree::new(&sorted_baskets);

        let mut unplaced = 0;
        for quantity in fruits {
            let pp = sorted_baskets.partition_point(|basket| baskets[*basket] < quantity);
            let min_index = seg_tree.query(pp, sorted_baskets.len() - 1);
            if min_index != usize::MAX {
                let min_index_position_in_sorted = enumerated[min_index].1;
                seg_tree.update(min_index_position_in_sorted, usize::MAX);
            } else {
                unplaced += 1;
            }
        }

        unplaced
    }
}
fn main() {
    let test_cases = [
        (vec![4, 2, 5], vec![3, 5, 4], 1),
        (vec![3, 6, 1], vec![6, 4, 7], 0),
    ];
    for (fruits, baskets, exp) in test_cases {
        println!(
            "{}  exp: {exp}",
            Solution::num_of_unplaced_fruits(fruits, baskets)
        );
    }
}

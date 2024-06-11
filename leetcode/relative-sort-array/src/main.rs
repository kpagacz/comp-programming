// https://leetcode.com/problems/relative-sort-array/description/
pub struct Solution;

impl Solution {
    pub fn relative_sort_array(mut arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let map = arr2
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (pos, num)| {
                map.insert(num, pos);
                map
            });

        arr1.sort_unstable_by(|a, b| match (map.get(&a), map.get(&b)) {
            (Some(posa), Some(posb)) => posa.cmp(posb),
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, None) => a.cmp(b),
        });

        arr1
    }
}

fn main() {
    let test_cases = [(
        vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19],
        vec![2, 1, 4, 3, 9, 6],
    )];
    for (arr1, arr2) in test_cases {
        println!("{:#?}", Solution::relative_sort_array(arr1, arr2));
    }
}

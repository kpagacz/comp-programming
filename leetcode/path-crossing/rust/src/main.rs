// https://leetcode.com/problems/path-crossing/description/
pub struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        use std::collections::HashSet;
        type Map = HashSet<(i32, i32)>;

        let mut current_coords = (0i32, 0i32);
        let mut visited = Map::from([current_coords]);
        path.chars().any(|c| {
            match c {
                'N' => current_coords = (current_coords.0 - 1, current_coords.1),
                'S' => current_coords = (current_coords.0 + 1, current_coords.1),
                'W' => current_coords = (current_coords.0, current_coords.1 - 1),
                'E' => current_coords = (current_coords.0, current_coords.1 + 1),
                _ => unreachable!(),
            }
            if visited.contains(&current_coords) {
                true
            } else {
                visited.insert(current_coords);
                false
            }
        })
    }
}

fn main() {
    let test_cases = ["SN"];
    for test in test_cases {
        println!("{}", Solution::is_path_crossing(test.to_string()));
    }
}

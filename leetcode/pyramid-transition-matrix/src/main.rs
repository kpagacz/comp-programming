// https://leetcode.com/problems/pyramid-transition-matrix/description/?envType=daily-question&envId=2025-12-29
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        use std::collections::HashMap;
        let allowed: HashMap<Vec<u8>, Vec<u8>> =
            allowed
                .into_iter()
                .fold(HashMap::new(), |mut map, allowed| {
                    let allowed = allowed.as_bytes();
                    map.entry(allowed[0..2].to_vec())
                        .and_modify(|arr| arr.push(allowed[2]))
                        .or_insert(vec![allowed[2]]);
                    map
                });

        fn rec(
            current_layer: &mut Vec<u8>,
            previous_layer: &[u8],
            allowed: &HashMap<Vec<u8>, Vec<u8>>,
        ) -> bool {
            if previous_layer.len() == 1 {
                return true;
            }
            if current_layer.len() + 1 == previous_layer.len() {
                let previous_layer = current_layer.clone();
                current_layer.clear();
                let possible = rec(current_layer, &previous_layer, allowed);
                *current_layer = previous_layer;
                possible
            } else {
                let n = current_layer.len();
                let base = &previous_layer[n..n + 2];
                let mut possible = false;
                for &top in allowed.get(base).unwrap_or(&Vec::default()) {
                    current_layer.push(top);
                    possible = possible || rec(current_layer, previous_layer, allowed);
                    current_layer.pop();
                }
                possible
            }
        }

        let layer = bottom.as_bytes();
        let mut current_layer = Vec::new();
        rec(&mut current_layer, layer, &allowed)
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/shortest-uncommon-substring-in-an-array/
pub struct Solution;

impl Solution {
    pub fn shortest_substrings(arr: Vec<String>) -> Vec<String> {
        use std::collections::BTreeSet;
        use std::collections::HashMap;

        let substrings: HashMap<String, BTreeSet<usize>> =
            arr.iter()
                .enumerate()
                .fold(HashMap::new(), |mut map, (id, str)| {
                    (0..str.len()).for_each(|start| {
                        (start..str.len()).for_each(|end| {
                            map.entry(str[start..=end].to_owned())
                                .or_default()
                                .insert(id);
                        })
                    });
                    map
                });

        arr.into_iter()
            .map(|str| {
                let mut this_str_substrings = vec![];

                (0..str.len()).for_each(|start| {
                    (start..str.len()).for_each(|end| {
                        if substrings.get(&str[start..=end]).unwrap().len() == 1 {
                            this_str_substrings.push(str[start..=end].to_string());
                        }
                    })
                });

                this_str_substrings.sort_unstable_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
                println!("{:?}", this_str_substrings);
                this_str_substrings
                    .first()
                    .unwrap_or(&"".to_string())
                    .clone()
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}

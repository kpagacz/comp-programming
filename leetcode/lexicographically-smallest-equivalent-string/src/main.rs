// https://leetcode.com/problems/lexicographically-smallest-equivalent-string/description/?envType=daily-question&envId=2025-06-05
pub struct Solution;

impl Solution {
    pub fn smallest_equivalent_string(s1: String, s2: String, base_str: String) -> String {
        let mut parents = vec![usize::MAX; 27];

        fn root(parents: &mut [usize], letter: usize) -> usize {
            if parents[letter] == usize::MAX {
                return letter;
            }
            parents[letter] = root(parents, parents[letter]);
            parents[letter]
        }

        fn merge(parents: &mut [usize], first: usize, second: usize) {
            let first_root = root(parents, first);
            let second_root = root(parents, second);
            if first_root != second_root {
                if first_root < second_root {
                    parents[second_root] = first_root;
                } else {
                    parents[first_root] = second_root;
                }
            }
        }

        for (&c1, &c2) in s1.as_bytes().iter().zip(s2.as_bytes()) {
            let c1 = (c1 - b'a') as usize;
            let c2 = (c2 - b'a') as usize;
            merge(&mut parents, c1, c2);
        }

        base_str
            .as_bytes()
            .iter()
            .map(|&c| {
                let c = (c - b'a') as usize;
                let root = root(&mut parents, c);
                (b'a' + root as u8) as char
            })
            .collect::<String>()
    }
}

fn main() {
    println!("Hello, world!");
}

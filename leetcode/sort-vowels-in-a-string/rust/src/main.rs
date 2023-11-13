// https://leetcode.com/problems/sort-vowels-in-a-string/
pub struct Solution {}

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vowels_set = vec![b'a', b'A', b'e', b'E', b'o', b'O', b'i', b'I', b'u', b'U'];
        let mut s: Vec<_> = s.bytes().collect();
        let (pos, mut vowels): (Vec<usize>, Vec<u8>) = s
            .iter()
            .enumerate()
            .filter(|(_, c)| vowels_set.contains(c))
            .map(|(pos, &c)| (pos, c))
            .unzip();
        vowels.sort();

        for (i, vowel) in pos.into_iter().zip(vowels.into_iter()) {
            s[i] = vowel;
        }

        String::from_utf8(s).unwrap()
    }
}

fn main() {
    let test_cases = vec!["lEetcOde"];
    for s in test_cases {
        println!("{}", Solution::sort_vowels(s.to_owned()));
    }
}

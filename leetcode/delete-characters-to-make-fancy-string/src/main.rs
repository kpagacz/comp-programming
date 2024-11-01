// https://leetcode.com/problems/delete-characters-to-make-fancy-string/description/
pub struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut mask = vec![true; s.len()];

        s.as_bytes()
            .windows(3)
            .enumerate()
            .for_each(|(id, window)| {
                if window.iter().all(|el| window[0] == *el) {
                    mask[id] = false;
                }
            });

        let mut mask = mask.iter();
        s.chars()
            .enumerate()
            .filter(|_| *mask.next().unwrap())
            .map(|(_, c)| c)
            .collect::<String>()
    }
}

fn main() {
    println!("Hello, world!");
}

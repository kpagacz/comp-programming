// https://leetcode.com/problems/shortest-and-lexicographically-smallest-beautiful-string/
pub struct Solution {}

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let string = s.bytes().collect::<Vec<u8>>();
        let one = '1' as u8;
        let mut answer: Vec<u8> = (0..101).collect();
        let mut current = std::collections::VecDeque::new();
        let mut score = 0;
        (0..string.len()).for_each(|id| {
            current.push_back(string[id]);
            if string[id] == one {
                score += 1;
            }
            if score == k {
                while current.front().unwrap() != &one && current.len() > 1 {
                    current.pop_front();
                }
                match answer.len().cmp(&current.len()) {
                    std::cmp::Ordering::Greater => answer = current.clone().into(),
                    std::cmp::Ordering::Less => {}
                    std::cmp::Ordering::Equal => {
                        if String::from_utf8(current.clone().into()).unwrap()
                            < String::from_utf8(answer.clone()).unwrap()
                        {
                            answer = current.clone().into();
                        }
                    }
                }
                if current.front().unwrap() == &one {
                    score -= 1;
                }
                current.pop_front();
            }
        });

        if answer.len() == 101 {
            return "".to_string();
        }
        String::from_utf8(answer).unwrap()
    }
}
fn main() {
    println!("Hello, world!");
}

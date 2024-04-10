// https://leetcode.com/problems/reveal-cards-in-increasing-order/description/
pub struct Solution;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        use std::collections::VecDeque;
        deck.sort_unstable();
        let mut answer = VecDeque::new();

        while let Some(card) = deck.pop() {
            if let Some(last_card) = answer.pop_back() {
                answer.push_front(last_card);
            }
            answer.push_front(card);
        }

        answer.into()
    }
}

fn main() {
    let test_cases = [vec![17, 13, 11, 2, 3, 5, 7], vec![1, 1000]];

    for deck in test_cases {
        println!("{:?}", Solution::deck_revealed_increasing(deck));
    }
}

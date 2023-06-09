// https://leetcode.com/problems/find-smallest-letter-greater-than-target/description/

use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let first_greater = letters
            .iter()
            .find(|&&letter| letter.cmp(&target) == Ordering::Greater);
        match first_greater {
            Some(first) => *first,
            None => letters[0],
        }
    }

    pub fn upper_bound<T, F: FnOnce(&T) -> bool + Copy>(v: &Vec<T>, f: F) -> Option<usize> {
        let mut from = 0;
        let mut to = v.len();

        while from < to {
            let middle = (from + to) / 2;
            if !f(&v[middle]) {
                from = middle + 1;
            } else {
                to = middle;
            }
        }

        if from == v.len() {
            None
        } else {
            Some(from)
        }
    }

    pub fn next_greatest_letter2(letters: Vec<char>, target: char) -> char {
        let first_greater = Self::upper_bound(&letters, |&letter| target > letter);
        match first_greater {
            Some(position) => letters[position],
            None => letters[0],
        }
    }
}
fn main() {
    println!("Hello, world!");
}

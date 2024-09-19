// https://leetcode.com/problems/different-ways-to-add-parentheses/description/
pub struct Solution;

use std::ops::{Add, Mul, Sub};

enum Token<'a> {
    Val(i32),
    Op(&'a dyn Fn(i32, i32) -> i32),
}

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut v = Vec::<Token>::new();
        let mut val = 0;
        for b in expression.bytes() {
            if b >= b'0' && b <= b'9' {
                val = val * 10 + (b - b'0') as i32;
            } else {
                let fun: &dyn Fn(i32, i32) -> i32 = match b {
                    b'+' => &i32::add,
                    b'-' => &i32::sub,
                    b'*' => &i32::mul,
                    _ => panic!("unexpected char!"),
                };
                v.push(Token::Val(val));
                v.push(Token::Op(fun));
                val = 0;
            }
        }
        v.push(Token::Val(val));
        Self::dac(&v[..])
    }

    fn dac(slice: &[Token]) -> Vec<i32> {
        if slice.len() == 1 {
            if let Token::Val(val) = slice[0] {
                return vec![val];
            } else {
                unreachable!();
            }
        }
        let mut v = Vec::<i32>::new();
        for i in (1..slice.len()).step_by(2) {
            if let Token::Op(op) = slice[i] {
                let left = Self::dac(&slice[..i]);
                let right = Self::dac(&slice[i + 1..]);
                for &a in left.iter() {
                    for &b in right.iter() {
                        v.push(op(a, b));
                    }
                }
            } else {
                unreachable!();
            }
        }
        v
    }
}

fn main() {
    println!("Hello, world!");
}

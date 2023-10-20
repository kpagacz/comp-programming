// https://leetcode.com/problems/flatten-nested-list-iterator/description
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::iter::Peekable;
use std::vec::IntoIter;
struct NestedIterator {
    iterators: Vec<Peekable<IntoIter<NestedInteger>>>,
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        Self {
            iterators: vec![nestedList.into_iter().peekable()],
        }
    }

    fn next(&mut self) -> i32 {
        let last = self.iterators.last_mut().unwrap();
        match last.next() {
            Some(nested_integer) => match nested_integer {
                NestedInteger::Int(i) => return i,
                NestedInteger::List(list) => {
                    self.iterators.push(list.into_iter().peekable());
                    return self.next();
                }
            },
            None => {
                self.iterators.pop();
                return self.next();
            }
        }
    }

    fn has_next(&mut self) -> bool {
        if self.iterators.is_empty() {
            return false;
        }

        let last = self.iterators.last_mut().unwrap();
        match last.peek() {
            Some(nested_integer) => match nested_integer {
                NestedInteger::Int(_) => true,
                NestedInteger::List(new_list) => {
                    if NestedIterator::any_non_list(new_list) {
                        true
                    } else {
                        last.next();
                        self.has_next()
                    }
                }
            },
            None => {
                self.iterators.pop();
                return self.has_next();
            }
        }
    }

    fn any_non_list(arr: &Vec<NestedInteger>) -> bool {
        arr.iter().any(|nested_integer| match nested_integer {
            NestedInteger::Int(_) => true,
            NestedInteger::List(new_arr) => NestedIterator::any_non_list(new_arr),
        })
    }
}

fn main() {
    println!("Hello, world!");
}

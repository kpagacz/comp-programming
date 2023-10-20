// https://leetcode.com/problems/flatten-nested-list-iterator/description
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

struct NestedIterator {
    arr: Vec<i32>,
    it: i32,
}

impl NestedIterator {
    fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut arr = vec![];
        nestedList
            .iter()
            .for_each(|nested_integer| NestedIterator::unnest(nested_integer, &mut arr));
        Self { arr, it: -1 }
    }

    fn next(&mut self) -> i32 {
        self.it += 1;
        self.arr[self.it as usize]
    }

    fn has_next(&mut self) -> bool {
        self.it + 1 < self.arr.len() as i32
    }

    fn unnest(nested_integer: &NestedInteger, arr: &mut Vec<i32>) {
        match nested_integer {
            NestedInteger::Int(i) => arr.push(*i),
            NestedInteger::List(nested_arr) => nested_arr
                .iter()
                .for_each(|nested_int| NestedIterator::unnest(nested_int, arr)),
        }
    }
}

fn main() {
    println!("Hello, world!");
}

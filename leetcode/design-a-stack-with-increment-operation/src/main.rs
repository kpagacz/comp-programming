// https://leetcode.com/problems/design-a-stack-with-increment-operation/description/
struct CustomStack {
    stack: Vec<i32>,
    capacity: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {
    fn new(maxSize: i32) -> Self {
        Self {
            stack: Vec::default(),
            capacity: maxSize as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.capacity {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        self.stack
            .iter_mut()
            .take(k as usize)
            .for_each(|num| *num += val);
    }
}

fn main() {
    println!("Hello, world!");
}

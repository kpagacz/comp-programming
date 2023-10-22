// https://leetcode.com/problems/implement-queue-using-stacks/description/
struct MyQueue {
    stack: std::collections::VecDeque<i32>,
}
impl MyQueue {
    fn new() -> Self {
        Self {
            stack: std::collections::VecDeque::<i32>::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop_front().unwrap()
    }

    fn peek(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}
fn main() {
    println!("Hello, world!");
}

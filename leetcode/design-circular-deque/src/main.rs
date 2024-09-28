// https://leetcode.com/problems/design-circular-deque/description/
struct MyCircularDeque {
    arr: Vec<i32>,
    front: usize,
    back: usize,
    size: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            arr: vec![-1; k as usize],
            front: 0,
            back: k as usize - 1,
            size: 0,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.arr[self.front] = value;
            self.front = (self.front + 1) % self.arr.len();
            self.size += 1;
            true
        } else {
            false
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if !self.is_full() {
            self.arr[self.back] = value;
            self.back = (self.back + self.arr.len() - 1) % self.arr.len();
            self.size += 1;
            true
        } else {
            false
        }
    }

    fn delete_front(&mut self) -> bool {
        if !self.is_empty() {
            self.front = (self.front + self.arr.len() - 1) % self.arr.len();
            self.size -= 1;
            true
        } else {
            false
        }
    }

    fn delete_last(&mut self) -> bool {
        if !self.is_empty() {
            self.back = (self.back + 1) % self.arr.len();
            self.size -= 1;
            true
        } else {
            false
        }
    }

    fn get_front(&self) -> i32 {
        if !self.is_empty() {
            self.arr[(self.front + self.arr.len() - 1) % self.arr.len()]
        } else {
            -1
        }
    }

    fn get_rear(&self) -> i32 {
        if !self.is_empty() {
            self.arr[(self.back + 1 + self.arr.len()) % self.arr.len()]
        } else {
            -1
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.arr.len()
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

fn main() {
    println!("Hello, world!");
}

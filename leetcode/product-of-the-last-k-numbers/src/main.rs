// https://leetcode.com/problems/product-of-the-last-k-numbers/description/

struct ProductOfNumbers {
    nums: Vec<i32>,
}
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self { nums: vec![1] }
    }

    fn add(&mut self, mut num: i32) {
        if num == 0 {
            self.nums.clear();
            self.nums.push(1);
        } else {
            self.nums.push(self.nums.last().unwrap() * num);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        if k as usize >= self.nums.len() {
            0
        } else {
            let n = self.nums.len();
            self.nums[n - 1] / self.nums[n - 1 - k as usize]
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */
fn main() {
    println!("Hello, world!");
}

//https://leetcode.com/problems/my-calendar-i/description/

use std::collections::BTreeMap;
struct MyCalendar {
    bookings: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Self {
            bookings: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        let mut range = self.bookings.range(start..);
        if let Some((&next_start, &_)) = range.next() {
            if end > next_start {
                return false;
            }
        }

        let mut range = self.bookings.range(..=start).rev();
        if let Some((&_, &prev_end)) = range.next() {
            if prev_end > start {
                return false;
            }
        }

        self.bookings.insert(start, end);
        true
    }
}

/**
 * Your MyCalendar object will be instantiated and called as such:
 * let obj = MyCalendar::new();
 * let ret_1: bool = obj.book(start, end);
 */

fn main() {
    println!("Hello, world!");
}

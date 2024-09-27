// https://leetcode.com/problems/my-calendar-ii/description
use std::collections::BTreeMap;

struct MyCalendarTwo {
    events: BTreeMap<i32, i32>, // end, start
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendarTwo {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.events.entry(start).or_default() += 1;
        *self.events.entry(end).or_default() -= 1;

        let mut running = 0;
        for &change in self.events.values() {
            running += change;

            if running > 2 {
                *self.events.entry(start).or_default() -= 1;
                *self.events.entry(end).or_default() += 1;
                return false;
            }
        }
        true
    }
}

/**
 * Your MyCalendarTwo object will be instantiated and called as such:
 * let obj = MyCalendarTwo::new();
 * let ret_1: bool = obj.book(start, end);
 */
fn main() {
    println!("Hello, world!");
}

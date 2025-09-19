// https://leetcode.com/problems/design-spreadsheet/description/?envType=daily-question&envId=2025-09-19

use std::collections::HashMap;
struct Spreadsheet {
    spreadsheet: HashMap<String, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl Spreadsheet {
    fn new(_rows: i32) -> Self {
        Self {
            spreadsheet: HashMap::new(),
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.spreadsheet
            .entry(cell)
            .and_modify(|previous| *previous = value)
            .or_insert(value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.spreadsheet.remove(&cell);
    }

    fn get_value(&self, formula: String) -> i32 {
        let (first_part, second_part) = formula.split_once('+').unwrap();

        let first_part = first_part.strip_prefix('=').unwrap();
        let first_part = if first_part.parse::<i32>().is_ok() {
            first_part.parse::<i32>().unwrap()
        } else {
            *self.spreadsheet.get(first_part).unwrap_or(&0)
        };

        let second_part = if second_part.parse::<i32>().is_ok() {
            second_part.parse::<i32>().unwrap()
        } else {
            *self.spreadsheet.get(second_part).unwrap_or(&0)
        };

        first_part + second_part
    }
}

fn main() {
    println!("Hello, world!");
}

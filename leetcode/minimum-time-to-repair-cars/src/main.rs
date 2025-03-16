// https://leetcode.com/problems/minimum-time-to-repair-cars/description/?envType=daily-question&envId=2025-03-16
pub struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut right = i64::MAX;
        let mut left = 1i64;
        let cars = cars as i64;

        fn can_repair(ranks: &[i32], mut cars: i64, minutes: i64) -> bool {
            let minutes = minutes as f64;
            for &rank in ranks {
                let fixed_cars = f64::sqrt(minutes / rank as f64).floor() as i64;
                cars -= fixed_cars;
            }
            cars <= 0
        }

        while left < right {
            let middle = left + (right - left) / 2;

            if can_repair(&ranks, cars, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left
    }
}

fn main() {
    println!("Hello, world!");
}

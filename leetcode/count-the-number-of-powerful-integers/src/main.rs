// https://leetcode.com/problems/count-the-number-of-powerful-integers/description/?envType=daily-question&envId=2025-04-10
pub struct Solution;

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let num_s = s.parse::<i64>().unwrap();
        if num_s > finish {
            return 0;
        }

        let pow10suffix = 10i64.pow(s.len() as u32);

        let start_prefix = start / pow10suffix + if start % pow10suffix > num_s { 1 } else { 0 };
        let finish_prefix =
            finish / pow10suffix + if finish % pow10suffix >= num_s { 1 } else { 0 };

        fn count_nums(mut n: i64, limit: i32) -> i64 {
            let limit = limit as i64;
            let mut pow = 1;
            let mut result = 0;

            while n > 0 {
                let digit = n % 10;
                n /= 10;

                if digit > limit {
                    result = 0;
                    n += 1;
                } else {
                    result += digit * pow;
                }

                pow *= limit + 1;
            }

            result
        }

        count_nums(finish_prefix, limit) - count_nums(start_prefix, limit)
    }
}

fn main() {
    println!("Hello, world!");
}

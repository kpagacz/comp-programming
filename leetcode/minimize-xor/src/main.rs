// https://leetcode.com/problems/minimize-xor/description/
pub struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut ones = num2.count_ones();

        let mut answer = 0;
        for shift in (0..32).rev() {
            if ones == 0 {
                break;
            }

            if (num1 >> shift) & 1 == 1 {
                answer |= 1 << shift;
                ones -= 1;
            }
        }

        for shift in 0..32 {
            if ones == 0 {
                break;
            }

            if (answer >> shift) & 1 == 0 {
                answer |= 1 << shift;
                ones -= 1;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}

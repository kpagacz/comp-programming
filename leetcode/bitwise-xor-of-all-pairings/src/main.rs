// https://leetcode.com/problems/bitwise-xor-of-all-pairings/description/
pub struct Soluion;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut ones = [0usize; 32];
        let l1 = nums1.len();

        for num in nums1 {
            for shift in 0..32 {
                if (num >> shift) & 1 == 1 {
                    ones[shift] += 1;
                }
            }
        }

        let mut n3_ones = [0usize; 32];
        for num in nums2 {
            for shift in 0..32 {
                if (num >> shift) & 1 == 1 {
                    n3_ones[shift] += l1 - ones[shift];
                } else {
                    n3_ones[shift] += ones[shift];
                }
            }
        }

        let mut answer = 0;
        for shift in 0..32 {
            if n3_ones[shift] & 1 == 1 {
                answer |= 1 << shift;
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}

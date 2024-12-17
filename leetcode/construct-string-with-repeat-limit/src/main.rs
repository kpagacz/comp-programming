// https://leetcode.com/problems/construct-string-with-repeat-limit/description/
pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut count = [0; 256]; // Count occurrences of each byte (0-255)

        // Step 1: Count occurrences of each character
        for &b in s.as_bytes() {
            count[b as usize] += 1;
        }

        let mut result = String::new();
        let mut current_index = Self::find_last_valid(&count, 255);

        while let Some(idx) = current_index {
            // Find the next largest valid byte smaller than `current_index`
            let next_index = Self::find_last_valid(&count, idx - 1);

            // Append `repeat_limit` or fewer occurrences of the current byte
            let use_count = repeat_limit.min(count[idx]);
            for _ in 0..use_count {
                result.push(idx as u8 as char);
            }
            count[idx] -= use_count;

            // If there are still occurrences left, insert one byte of `next_index`
            if count[idx] > 0 {
                if let Some(next_idx) = next_index {
                    result.push(next_idx as u8 as char);
                    count[next_idx] -= 1;
                } else {
                    break; // No valid character to break the sequence
                }
            } else {
                current_index = next_index;
            }
        }

        result
    }

    // Helper function to find the last valid index with non-zero count
    fn find_last_valid(count: &[i32], mut index: usize) -> Option<usize> {
        while index > 0 {
            if count[index] > 0 {
                return Some(index);
            }
            index -= 1;
        }
        None // Return None if no valid index is found
    }
}

fn main() {
    println!("Hello, world!");
}

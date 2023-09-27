// https://leetcode.com/problems/decoded-string-at-index
pub struct Solution {}

impl Solution {
    // Works
    pub fn decode_at_index(s: String, k: i32) -> String {
        let k = k as usize;
        let s: Vec<char> = s.chars().collect();
        let mut cycles: Vec<(usize, u32)> = Vec::new(); // (cycle end position, remaining repeats)
        let mut cycle_sizes = vec![0; s.len()];

        let mut it = 0_usize;
        let mut letter_counter = 1;
        let mut cycle_memory = 0;

        while letter_counter < k {
            let c = s[it + 1];

            if c.is_digit(10) {
                if cycle_sizes[it] == 0 {
                    cycle_sizes[it] = letter_counter - cycle_memory;
                }
                if let Some(cycle) = cycles.last_mut() {
                    if cycle.0 == it + 1 {
                        if cycle.1 == 0 {
                            cycles.pop();
                            it += 1;
                        } else {
                            cycle.1 -= 1;
                            it = 0;
                            cycle_memory = letter_counter;
                            letter_counter += 1;
                        }
                        continue;
                    }
                }

                let rounds = c.to_digit(10).unwrap() as usize;
                if letter_counter + (rounds - 1) * cycle_sizes[it] < k {
                    letter_counter += (rounds - 1) * cycle_sizes[it];
                    it += 1;
                } else {
                    cycles.push((it, rounds as u32 - 2));
                    letter_counter += 1;
                    it = 0;
                }
            } else {
                letter_counter += 1;
                it += 1;
            }
        }

        s[it].to_string()
    }

    // This one I didn't come up with on my own, but hey, if you don't implement,
    // then you don't improve
    // The idea is to calculate the position and the length that go just above the
    // required length, and then do iterate through the string in reverse
    pub fn decode_at_index_reverse(s: String, k: i32) -> String {
        let mut k = k as i64;
        let mut i = 0_usize;
        let mut length = 0_i64;
        let s: Vec<char> = s.chars().collect();

        while length < k {
            if s[i].is_digit(10) {
                length *= s[i].to_digit(10).unwrap() as i64;
            } else {
                length += 1;
            }
            i += 1;
        }

        for it in (0..i).rev() {
            if s[it].is_digit(10) {
                length /= (s[it].to_digit(10)).unwrap() as i64;
                k = k % length;
            } else {
                if k == 0 || k == length {
                    return s[it].to_string();
                }
                length -= 1;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}

// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-ii/description/?envType=daily-question&envId=2025-06-11
pub struct Solution;

// prefix_a[i] = count of as until i (i included)
// prefix_b[i] = count of bs until i (i included)
//
// For a given chars a and b we maximize:
// (prefix_a[right] - prefix_a[left]) - (prefix_b[right] - prefix_b[left])
// (prefix_a[right] - prefix_b[right]) - (prefix_a[left] - prefix_b[left])
//
// The right will track the right end, the left iterator will track the left end
// of the substring.
// There's also parity to be worried about.
// prefix_a[right] - prefix_a[left] needs to be odd.
// prefix-b[right] - prefix_b[left] needs to be even.
// So:
// (prefix_a[right] % 2) ^ (prefix_a[left]) == 1
// (prefix_b[right] % 2) ^ (prefix_b[left]) == 0
// So for a given right:
// prefix_a[right] ^ 1 = prefix_a[left]
// prefix_b[right] ^ 0 = prefix_b[left]
impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut answer = i32::MIN;
        for a in b'0'..=b'4' {
            for b in b'0'..=b'4' {
                if a == b {
                    continue;
                }
                let mut count_a = 0;
                let mut count_b = 0;
                let mut prefix_as = 0;
                let mut prefix_bs = 0;
                let mut left = -1;
                let mut best = [i32::MAX; 4];

                for right in 0..s.len() {
                    if s[right] == a {
                        count_a += 1;
                    }
                    if s[right] == b {
                        count_b += 1;
                    }

                    // Need to ensure that the substring is of size > 0
                    // and that there is at least one b
                    while right as i32 - left >= k && count_b - prefix_bs >= 2 {
                        let current_parity = Self::get_parity(prefix_as, prefix_bs);

                        best[current_parity] = best[current_parity].min(prefix_as - prefix_bs);
                        left += 1;
                        if s[left as usize] == a {
                            prefix_as += 1;
                        }
                        if s[left as usize] == b {
                            prefix_bs += 1;
                        }
                    }

                    let right_parity = Self::get_parity(count_a, count_b);
                    let required_left_end_parity = right_parity ^ 0b10;
                    if best[required_left_end_parity] != i32::MAX {
                        answer = answer.max(count_a - count_b - best[required_left_end_parity]);
                    }
                }
            }
        }
        answer
    }

    fn get_parity(prefix_as: i32, prefix_bs: i32) -> usize {
        (((prefix_as & 1) << 1) | (prefix_bs & 1)) as usize
    }
}

fn main() {
    println!("Hello, world!");
}

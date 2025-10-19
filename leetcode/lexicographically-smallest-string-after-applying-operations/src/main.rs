// https://leetcode.com/problems/lexicographically-smallest-string-after-applying-operations/description/?envType=daily-question&envId=2025-10-19
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let mut s = s.as_bytes().to_vec();

        let mut smallest: Vec<u8> = vec![];

        let max_inc = (a * 10) / Solution::gcd(a, 10);
        let max_rotate = (b as usize * s.len()) / Solution::gcd(b, s.len() as i32) as usize;
        use std::collections::HashSet;
        let mut visited: HashSet<Vec<u8>> = HashSet::new();

        #[allow(clippy::too_many_arguments)]
        fn rec(
            s: &mut [u8],
            increment_by: usize,
            max_inc: i32,
            rotate_by: usize,
            current_rotation: usize,
            max_rotation: usize,
            smallest: &mut Vec<u8>,
            visited: &mut HashSet<Vec<u8>>,
        ) {
            if visited.contains(s) {
                return;
            }
            if s < smallest || smallest.is_empty() {
                *smallest = s.to_vec();
            }
            visited.insert(s.to_vec());
            if current_rotation == max_rotation {
                return;
            }

            for increment in (0..max_inc).step_by(increment_by) {
                Solution::increment_odd_digits(s, increment);
                s.rotate_right(rotate_by);
                if !visited.contains(s) {
                    rec(
                        s,
                        increment_by,
                        max_inc,
                        rotate_by,
                        current_rotation + rotate_by,
                        max_rotation,
                        smallest,
                        visited,
                    );
                }
                s.rotate_left(rotate_by);
                Solution::increment_odd_digits(s, 10 - (increment % 10));
            }
        }

        rec(
            &mut s,
            a as usize,
            max_inc,
            b as usize,
            0,
            max_rotate,
            &mut smallest,
            &mut visited,
        );

        String::from_utf8(smallest).unwrap()
    }

    fn increment_odd_digits(s: &mut [u8], by: i32) {
        s.iter_mut()
            .skip(1)
            .step_by(2)
            .for_each(|b| Solution::increment_digit(b, by));
    }

    fn increment_digit(digit: &mut u8, by: i32) {
        let from_zero = *digit - b'0';
        *digit = b'0' + (from_zero + by as u8) % 10;
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Solution::gcd(b, a % b) }
    }
}

fn main() {
    let test_cases = [
        ("5525", 9, 2, "2050"),
        ("74", 5, 1, "24"),
        ("0011", 4, 2, "0011"),
    ];

    for (s, a, b, exp) in test_cases {
        let s = s.to_string();
        println!(
            "{}   exp: {exp}",
            Solution::find_lex_smallest_string(s, a, b)
        );
    }
}

// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/description/
pub struct Solution;

impl Solution {
    pub fn find_kth_bit_old(n: i32, k: i32) -> char {
        let mut s = vec!['0'];

        fn invert(s: &[char]) -> Vec<char> {
            s.iter()
                .map(|&c| if c == '0' { '1' } else { '0' })
                .collect::<Vec<_>>()
        }

        for _ in 1..n {
            s.extend(std::iter::once('1').chain(invert(&s).into_iter().rev()));
        }

        s[(k - 1) as usize]
    }

    pub fn find_kth_bit(_n: i32, k: i32) -> char {
        fn find_kth_bit_rec(bit: usize) -> char {
            if bit == 1 {
                return '0';
            }
            if bit.count_ones() == 1 {
                return '1';
            }

            let lower_power_of_two = (bit as f64).log2().floor() as u32;
            let power_of_two = 2usize.pow(lower_power_of_two + 1);
            let inversed = find_kth_bit_rec(power_of_two - bit);

            match inversed {
                '0' => '1',
                '1' => '0',
                _ => unreachable!(),
            }
        }

        let k = k as usize;
        find_kth_bit_rec(k)
    }
}

fn main() {
    let test_cases = [(3, 1), (4, 11)];

    for (n, k) in test_cases {
        println!(
            "old {} new {}",
            Solution::find_kth_bit_old(n, k),
            Solution::find_kth_bit(n, k)
        );
    }
}

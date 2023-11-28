// https://leetcode.com/problems/number-of-ways-to-divide-a-long-corridor/
pub struct Solution {}

const MOD: i64 = 10i64.pow(9) + 7;
impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let c = corridor.as_bytes();
        let mut answer = 1i64;
        let mut seats_seen = 0;
        let mut last_seat = 0usize;
        for (i, s) in c.iter().enumerate() {
            // println!("{i} {s} {seats_seen} {last_seat}");
            if s == &b'S' {
                seats_seen += 1;
                if seats_seen == 2 {
                    last_seat = i
                }
                if seats_seen == 3 {
                    // println!("Pushing to possible seats at {i} with seats_seen {seats_seen} and last_seat {last_seat}");
                    answer = (answer * (i - last_seat) as i64) % MOD;
                    seats_seen = 1;
                }
            }
            if i == c.len() - 1 && seats_seen != 2 {
                answer *= 0;
            }
        }
        // println!("{possible_dividers:?}");
        answer as i32
    }
}

fn main() {
    let test_cases = ["SSPPSPS", "PPSPSP", "S", "SSSPPSSS", "SSPPSSPPSS"];
    for c in test_cases {
        println!("{}", Solution::number_of_ways(String::from(c)));
    }
}

// https://leetcode.com/problems/maximum-total-damage-with-spell-casting/?envType=daily-question&envId=2025-10-11
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        use std::collections::BTreeMap;

        let freqs = power.into_iter().fold(BTreeMap::new(), |mut map, value| {
            map.entry(value).and_modify(|freq| *freq += 1).or_insert(1);
            map
        });

        let mut dp = vec![0i64; freqs.len()];
        let mut previous_powers = [i64::MAX, i64::MAX];

        for (pos, (power, freq)) in freqs.into_iter().enumerate() {
            if previous_powers[0] == i64::MAX && previous_powers[1] == i64::MAX {
                dp[pos] = power as i64 * freq as i64;
                previous_powers[0] = power as i64;
                continue;
            }

            dp[pos] = dp[pos].max(dp[pos - 1]); // you can always just not cast spells of this

            // Find how many spells of lower power have power <= this spells power
            let mut rewind = 1usize;
            for previous_power in previous_powers {
                if previous_power != i64::MAX && previous_power + 2 >= power as i64 {
                    rewind += 1;
                }
            }

            if rewind <= pos {
                dp[pos] = dp[pos].max(dp[pos - rewind] + power as i64 * freq as i64);
            } else {
                dp[pos] = dp[pos].max(power as i64 * freq as i64);
            }

            previous_powers.rotate_right(1);
            previous_powers[0] = power as i64;
        }

        *dp.last().unwrap()
    }
}

fn main() {
    let test_cases = [(vec![1, 1, 3, 4], 6)];
    for (power, exp) in test_cases {
        println!("{}   exp: {exp}", Solution::maximum_total_damage(power));
    }
}

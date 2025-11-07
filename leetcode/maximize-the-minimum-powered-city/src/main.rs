// https://leetcode.com/problems/maximize-the-minimum-powered-city/description/?envType=daily-question&envId=2025-11-07
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let r = r as usize;
        let k = k as i64;

        let mut left = 0;
        let mut right = i64::MAX;
        let mut additional_stations = vec![0; stations.len()];

        while left < right {
            let target_power = left + (right - left) / 2;
            let mut additional_stations_left = k;

            let mut current_power = stations
                .iter()
                .take(r)
                .fold(0i64, |acc, count| acc + *count as i64);

            for i in 0..stations.len() {
                if i + r < stations.len() {
                    current_power += stations[i + r] as i64;
                }
                if i.wrapping_sub(r).wrapping_sub(1) < stations.len() {
                    current_power -= stations[i - r - 1] as i64;
                    current_power -= additional_stations[i - r - 1];
                }
                let missing_power = (target_power - current_power).max(0);
                additional_stations[(i + r).min(stations.len() - 1)] += missing_power;
                current_power += missing_power;
                additional_stations_left = additional_stations_left.saturating_sub(missing_power);
            }

            if additional_stations_left >= 0 {
                left = target_power + 1;
            } else {
                right = target_power;
            }
            additional_stations.fill(0);
        }

        if left > 0 { left - 1 } else { 0 }
    }
}

fn main() {
    let test_cases = [(vec![1, 2, 4, 5, 0], 1, 2), (vec![4, 4, 4, 4], 0, 3)];

    for (stations, r, k) in test_cases {
        println!("{}", Solution::max_power(stations, r, k));
    }
}

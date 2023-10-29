//  https://leetcode.com/problems/poor-pigs/
pub struct Solution {}
impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        if buckets == 1 {
            return 0;
        }
        let buckets = buckets as u32;
        let testing_sessions = (minutes_to_test as f32 / minutes_to_die as f32).ceil() as usize + 1;
        let mut dp = vec![vec![1u32; testing_sessions]; 10001];

        let mut pigs = 0_usize;
        loop {
            pigs += 1;
            for time in 1..testing_sessions {
                let mut possible_buckets = 0;
                // A variable number of pigs die according to the labelling schema
                // 0000 <- no pigs die
                // 0001, 0010, 0100, 1000 <- one pig dies
                // 0011, 0101, 0110, 1001, 1010, 1100 <- two pigs die
                // etc...
                // 1111 <- all pigs die
                (0..=pigs).for_each(|pigs_left| {
                    possible_buckets +=
                        Solution::n_choose_k(pigs, pigs_left) * dp[pigs_left][time - 1];
                });
                if possible_buckets >= buckets {
                    for row in dp.iter().take(pigs + 1) {
                        println!("{row:?}");
                    }
                    return pigs as i32;
                }
                dp[pigs][time] = possible_buckets;
            }
        }
    }

    fn n_choose_k(n: usize, k: usize) -> u32 {
        if k > n / 2 {
            return Solution::n_choose_k(n, n - k);
        }
        (1..=k).fold(1, |acc, i| acc * (n - i + 1) / i) as u32
    }

    pub fn poor_pigs_smarter(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let max_sessions = (minutes_to_test as f32 / minutes_to_die as f32).ceil() as i32 + 1;
        let mut it = 0; // 2 -> 4 -> 16
        let mut max_buckets = 1;
        while max_buckets < buckets {
            max_buckets *= max_sessions;
            it += 1;
        }
        it
    }
}
fn main() {
    let test_cases = vec![
        (1, 1, 1),
        (8, 1, 1),
        (16, 1, 1),
        (32, 1, 1),
        (32, 1, 2),
        // (4, 15, 30),
        // (7, 15, 30),
        // (8, 5, 15),
        // (1024, 1, 1),
        (1000, 1, 50),
        (10, 1, 2),
    ];
    for (buckets, minutes_to_die, minutes_to_test) in test_cases {
        println!(
            "{}",
            Solution::poor_pigs(buckets, minutes_to_die, minutes_to_test)
        );
    }
}

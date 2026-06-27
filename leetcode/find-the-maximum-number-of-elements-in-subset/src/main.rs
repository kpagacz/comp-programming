// https://leetcode.com/problems/find-the-maximum-number-of-elements-in-subset/description/?envType=daily-question&envId=2026-06-27
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        use std::collections::{BTreeMap, HashMap};
        type Map = BTreeMap<i64, u32>;
        let freqs = nums.into_iter().fold(Map::new(), |mut map, num| {
            map.entry(num as i64)
                .and_modify(|count| *count += 1)
                .or_insert(1);
            map
        });
        let mut dp = HashMap::new();

        for &key in freqs.keys() {
            dp.insert(key, 1);
        }

        let ones_count = *freqs.get(&1).unwrap_or(&0);
        for (num, count) in freqs.into_iter() {
            if count >= 2 && dp.contains_key(&(num * num)) && num != 1 {
                dp.insert(num * num, *dp.get(&num).unwrap() + 2);
            }
        }

        *dp.values().max().unwrap().max(
            &(if ones_count % 2 == 1 {
                ones_count as i32
            } else {
                ones_count as i32 - 1
            }),
        )
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![5, 4, 1, 2, 2];
        let expected = 3;

        assert_eq!(Solution::maximum_length(nums), expected);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 3, 2, 4];
        let expected = 1;

        assert_eq!(Solution::maximum_length(nums), expected);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1];
        let expected = 1;

        assert_eq!(Solution::maximum_length(nums), expected);
    }
}

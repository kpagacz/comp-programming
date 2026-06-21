// https://leetcode.com/problems/maximum-ice-cream-bars/description/?envType=daily-question&envId=2026-06-21
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, mut coins: i32) -> i32 {
        let mut counting_sort = vec![0; 100_001];
        for cost in costs {
            counting_sort[cost as usize] += 1;
        }
        let mut bought = 0;
        for (cost, count) in counting_sort.into_iter().enumerate() {
            if count == 0 {
                continue;
            }
            let cost = cost as i32;
            if cost * count > coins {
                bought += coins / cost;
                break;
            } else {
                coins -= cost * count;
                bought += count;
            }
        }
        bought
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
        let costs = vec![1, 3, 2, 4, 1];
        let coins = 7;
        let expected = 4;
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }

    #[test]
    fn test2() {
        let costs = vec![10, 6, 8, 7, 7, 8];
        let coins = 5;
        let expected = 0;
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }

    #[test]
    fn test3() {
        let costs = vec![1, 6, 3, 1, 2, 5];
        let coins = 20;
        let expected = 6;
        assert_eq!(Solution::max_ice_cream(costs, coins), expected);
    }
}

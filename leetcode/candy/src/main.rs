// https://leetcode.com/problems/candy/description/?envType=daily-question&envId=2025-06-02
pub struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.is_empty() {
            return 0;
        }
        let mut candies = 1;
        let mut ups = 0;
        let mut downs = 0;
        let mut peak = 0;
        for i in 0..ratings.len() - 1 {
            let (prev, curr) = (ratings[i], ratings[i + 1]);
            match prev.cmp(&curr) {
                std::cmp::Ordering::Less => {
                    ups += 1;
                    downs = 0;
                    peak = ups + 1;
                    candies += ups + 1;
                }
                std::cmp::Ordering::Equal => {
                    ups = 0;
                    downs = 0;
                    peak = 0;
                    candies += 1;
                }
                std::cmp::Ordering::Greater => {
                    ups = 0;
                    downs += 1;
                    candies += downs;
                    if peak <= downs {
                        candies += 1;
                    }
                }
            }
        }
        candies
    }
}

fn main() {
    let test_cases = [
        (vec![1, 0, 2], 5),
        (vec![1, 2, 2], 4),
        (vec![1, 3, 2, 2, 1], 7),
        (vec![1, 2, 87, 87, 87, 2, 1], 13),
        (vec![1, 6, 10, 8, 7, 3, 2], 18),
    ];
    for (ratings, exp) in test_cases {
        println!("ratings: {ratings:?}");
        println!("{} exp: {exp}", Solution::candy(ratings));
    }
}

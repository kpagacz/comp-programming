// https://leetcode.com/problems/maximum-building-height/description/?envType=daily-question&envId=2026-06-20
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        let mut limits = restrictions.clone();
        limits.push(vec![1, 0]);
        limits.sort_by_key(|restriction| restriction[0]);
        if limits.last().unwrap()[0] != n {
            limits.push(vec![n, n - 1]);
        }

        for i in 1..limits.len() {
            let (previous_id, previous_limit) = (limits[i - 1][0], limits[i - 1][1]);
            let id = limits[i][0];

            limits[i][1] = limits[i][1].min(previous_limit + id - previous_id);
        }

        for i in (0..limits.len() - 1).rev() {
            let (previous_id, previous_limit) = (limits[i + 1][0], limits[i + 1][1]);
            let id = limits[i][0];

            limits[i][1] = limits[i][1].min(previous_limit + previous_id - id);
        }

        let mut answer = 0;
        limits.windows(2).for_each(|window| {
            let [first_id, first_limit]: &[_; 2] = window[0][..2].try_into().unwrap();
            let [second_id, second_limit]: &[_; 2] = window[1][..2].try_into().unwrap();
            let max = (second_id - first_id + first_limit + second_limit) / 2;
            answer = answer.max(max);
        });
        answer
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
        let n = 5;
        let restrictions = vec![vec![2, 1], vec![4, 1]];
        assert_eq!(Solution::max_building(n, restrictions), 2);
    }

    #[test]
    fn test2() {
        let n = 6;
        let restrictions = vec![];
        assert_eq!(Solution::max_building(n, restrictions), 5);
    }

    #[test]
    fn test3() {
        let n = 10;
        let restrictions = vec![vec![5, 3], vec![2, 5], vec![7, 4], vec![10, 3]];
        assert_eq!(Solution::max_building(n, restrictions), 5);
    }
    #[test]
    fn test4() {
        let n = 10;
        let restrictions = vec![
            vec![8, 5],
            vec![9, 0],
            vec![6, 2],
            vec![4, 0],
            vec![3, 2],
            vec![10, 0],
            vec![5, 3],
            vec![7, 3],
            vec![2, 4],
        ];
        assert_eq!(Solution::max_building(n, restrictions), 2);
    }
}

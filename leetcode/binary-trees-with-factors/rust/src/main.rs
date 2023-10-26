// https://leetcode.com/problems/binary-trees-with-factors/
pub struct Solution {}

const MODULO: usize = 10usize.pow(9) + 7;
impl Solution {
    pub fn num_factored_binary_trees(arr: Vec<i32>) -> i32 {
        let mut arr: Vec<usize> = arr.into_iter().map(|num| num as usize).collect();
        arr.sort_unstable();
        let mut dp = Vec::with_capacity(arr.len());

        (0..arr.len()).for_each(|it| {
            let num = arr[it];
            let mut trees = 1usize;
            (0..it).for_each(|first| {
                let left = arr[first];
                if num % left == 0 {
                    let right = num / left;
                    let right_pos = arr.partition_point(|e| e < &right);
                    if right_pos < arr.len() && arr[right_pos] == right {
                        trees += dp[first] * dp[right_pos];
                    }
                }
            });
            dp.push(trees);
        });
        dp.into_iter()
            .reduce(|acc, num| (acc + num) % MODULO)
            .unwrap() as i32
    }
    pub fn num_factored_binary_trees_rec(arr: Vec<i32>) -> i32 {
        use std::collections::{HashMap, HashSet};
        let mut factors = HashMap::new();
        let mut nums = HashSet::new();
        let mut mem = HashMap::new();

        arr.iter().for_each(|&num| {
            nums.insert(num);
        });
        (0..arr.len()).for_each(|first| {
            (first..arr.len()).for_each(|second| {
                let mul = arr[first].saturating_mul(arr[second]);
                if mul != i32::MAX && nums.contains(&mul) {
                    factors
                        .entry(mul)
                        .or_insert(Vec::new())
                        .push((arr[first], arr[second]));
                }
            })
        });

        fn count_trees(
            root: i32,
            mem: &mut HashMap<i32, i32>,
            nums: &HashSet<i32>,
            factors: &HashMap<i32, Vec<(i32, i32)>>,
        ) -> i32 {
            if mem.contains_key(&root) {
                return *mem.get(&root).unwrap();
            }
            let sum = ((factors
                .get(&root)
                .unwrap_or(&Vec::new())
                .iter()
                .map(|(left, right)| {
                    if left == right {
                        (count_trees(*right, mem, nums, factors) as usize
                            * count_trees(*right, mem, nums, factors) as usize)
                            % MODULO
                    } else {
                        (2 * count_trees(*left, mem, nums, factors) as usize
                            * count_trees(*right, mem, nums, factors) as usize)
                            % MODULO
                    }
                })
                .fold(0, |acc, count| (acc + count) % MODULO)
                + 1)
                % MODULO) as i32;
            mem.insert(root, sum);
            sum
        }

        arr.iter()
            .map(|&num| count_trees(num, &mut mem, &nums, &factors))
            .reduce(|acc, num| (acc + num) % MODULO as i32)
            .unwrap()
    }
}
fn main() {
    let test_cases = vec![vec![2, 4, 5, 10]];
    for arr in test_cases {
        println!("{}", Solution::num_factored_binary_trees(arr));
    }
}

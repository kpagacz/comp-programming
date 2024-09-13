// https://leetcode.com/problems/xor-queries-of-a-subarray/description/
pub struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let prefix_xor: Vec<_> = std::iter::once(&0)
            .chain(arr.iter())
            .scan(0, |acc, num| {
                *acc ^= num;
                Some(*acc)
            })
            .collect();

        // The prefix starts with 0 and continues,
        // so prefix[0] = 0, and prefix[1] has prefix XOR
        // of just one number from arr

        queries
            .into_iter()
            .map(|query| {
                let (from, to) = (query[0] as usize, query[1] as usize);
                prefix_xor[to + 1] ^ prefix_xor[from]
            })
            .collect()
    }
}

fn main() {
    let test_cases = [(
        vec![1, 3, 4, 8],
        vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]],
    )];

    for (arr, queries) in test_cases {
        println!("{:?}", Solution::xor_queries(arr, queries));
    }
}
